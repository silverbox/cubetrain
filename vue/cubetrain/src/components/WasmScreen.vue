<template>
  <div>
    <div :id="id" :class="{'backview-hide': !isBackViewVisible}"></div>
    <div class="wasm-screen_floatingpnl" @mousemove="onMouseMove" ref="floatingPanel">
      <div v-if="isAxixBtnActive('x', true)" class="wasm-screen_xaxis_btnset">
        <v-icon class="wasm-screen_xaxis_p wasm-screen_floatbtn" @click="onAxisButton('x', 'p')">mdi-rotate-left</v-icon>
        <v-icon class="wasm-screen_xaxis_n wasm-screen_floatbtn" @click="onAxisButton('x', 'n')">mdi-rotate-right</v-icon>
      </div>
      <div v-if="isAxixBtnActive('y', true)" class="wasm-screen_yaxis_btnset">
        <v-icon class="wasm-screen_yaxis_p wasm-screen_floatbtn" @click="onAxisButton('y', 'p')">mdi-rotate-left</v-icon>
        <v-icon class="wasm-screen_yaxis_n wasm-screen_floatbtn" @click="onAxisButton('y', 'n')">mdi-rotate-right</v-icon>
      </div>
      <div v-if="isAxixBtnActive('z', true)" class="wasm-screen_zaxis_btnset">
        <v-icon class="wasm-screen_zaxis_p wasm-screen_floatbtn" @click="onAxisButton('z', 'p')">mdi-rotate-left</v-icon>
        <v-icon class="wasm-screen_zaxis_n wasm-screen_floatbtn" @click="onAxisButton('z', 'n')">mdi-rotate-right</v-icon>
      </div>
      <div v-if="isAxixBtnActive('z', false)" class="wasm-screen_zraxis_btnset">
        <v-icon class="wasm-screen_xaxis_p wasm-screen_floatbtn" @click="onAxisButton('z', 'n')">mdi-rotate-left</v-icon>
        <v-icon class="wasm-screen_xaxis_n wasm-screen_floatbtn" @click="onAxisButton('z', 'p')">mdi-rotate-right</v-icon>
      </div>
      <div v-if="isAxixBtnActive('y', false)" class="wasm-screen_yraxis_btnset">
        <v-icon class="wasm-screen_yaxis_p wasm-screen_floatbtn" @click="onAxisButton('y', 'n')">mdi-rotate-left</v-icon>
        <v-icon class="wasm-screen_yaxis_n wasm-screen_floatbtn" @click="onAxisButton('y', 'p')">mdi-rotate-right</v-icon>
      </div>
      <div v-if="isAxixBtnActive('x', false)" class="wasm-screen_xraxis_btnset">
        <v-icon class="wasm-screen_zaxis_p wasm-screen_floatbtn" @click="onAxisButton('x', 'n')">mdi-rotate-left</v-icon>
        <v-icon class="wasm-screen_zaxis_n wasm-screen_floatbtn" @click="onAxisButton('x', 'p')">mdi-rotate-right</v-icon>
      </div>
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

interface AxisAreaInfo{
  cx: number, cy: number, axis: Axis,
  minx: number, maxx: number,
  miny: number, maxy:number,
}
const AXISAREA_INFO_LIST: AxisAreaInfo[] = [
  {cx: 335, cy: 300, minx: 0, maxx: 0, miny: 0, maxy: 0, axis: "x"},
  {cx: 205, cy:  80, minx: 0, maxx: 0, miny: 0, maxy: 0, axis: "y"},
  {cx:  75, cy: 300, minx: 0, maxx: 0, miny: 0, maxy: 0, axis: "z"},
  {cx: 745, cy: 300, minx: 0, maxx: 0, miny: 0, maxy: 0, axis: "z"},
  {cx: 615, cy:  80, minx: 0, maxx: 0, miny: 0, maxy: 0, axis: "y"},
  {cx: 485, cy: 300, minx: 0, maxx: 0, miny: 0, maxy: 0, axis: "x"},
]

