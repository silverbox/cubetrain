<template>
  <div>
    <div :id="id"></div>
    <div class="wasm-screen_floatingpnl" @mousemove="onMouseMove" ref="floatingPanel">
      <div class="wasm-screen_floating_btnset" :style="btnsetPosStyle">
        <v-icon v-if="isBtnActive('y')" class="wasm-screen_arrow_u wasm-screen_floatbtn" @click="onFloatingButton('y', 'p')">mdi-arrow-up-bold</v-icon>
        <v-icon v-if="isBtnActive('y')" class="wasm-screen_arrow_d wasm-screen_floatbtn" @click="onFloatingButton('y', 'n')">mdi-arrow-up-bold</v-icon>
        <v-icon v-if="isBtnActive('z')" class="wasm-screen_arrow_ru wasm-screen_floatbtn" @click="onFloatingButton('z', 'n')">mdi-arrow-up-bold</v-icon>
        <v-icon v-if="isBtnActive('x')" class="wasm-screen_arrow_rd wasm-screen_floatbtn" @click="onFloatingButton('x', 'p')">mdi-arrow-up-bold</v-icon>
        <v-icon v-if="isBtnActive('x')" class="wasm-screen_arrow_lu wasm-screen_floatbtn" @click="onFloatingButton('x', 'n')">mdi-arrow-up-bold</v-icon>
        <v-icon v-if="isBtnActive('z')" class="wasm-screen_arrow_ld wasm-screen_floatbtn" @click="onFloatingButton('z', 'p')">mdi-arrow-up-bold</v-icon>
      </div>
      <!-- <v-btn block @click="onButtonOperation('z', 'neg', 'n')" class="wasm-screen_floatbtn">
        B'
      </v-btn> -->
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, toRefs, onMounted, ref } from 'vue';
import { RotateInfo, Axis, Layer, Dir } from '@/class/cubeutils';
import init, { start, on_animation } from '@/wasm/package.js';

interface SurfaceInfo{
  cx: number, cy: number,
  minx: number, maxx: number,
  miny: number, maxy:number,
  surAxis: Axis, surLayer: Layer,
  xlayer: Layer, ylayer: Layer, zlayer: Layer,
}

// TODO get it from rust
const SURFACE_INFO_LIST_ORG: SurfaceInfo[] = [
  {cx: 135, cy: 165, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "pos", xlayer: "neg", ylayer: "pos", zlayer: "pos"},
  {cx: 170, cy: 145, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "pos", xlayer: "neg", ylayer: "pos", zlayer: "neu"},
  {cx: 205, cy: 125, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "pos", xlayer: "neg", ylayer: "pos", zlayer: "neg"},
  {cx: 170, cy: 185, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "pos", xlayer: "neu", ylayer: "pos", zlayer: "pos"},
  {cx: 205, cy: 165, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "pos", xlayer: "neu", ylayer: "pos", zlayer: "neu"},
  {cx: 240, cy: 145, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "pos", xlayer: "neu", ylayer: "pos", zlayer: "neg"},
  {cx: 205, cy: 205, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "pos", xlayer: "pos", ylayer: "pos", zlayer: "pos"},
  {cx: 240, cy: 185, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "pos", xlayer: "pos", ylayer: "pos", zlayer: "neu"},
  {cx: 275, cy: 165, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "pos", xlayer: "pos", ylayer: "pos", zlayer: "neg"},
  //
  {cx: 115, cy: 195, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "pos", xlayer: "neg", ylayer: "pos", zlayer: "pos"},
  {cx: 115, cy: 235, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "pos", xlayer: "neg", ylayer: "neu", zlayer: "pos"},
  {cx: 115, cy: 275, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "pos", xlayer: "neg", ylayer: "neg", zlayer: "pos"},
  {cx: 150, cy: 215, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "pos", xlayer: "neu", ylayer: "pos", zlayer: "pos"},
  {cx: 150, cy: 255, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "pos", xlayer: "neu", ylayer: "neu", zlayer: "pos"},
  {cx: 150, cy: 295, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "pos", xlayer: "neu", ylayer: "neg", zlayer: "pos"},
  {cx: 185, cy: 235, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "pos", xlayer: "pos", ylayer: "pos", zlayer: "pos"},
  {cx: 185, cy: 275, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "pos", xlayer: "pos", ylayer: "neu", zlayer: "pos"},
  {cx: 185, cy: 315, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "pos", xlayer: "pos", ylayer: "neg", zlayer: "pos"},
  //
  {cx: 225, cy: 235, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "pos", xlayer: "pos", ylayer: "pos", zlayer: "pos"},
  {cx: 260, cy: 215, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "pos", xlayer: "pos", ylayer: "pos", zlayer: "neu"},
  {cx: 295, cy: 195, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "pos", xlayer: "pos", ylayer: "pos", zlayer: "neg"},
  {cx: 225, cy: 275, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "pos", xlayer: "pos", ylayer: "neu", zlayer: "pos"},
  {cx: 260, cy: 255, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "pos", xlayer: "pos", ylayer: "neu", zlayer: "neu"},
  {cx: 295, cy: 235, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "pos", xlayer: "pos", ylayer: "neu", zlayer: "neg"},
  {cx: 225, cy: 315, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "pos", xlayer: "pos", ylayer: "neg", zlayer: "pos"},
  {cx: 260, cy: 295, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "pos", xlayer: "pos", ylayer: "neg", zlayer: "neu"},
  {cx: 295, cy: 275, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "pos", xlayer: "pos", ylayer: "neg", zlayer: "neg"},
  //
];
// let sortedArray: {cx: number, cy: number, minx: number, maxx: number, miny: number, maxy:number, sortval: number;}[] = [];
const AREA_OFFSET = 32;

