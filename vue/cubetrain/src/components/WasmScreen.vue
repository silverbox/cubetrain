<template>
  <div>
    <div :id="id"></div>
    <div class="wasm-screen_floatingpnl">
      <v-btn block @click="onButtonOperation('z', 'neg', 'n')" class="wasm-screen_floatbtn" :style="getPosStyle('b-r')">
        B'
      </v-btn>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, toRefs, onMounted, ref } from 'vue';
import { RotateInfo, Axis, Layer, Dir } from '@/class/cubeutils';
import init, { start, on_animation } from '@/wasm/package.js';

export default defineComponent({
  name: "WasmScreen",
  setup(props: any, context: any){
    const { id } = toRefs(props)
    const interfaceSetConfig = ref<any>(() => {});
    const interfaceRotate = ref<any>(() => {});
    const setConfig = (type: string, val: number) => {
      const unitedstr = `${type} ${val}`;
      interfaceSetConfig.value(unitedstr);
    };
    const rotate = (axis: string, layer: string, dir: string) => {
      const unitedstr = `${axis} ${layer} ${dir}`;
      interfaceRotate.value(unitedstr);
    };
    const rotateAction = (rotateInfo: RotateInfo) => {
      const { axis, layer, dir } = rotateInfo;
      context.emit("rotateAction", axis, layer, dir);
    };
    const getPosStyle = (key: string) => {
      console.log(key)
      const left = 100;
      const top = 50;
      return {
        left: left + 'px',
        top: top + 'px'
      }
    };
    const onButtonOperation = (axis: Axis, layer: Layer, dir: Dir) => {
      rotateAction({ axis: axis, layer: layer, dir: dir });
    };
    const onWasmAnimation = () => {
      return on_animation();
    };
    const onMountedOperation = () => {
      init('/wasm/package_bg.wasm').then(() => {
        const [set_config, rotate] = start(id.value);
        interfaceSetConfig.value = set_config;
        interfaceRotate.value = rotate;
      });
    }
    onMounted(onMountedOperation);
    return {
      setConfig,
      rotate,
      rotateAction,
      getPosStyle,
      onButtonOperation,
      onWasmAnimation
    }
  },
  props: {
    id: {type: String, required: true}
  },
})
</script>
<style>
.wasm-container {
  position: absolute;
  display: flex;
}
.wasm-sub-container {
  position: relative;
}
.wasm-canvas {
  margin: 5px;
}
.wasm-label {
  position: absolute;
  top: 10px;
  left: 15px;
  font-size: 24px;
}
.wasm-screen_floatingpnl{
  position: absolute;
}
.wasm-screen_floatbtn{
  position: absolute;
}
</style>