interface SurfaceInfo{
  cx: number, cy: number,
  minx: number, maxx: number,
  miny: number, maxy:number,
  surAxis: Axis, surLayer: Layer,
  xlayer: Layer, ylayer: Layer, zlayer: Layer,
}

// TODO get it from rust
const SURFACE_INFO_LIST: SurfaceInfo[] = [
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
  //
  {cx: 545, cy: 285, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "neg", ylayer: "neg", xlayer: "pos"},
  {cx: 580, cy: 265, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "neg", ylayer: "neg", xlayer: "neu"},
  {cx: 615, cy: 245, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "neg", ylayer: "neg", xlayer: "neg"},
  {cx: 580, cy: 305, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "neu", ylayer: "neg", xlayer: "pos"},
  {cx: 615, cy: 285, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "neu", ylayer: "neg", xlayer: "neu"},
  {cx: 650, cy: 265, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "neu", ylayer: "neg", xlayer: "neg"},
  {cx: 615, cy: 325, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "pos", ylayer: "neg", xlayer: "pos"},
  {cx: 650, cy: 305, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "pos", ylayer: "neg", xlayer: "neu"},
  {cx: 685, cy: 285, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "pos", ylayer: "neg", xlayer: "neg"},
  //
  {cx: 530, cy: 175, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "pos", xlayer: "pos"},
  {cx: 565, cy: 155, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "pos", xlayer: "neu"},
  {cx: 600, cy: 135, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "pos", xlayer: "neg"},
  {cx: 530, cy: 215, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "neu", xlayer: "pos"},
  {cx: 565, cy: 195, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "neu", xlayer: "neu"},
  {cx: 600, cy: 175, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "neu", xlayer: "neg"},
  {cx: 530, cy: 255, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "neg", xlayer: "pos"},
  {cx: 565, cy: 235, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "neg", xlayer: "neu"},
  {cx: 600, cy: 215, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "neg", xlayer: "neg"},
  //
  {cx: 635, cy: 135, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "neg", ylayer: "pos", xlayer: "neg"},
  {cx: 635, cy: 175, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "neg", ylayer: "neu", xlayer: "neg"},
  {cx: 635, cy: 215, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "neg", ylayer: "neg", xlayer: "neg"},
  {cx: 670, cy: 155, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "neu", ylayer: "pos", xlayer: "neg"},
  {cx: 670, cy: 195, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "neu", ylayer: "neu", xlayer: "neg"},
  {cx: 670, cy: 235, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "neu", ylayer: "neg", xlayer: "neg"},
  {cx: 705, cy: 175, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "pos", ylayer: "pos", xlayer: "neg"},
  {cx: 705, cy: 215, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "pos", ylayer: "neu", xlayer: "neg"},
  {cx: 705, cy: 255, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "pos", ylayer: "neg", xlayer: "neg"},

  //
];
// let sortedArray: {cx: number, cy: number, minx: number, maxx: number, miny: number, maxy:number, sortval: number;}[] = [];
const AREA_OFFSET = 28;