export default defineComponent({
  name: "WasmScreen",
  setup(props: any, context: any){
    const { id } = toRefs(props)
    const interfaceSetConfig = ref<any>(() => {});
    const interfaceRotate = ref<any>(() => {});
    const infoIdx = ref<number>(-1);
    const btnsetPosStyle = ref<object>({display: 'none'});
    const floatingPanel = ref(null);

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
    // const getPosStyle = (key: string) => {
    //   console.log(key)
    //   const left = 205;
    //   const top = 153;
    //   return {
    //     left: left + 'px',
    //     top: top + 'px'
    //   }
    // };
    const onButtonOperation = (axis: Axis, layer: Layer, dir: Dir) => {
      rotateAction({ axis: axis, layer: layer, dir: dir });
    };
    const isBtnActive = (axis: Axis) => {
      const wkInfo = SURFACE_INFO_LIST_ORG[infoIdx.value];
      return (!!wkInfo && wkInfo.surAxis != axis);
    };
    const onFloatingButton = (btnAxis: Axis, btnDir: Dir) => {
      // console.log(`axis=${btnAxis}, dir=${btnDir}`);
      const wkInfo = SURFACE_INFO_LIST_ORG[infoIdx.value];
      if (btnAxis == wkInfo.surAxis) return;
      //
      let rAxis: Axis = "x";
      if (btnAxis == "x") {
        if (wkInfo.surAxis == "y") {rAxis = "z"} else {rAxis = "y"}
      } else if (btnAxis == "y") {
        if (wkInfo.surAxis == "x") {rAxis = "z"} else {rAxis = "x"}
      } else if (btnAxis == "z") {
        if (wkInfo.surAxis == "y") {rAxis = "x"} else {rAxis = "y"}
      }
      //
      let rLayer: Layer = "pos";
      if (rAxis == "x") {
        rLayer = wkInfo.xlayer;
      } else if (rAxis == "y") {
        rLayer = wkInfo.ylayer;
      } else if (rAxis == "z") {
        rLayer = wkInfo.zlayer;
      }
      //
      let rDir: Dir = btnDir;
      if (wkInfo.surAxis == "y" && rAxis == "z") {
        rDir = btnDir == "p" ? "n" : "p";
      } else if (wkInfo.surAxis == "z" && rAxis == "x") {
        rDir = btnDir == "p" ? "n" : "p";
      } else if (wkInfo.surAxis == "x" && rAxis == "y") {
        rDir = btnDir == "p" ? "n" : "p";
      }
      //
      // console.log(`rAxis=${rAxis}, rDir=${rDir}`);
      rotateAction({ axis: rAxis, layer: rLayer, dir: rDir });
    };
    const onMouseMove = (e: MouseEvent) => {
      if (floatingPanel.value != e.target) {
        return;
      }
      const currentIdx = getCurrentPosIdx(e.offsetX, e.offsetY);
      console.log(`x=${e.offsetX}, Y=${e.offsetY}, currentIdx=${currentIdx}`);
      if (infoIdx.value != currentIdx) {
        if (currentIdx >= 0) {
          const wkInfo = SURFACE_INFO_LIST_ORG[currentIdx];
          btnsetPosStyle.value =  {
            display: 'block',
            left: wkInfo.cx + 'px',
            top: wkInfo.cy + 'px'
          }
        } else {
          btnsetPosStyle.value = {display: 'none'}
        }
        infoIdx.value = currentIdx;
      }
    };
    const onWasmAnimation = () => {
      return on_animation();
    };
    const initCalcPosArea = () => {
      for (let wk_surface_inf of SURFACE_INFO_LIST_ORG) {
        wk_surface_inf.minx = wk_surface_inf.cx - AREA_OFFSET;
        wk_surface_inf.maxx = wk_surface_inf.cx + AREA_OFFSET;
        wk_surface_inf.miny = wk_surface_inf.cy - AREA_OFFSET;
        wk_surface_inf.maxy = wk_surface_inf.cy + AREA_OFFSET;
      }
      // sortedArray = SURFACE_INFO_LIST_ORG.sort((n1,n2) => {
      //   if (n1.sortval > n2.sortval) return 1;
      //   if (n1.sortval < n2.sortval) return -1;
      //   return 0;
      // });
    };
    const getCurrentPosIdx = (x: number, y:number): number => {
      const isInInfo = (info: SurfaceInfo): boolean => {
        return (info.minx <= x) && (x <= info.maxx) && (info.miny <= y) && (y <= info.maxy);
      };
      if (infoIdx.value >= 0 && isInInfo(SURFACE_INFO_LIST_ORG[infoIdx.value])) {
        return infoIdx.value;
      }

      let retIdx = -1;
      SURFACE_INFO_LIST_ORG.forEach((info, index) => {
        if (retIdx >= 0) return;
        if (isInInfo(info)) {
          retIdx = index;
          return;
        }
      });
      return retIdx;
    };
    const onMountedOperation = () => {
      init('/wasm/package_bg.wasm').then(() => {
        const [set_config, rotate] = start(id.value);
        interfaceSetConfig.value = set_config;
        interfaceRotate.value = rotate;
      });
      initCalcPosArea();
      // console.log(sortedArray)
    }
    onMounted(onMountedOperation);
    return {
      setConfig,
      rotate,
      rotateAction,
      btnsetPosStyle,
      isBtnActive,
      //
      onMouseMove,
      onButtonOperation,
      onFloatingButton,
      onWasmAnimation,
      //
      floatingPanel
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
  width: 400px;
  height: 400px;
}
.wasm-screen_floatbtn, .wasm-screen_floating_btnset{
  position: absolute;
}
.wasm-screen_arrow_u{
  transform: rotate(0deg);
  top: -36px;
  left: -12px;
}
.wasm-screen_arrow_d{
  transform: rotate(180deg);
  top: 12px;
  left: -12px;
}
.wasm-screen_arrow_ru{
  transform: rotate(60deg);
  top: -24px;
  left: 6px;
}
.wasm-screen_arrow_rd{
  transform: rotate(120deg);
  top: 0px;
  left: 6px;
}
.wasm-screen_arrow_lu{
  transform: rotate(-60deg);
  top: -24px;
  left: -30px;
}
.wasm-screen_arrow_ld{
  transform: rotate(-120deg);
  top: 0px;
  left: -30px;
}

</style>