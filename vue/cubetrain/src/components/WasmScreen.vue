<template>
  <div :id="id"></div>
</template>
<script lang="ts">
import { defineComponent, toRefs, onMounted, ref } from 'vue';
import init from '@/wasm/package.js';
import { start } from '@/wasm/package.js';

export default defineComponent({
  name: "WasmScreen",
  setup(props){
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
      rotate
    }
  },
  props: {
    id: {type: String, required: true}
  },
})
</script>