export default defineComponent({
  name: "WasmScreen",
  setup(props: any, context: any){
    const { id } = toRefs(props)
    const interfaceSetConfig = ref<any>(() => {});
    const interfaceRotate = ref<any>(() => {});
    const infoIdx = ref<number>(-1);
    const axisInfoIdx = ref<number>(-1);
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
    const isAxixBtnActive = (axis: Axis, isFront: boolean) => {
      const wkInfo = AXISAREA_INFO_LIST[axisInfoIdx.value];
      return (!!wkInfo && wkInfo.axis == axis && isBackAxisIdx() != isFront);
    };
    const isBtnActive = (btnAxis: Axis) => {
      const wkInfo = SURFACE_INFO_LIST[infoIdx.value];
      if (!wkInfo) return false;
      const wkBtnAxis = convertBtnAxisToAxis(btnAxis);
      return (!!wkInfo && wkInfo.surAxis != wkBtnAxis);
    };
    const convertBtnAxisToAxis = (btnAxis: Axis) => {
      if (isBackIdx()) {
        if (btnAxis == 'x') return 'z'
        else if (btnAxis == 'z') return 'x';
      }
      return btnAxis;
    }
    const isBackAxisIdx = (): boolean => {
      return axisInfoIdx.value >= 3;
    }
    const isBackIdx = (): boolean => {
      return infoIdx.value >= 27;
    }
    const onAxisButton = (axis: Axis, dir: Dir) => {
      rotateAction({ axis: axis, layer: "all", dir: dir });
    };
    const onFloatingButton = (btnAxis: Axis, btnDir: Dir) => {
      const wkInfo = SURFACE_INFO_LIST[infoIdx.value];
      if (!wkInfo) return false;
      const wkBtnAxis = convertBtnAxisToAxis(btnAxis);
      // console.log(`btnAxis=${btnAxis}, wkBtnAxis=${wkBtnAxis}, dir=${btnDir}`);
      if (wkBtnAxis == wkInfo.surAxis) return;
      //
      let rAxis: Axis = "x";
      if (isBackIdx()) {
        if (wkBtnAxis == "y") {
          if (wkInfo.surAxis == "x") {rAxis = "z"} else {rAxis = "x"}
        } else if (wkBtnAxis == "x") {
          if (wkInfo.surAxis == "z") {rAxis = "y"} else {rAxis = "z"}
        }else if (wkBtnAxis == "z") {
          if (wkInfo.surAxis == "y") {rAxis = "x"} else {rAxis = "y"}
        }
      } else {
        if (wkBtnAxis == "x") {
          if (wkInfo.surAxis == "y") {rAxis = "z"} else {rAxis = "y"}
        } else if (wkBtnAxis == "y") {
          if (wkInfo.surAxis == "x") {rAxis = "z"} else {rAxis = "x"}
        } else if (wkBtnAxis == "z") {
          if (wkInfo.surAxis == "y") {rAxis = "x"} else {rAxis = "y"}
        }
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
      if (isBackIdx()) {
        if (wkInfo.surAxis == "y" && rAxis == "x") {
          rDir = btnDir == "p" ? "n" : "p";
        } else if (wkInfo.surAxis == "z" && rAxis == "y") {
          rDir = btnDir == "p" ? "n" : "p";
        } else if (wkInfo.surAxis == "x" && rAxis == "z") {
          rDir = btnDir == "p" ? "n" : "p";
        }
      } else {
        if (wkInfo.surAxis == "y" && rAxis == "z") {
          rDir = btnDir == "p" ? "n" : "p";
        } else if (wkInfo.surAxis == "z" && rAxis == "x") {
          rDir = btnDir == "p" ? "n" : "p";
        } else if (wkInfo.surAxis == "x" && rAxis == "y") {
          rDir = btnDir == "p" ? "n" : "p";
        }
      }
      //
      // console.log(`rAxis=${rAxis}, rDir=${rDir}`);
      rotateAction({ axis: rAxis, layer: rLayer, dir: rDir });
    };
    const onMouseMove = (e: MouseEvent) => {
      if (floatingPanel.value != e.target) {
        return;
      }
      // console.log(`x=${e.offsetX}, Y=${e.offsetY}`);

      const currentAxisIdx = getCurrentAxisPosIdx(e.offsetX, e.offsetY);
      if (axisInfoIdx.value != currentAxisIdx) {
        axisInfoIdx.value = currentAxisIdx;
      }

      const currentIdx = (currentAxisIdx >= 0) ? -1 : getCurrentPosIdx(e.offsetX, e.offsetY);
      if (infoIdx.value != currentIdx) {
        if (currentIdx >= 0) {
          const wkInfo = SURFACE_INFO_LIST[currentIdx];
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
    // TODO calc it at rust module
    const initCalcPosArea = () => {
      for (let wk_axis_inf of AXISAREA_INFO_LIST) {
        wk_axis_inf.minx = wk_axis_inf.cx - AREA_OFFSET;
        wk_axis_inf.maxx = wk_axis_inf.cx + AREA_OFFSET;
        wk_axis_inf.miny = wk_axis_inf.cy - AREA_OFFSET;
        wk_axis_inf.maxy = wk_axis_inf.cy + AREA_OFFSET;
      }
      for (let wk_surface_inf of SURFACE_INFO_LIST) {
        wk_surface_inf.minx = wk_surface_inf.cx - AREA_OFFSET;
        wk_surface_inf.maxx = wk_surface_inf.cx + AREA_OFFSET;
        wk_surface_inf.miny = wk_surface_inf.cy - AREA_OFFSET;
        wk_surface_inf.maxy = wk_surface_inf.cy + AREA_OFFSET;
      }
      // sortedArray = SURFACE_INFO_LIST.sort((n1,n2) => {
      //   if (n1.sortval > n2.sortval) return 1;
      //   if (n1.sortval < n2.sortval) return -1;
      //   return 0;
      // });
    };
    const getCurrentAxisPosIdx = (x: number, y:number): number => {
      const isInAxisInfo = (info: AxisAreaInfo): boolean => {
        return (info.minx <= x) && (x <= info.maxx) && (info.miny <= y) && (y <= info.maxy);
      };
      if (axisInfoIdx.value >= 0 && isInAxisInfo(AXISAREA_INFO_LIST[axisInfoIdx.value])) {
        return axisInfoIdx.value;
      }

      let retIdx = -1;
      AXISAREA_INFO_LIST.forEach((info, index) => {
        if (retIdx >= 0) return;
        if (isInAxisInfo(info)) {
          retIdx = index;
          return;
        }
      });
      return retIdx;
    };
    const getCurrentPosIdx = (x: number, y:number): number => {
      const isInInfo = (info: SurfaceInfo): boolean => {
        return (info.minx <= x) && (x <= info.maxx) && (info.miny <= y) && (y <= info.maxy);
      };

      if (infoIdx.value >= 0 && isInInfo(SURFACE_INFO_LIST[infoIdx.value])) {
        return infoIdx.value;
      }

      let retIdx = -1;
      SURFACE_INFO_LIST.forEach((info, index) => {
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
      isAxixBtnActive,
      isBtnActive,
      //
      onMouseMove,
      onButtonOperation,
      onFloatingButton,
      onAxisButton,
      //
      onWasmAnimation,
      //
      floatingPanel
    }
  },
  props: {
    id: {type: String, required: true},
    isBackViewVisible: {type: Boolean, required: true},
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
  width: 900px;
  height: 400px;
}
.wasm-screen_floatbtn, .wasm-screen_floating_btnset{
  position: absolute;
  color: gray;
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

.wasm-screen_xaxis_btnset, .wasm-screen_zraxis_btnset{
  position: absolute;
  top: 280px;
}
.wasm-screen_xaxis_btnset{
  left: 315px;
}
.wasm-screen_zraxis_btnset{
  left: 725px;
}
.wasm-screen_xaxis_p, .wasm-screen_xaxis_n{
  transform: scale(1.7, 1.7) rotateY(60deg);
}
.wasm-screen_xaxis_n {
  top: 12px;
  left: 16px;
}

.wasm-screen_zaxis_btnset, .wasm-screen_xraxis_btnset {
  position: absolute;
  top: 295px;
}
.wasm-screen_zaxis_btnset {
  left: 50px;
}
.wasm-screen_xraxis_btnset {
  left: 460px;
}
.wasm-screen_zaxis_p, .wasm-screen_zaxis_n {
  transform: scale(1.7, 1.7) rotateY(60deg);
}
.wasm-screen_zaxis_n {
  top: -10px;
  left: 18px;
}

.wasm-screen_yaxis_btnset, .wasm-screen_yraxis_btnset {
  position: absolute;
  top: 55px;
}
.wasm-screen_yaxis_btnset {
  left: 206px;
}
.wasm-screen_yraxis_btnset {
  left: 616px;
}
.wasm-screen_yaxis_p, .wasm-screen_yaxis_n{
  transform: scale(1.5, 1);
  left: -12px;
}
.wasm-screen_yaxis_n{
  left: -12px;
  top: 20px;
}
.backview-hide .wasm-back-view{
  display: none;
}
</style>