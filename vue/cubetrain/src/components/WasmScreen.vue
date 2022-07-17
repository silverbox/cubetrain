<template>
  <div>
    <div :id="id" :class="getPanelClass"></div>
    <div class="wasm-screen_floatingpnl" :style="getFloatingPnlStyle" @mousemove="onMouseMove" @touchend="onTouchEnd" ref="floatingPanel">
      <v-btn icon @click="toggleAltView" class="wasm-screen_altviewbtn">
        <v-icon>mdi-format-rotate-90</v-icon>
      </v-btn>
      <div :style="getAxixBtnStyle('x', true)" class="wasm-screen_xaxis_btnset">
        <v-icon class="wasm-screen_xaxis_p wasm-screen_floatbtn" @click="onAxisButton('x', 'p')">mdi-rotate-left</v-icon>
        <v-icon class="wasm-screen_xaxis_n wasm-screen_floatbtn" @click="onAxisButton('x', 'n')">mdi-rotate-right</v-icon>
      </div>
      <div :style="getAxixBtnStyle('y', true)" class="wasm-screen_yaxis_btnset">
        <v-icon class="wasm-screen_yaxis_p wasm-screen_floatbtn" @click="onAxisButton('y', 'p')">mdi-rotate-left</v-icon>
        <v-icon class="wasm-screen_yaxis_n wasm-screen_floatbtn" @click="onAxisButton('y', 'n')">mdi-rotate-right</v-icon>
      </div>
      <div :style="getAxixBtnStyle('z', true)" class="wasm-screen_zaxis_btnset">
        <v-icon class="wasm-screen_zaxis_p wasm-screen_floatbtn" @click="onAxisButton('z', 'p')">mdi-rotate-left</v-icon>
        <v-icon class="wasm-screen_zaxis_n wasm-screen_floatbtn" @click="onAxisButton('z', 'n')">mdi-rotate-right</v-icon>
      </div>
      <div :style="getAxixBtnStyle('z', false)" class="wasm-screen_zraxis_btnset">
        <v-icon class="wasm-screen_xaxis_p wasm-screen_floatbtn" @click="onAxisButton('z', 'n')">mdi-rotate-left</v-icon>
        <v-icon class="wasm-screen_xaxis_n wasm-screen_floatbtn" @click="onAxisButton('z', 'p')">mdi-rotate-right</v-icon>
      </div>
      <div :style="getAxixBtnStyle('y', false)" class="wasm-screen_yraxis_btnset">
        <v-icon class="wasm-screen_yaxis_p wasm-screen_floatbtn" @click="onAxisButton('y', 'n')">mdi-rotate-left</v-icon>
        <v-icon class="wasm-screen_yaxis_n wasm-screen_floatbtn" @click="onAxisButton('y', 'p')">mdi-rotate-right</v-icon>
      </div>
      <div :style="getAxixBtnStyle('x', false)" class="wasm-screen_xraxis_btnset">
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
import { defineComponent, toRefs, onMounted, ref, computed } from 'vue';
import init, { start, on_animation } from '@/wasm/package.js';
import { RotateInfo, Axis, Layer, Dir } from '@/class/cubeutils';
// import { CubeViewType } from '@/class/types';

interface AxisAreaInfo{
  cx: number, cy: number, axis: Axis,
  minx: number, maxx: number,
  miny: number, maxy:number,
}

// TODO get it from rust
const AXISAREA_INFO_LIST_FR: AxisAreaInfo[] = [
  {cx: 335, cy: 300, minx: 0, maxx: 0, miny: 0, maxy: 0, axis: "x"},
  {cx: 205, cy:  80, minx: 0, maxx: 0, miny: 0, maxy: 0, axis: "y"},
  {cx:  75, cy: 300, minx: 0, maxx: 0, miny: 0, maxy: 0, axis: "z"},
]
const AXISAREA_INFO_LIST_BK: AxisAreaInfo[] = [
  {cx: 335, cy: 300, minx: 0, maxx: 0, miny: 0, maxy: 0, axis: "z"},
  {cx: 205, cy:  80, minx: 0, maxx: 0, miny: 0, maxy: 0, axis: "y"},
  {cx:  75, cy: 300, minx: 0, maxx: 0, miny: 0, maxy: 0, axis: "x"},
]
const AXISAREA_INFO_LIST_SET = [AXISAREA_INFO_LIST_FR, AXISAREA_INFO_LIST_BK]

interface SurfaceInfo{
  cx: number, cy: number,
  minx: number, maxx: number,
  miny: number, maxy:number,
  surAxis: Axis, surLayer: Layer,
  xlayer: Layer, ylayer: Layer, zlayer: Layer,
}

// TODO get it from rust
const SURFACE_INFO_LIST_FR: SurfaceInfo[] = [
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
];
  //
const SURFACE_INFO_LIST_BK: SurfaceInfo[] = [
  {cx: 135, cy: 285, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "neg", ylayer: "neg", xlayer: "pos"},
  {cx: 170, cy: 265, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "neg", ylayer: "neg", xlayer: "neu"},
  {cx: 205, cy: 245, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "neg", ylayer: "neg", xlayer: "neg"},
  {cx: 175, cy: 305, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "neu", ylayer: "neg", xlayer: "pos"},
  {cx: 205, cy: 285, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "neu", ylayer: "neg", xlayer: "neu"},
  {cx: 240, cy: 265, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "neu", ylayer: "neg", xlayer: "neg"},
  {cx: 205, cy: 325, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "pos", ylayer: "neg", xlayer: "pos"},
  {cx: 240, cy: 305, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "pos", ylayer: "neg", xlayer: "neu"},
  {cx: 275, cy: 285, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "y", surLayer: "neg", zlayer: "pos", ylayer: "neg", xlayer: "neg"},
  //
  {cx: 115, cy: 175, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "pos", xlayer: "pos"},
  {cx: 150, cy: 155, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "pos", xlayer: "neu"},
  {cx: 185, cy: 135, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "pos", xlayer: "neg"},
  {cx: 115, cy: 215, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "neu", xlayer: "pos"},
  {cx: 150, cy: 195, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "neu", xlayer: "neu"},
  {cx: 185, cy: 175, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "neu", xlayer: "neg"},
  {cx: 115, cy: 255, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "neg", xlayer: "pos"},
  {cx: 150, cy: 235, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "neg", xlayer: "neu"},
  {cx: 185, cy: 215, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "z", surLayer: "neg", zlayer: "neg", ylayer: "neg", xlayer: "neg"},
  //
  {cx: 220, cy: 135, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "neg", ylayer: "pos", xlayer: "neg"},
  {cx: 220, cy: 175, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "neg", ylayer: "neu", xlayer: "neg"},
  {cx: 220, cy: 215, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "neg", ylayer: "neg", xlayer: "neg"},
  {cx: 255, cy: 155, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "neu", ylayer: "pos", xlayer: "neg"},
  {cx: 255, cy: 195, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "neu", ylayer: "neu", xlayer: "neg"},
  {cx: 255, cy: 235, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "neu", ylayer: "neg", xlayer: "neg"},
  {cx: 290, cy: 175, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "pos", ylayer: "pos", xlayer: "neg"},
  {cx: 290, cy: 215, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "pos", ylayer: "neu", xlayer: "neg"},
  {cx: 290, cy: 255, minx: 0, maxx: 0, miny: 0, maxy: 0, surAxis: "x", surLayer: "neg", zlayer: "pos", ylayer: "neg", xlayer: "neg"},
  //
];
const SURFACE_INFO_LIST_SET = [SURFACE_INFO_LIST_FR, SURFACE_INFO_LIST_BK]
// let sortedArray: {cx: number, cy: number, minx: number, maxx: number, miny: number, maxy:number, sortval: number;}[] = [];
const AREA_OFFSET = 28;

const X_OFFSET = 415;
const Y_OFFSET = 415;

export default defineComponent({
  name: "WasmScreen",
  setup(props: any, context: any){
    const { id, cubeViewType } = toRefs(props)
    const interfaceSetConfig = ref<any>(() => {});
    const interfaceRotate = ref<any>(() => {});
    const isAltView = ref<boolean>(false);
    const isFrontViewMoving = ref<boolean>(true);
    const infoIdx = ref<number>(-1);
    const axisInfoIdx = ref<number>(-1);
    const btnsetPosStyle = ref<object>({display: 'none'});
    const floatingPanel = ref<any>(undefined);

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
    const getFloatingPnlStyle = computed(() => {
      const width = cubeViewType.value === "horizon" ? X_OFFSET * 2 : X_OFFSET;
      const height = cubeViewType.value === "vertical" ? Y_OFFSET * 2 : Y_OFFSET;
      return {
        width: width + 'px',
        height: height + 'px'
      }
    });
    const getAxixBtnStyle = (axis: Axis, isFront: boolean) => {
      const wkInfo = AXISAREA_INFO_LIST_SET[getInfoSetIndex()][axisInfoIdx.value];
      const isVisible = (!!wkInfo && wkInfo.axis == axis && (getInfoSetIndex() == 0) == isFront);

      const wkOX = getMousePosOffsetX();
      const wkOY = getMousePosOffsetY();
      let top = 55;
      let left = 206;
      switch(axis){
        case "x":
          top = isFront ? 280 : 295;
          left = isFront ? 315 : 45;
          break;
        case "z":
          top = isFront ? 295 : 280;
          left = isFront ? 50 : 310;
          break;
      }
      return {
        display: isVisible ? "block" : "none",
        left: (left + wkOX) + 'px',
        top: (top + wkOY) + 'px'
      }
    };
    const isBtnActive = (btnAxis: Axis) => {
      const wkInfo = SURFACE_INFO_LIST_SET[getInfoSetIndex()][infoIdx.value];
      if (!wkInfo) return false;
      const wkBtnAxis = convertBtnAxisToAxis(btnAxis);
      return (!!wkInfo && wkInfo.surAxis != wkBtnAxis);
    };
    const convertBtnAxisToAxis = (btnAxis: Axis) => {
      if (!isFrontViewMoving.value) {
        if (btnAxis == 'x') return 'z'
        else if (btnAxis == 'z') return 'x';
      }
      return btnAxis;
    }
    const toggleAltView = () => {
      isAltView.value = !isAltView.value;
      onPanelMouseAction(-1, -1);
    };
    const onAxisButton = (axis: Axis, dir: Dir) => {
      rotateAction({ axis: axis, layer: "all", dir: dir });
    };
    const onFloatingButton = (btnAxis: Axis, btnDir: Dir) => {
      const wkInfo = SURFACE_INFO_LIST_SET[getInfoSetIndex()][infoIdx.value];
      if (!wkInfo) return false;
      const wkBtnAxis = convertBtnAxisToAxis(btnAxis);
      // console.log(`btnAxis=${btnAxis}, wkBtnAxis=${wkBtnAxis}, dir=${btnDir}`);
      if (wkBtnAxis == wkInfo.surAxis) return;
      //
      let rAxis: Axis = "x";
      if (isFrontViewMoving.value) {
        if (wkBtnAxis == "x") {
          if (wkInfo.surAxis == "y") {rAxis = "z"} else {rAxis = "y"}
        } else if (wkBtnAxis == "y") {
          if (wkInfo.surAxis == "x") {rAxis = "z"} else {rAxis = "x"}
        } else if (wkBtnAxis == "z") {
          if (wkInfo.surAxis == "y") {rAxis = "x"} else {rAxis = "y"}
        }
      } else {
        if (wkBtnAxis == "y") {
          if (wkInfo.surAxis == "x") {rAxis = "z"} else {rAxis = "x"}
        } else if (wkBtnAxis == "x") {
          if (wkInfo.surAxis == "z") {rAxis = "y"} else {rAxis = "z"}
        }else if (wkBtnAxis == "z") {
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
      if (isFrontViewMoving.value) {
        if (wkInfo.surAxis == "y" && rAxis == "z") {
          rDir = btnDir == "p" ? "n" : "p";
        } else if (wkInfo.surAxis == "z" && rAxis == "x") {
          rDir = btnDir == "p" ? "n" : "p";
        } else if (wkInfo.surAxis == "x" && rAxis == "y") {
          rDir = btnDir == "p" ? "n" : "p";
        }
      } else {
        if (wkInfo.surAxis == "y" && rAxis == "x") {
          rDir = btnDir == "p" ? "n" : "p";
        } else if (wkInfo.surAxis == "z" && rAxis == "y") {
          rDir = btnDir == "p" ? "n" : "p";
        } else if (wkInfo.surAxis == "x" && rAxis == "z") {
          rDir = btnDir == "p" ? "n" : "p";
        }
      }
      //
      // console.log(`rAxis=${rAxis}, rDir=${rDir}`);
      rotateAction({ axis: rAxis, layer: rLayer, dir: rDir });
    };
    const getInfoSetIndex = () => {
      return isFrontViewMoving.value ? 0 : 1;
    };
    const getPanelClass = computed(() => {
      let retClass = "";
      if (cubeViewType.value === "alone") {
        retClass = (isAltView.value) ? "frontview-hide" : "backview-hide" ;
      }
      retClass += (isAltView.value) ? " altview" : " norview";
      retClass += " " + cubeViewType.value;
      // switch(cubeViewType.value){
      //   case "horizon":
      //     isFrontViewMoving.value = e.offsetX < X_OFFSET;
      //     break;
      //   case "vertical":
      //     isFrontViewMoving.value = e.offsetY < Y_OFFSET;
      //     break;
      //   default:
      //     retClass =  (isAltView.value) ? "backview-hide" : "frontview-hide";
      // }
      return retClass;
    });
    const onMouseMove = (e: MouseEvent) => {
      if (floatingPanel.value != e.target) {
        return;
      }
      onPanelMouseAction(e.offsetX, e.offsetY);
    };
    const onTouchEnd = (e: TouchEvent) => {
      if (floatingPanel.value == undefined) {
        return;
      }
      var touchObject = e.changedTouches[0] ;
      var touchX = touchObject.clientX ;
      var touchY = touchObject.clientY ;
      const clientRect = floatingPanel.value.getBoundingClientRect();
      if (!clientRect) {
        return;
      }
      onPanelMouseAction(touchX - clientRect.left, touchY - clientRect.top);
    };
    const onPanelMouseAction = (x: number, y: number) => {
      switch(cubeViewType.value){
        case "horizon":
          isFrontViewMoving.value = x < X_OFFSET;
          break;
        case "vertical":
          isFrontViewMoving.value = y < Y_OFFSET;
          break;
        default:
          isFrontViewMoving.value = true;
      }
      if (isAltView.value) isFrontViewMoving.value = !isFrontViewMoving.value;
      // const wkOX = getMousePosOffsetX();
      // const wkOY = getMousePosOffsetY();

      const currentAxisIdx = getCurrentAxisPosIdx(x, y);
      if (axisInfoIdx.value != currentAxisIdx) {
        axisInfoIdx.value = currentAxisIdx;
      }

      const currentIdx = (currentAxisIdx >= 0) ? -1 : getCurrentPosIdx(x, y);
      if (infoIdx.value != currentIdx) {
        if (currentIdx >= 0) {
          const wkInfo = SURFACE_INFO_LIST_SET[getInfoSetIndex()][currentIdx];
          btnsetPosStyle.value =  {
            display: 'block',
            left: (wkInfo.cx + getMousePosOffsetX()) + 'px',
            top: (wkInfo.cy + getMousePosOffsetY()) + 'px'
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
      for (const wkAxisInfoList of AXISAREA_INFO_LIST_SET) {
        for (let wk_axis_inf of wkAxisInfoList) {
          wk_axis_inf.minx = wk_axis_inf.cx - AREA_OFFSET;
          wk_axis_inf.maxx = wk_axis_inf.cx + AREA_OFFSET;
          wk_axis_inf.miny = wk_axis_inf.cy - AREA_OFFSET;
          wk_axis_inf.maxy = wk_axis_inf.cy + AREA_OFFSET;
        }
      }
      for (const wkInfoList of SURFACE_INFO_LIST_SET) {
        for (let wk_surface_inf of wkInfoList) {
          wk_surface_inf.minx = wk_surface_inf.cx - AREA_OFFSET;
          wk_surface_inf.maxx = wk_surface_inf.cx + AREA_OFFSET;
          wk_surface_inf.miny = wk_surface_inf.cy - AREA_OFFSET;
          wk_surface_inf.maxy = wk_surface_inf.cy + AREA_OFFSET;
        }
      }
      // sortedArray = SURFACE_INFO_LIST.sort((n1,n2) => {
      //   if (n1.sortval > n2.sortval) return 1;
      //   if (n1.sortval < n2.sortval) return -1;
      //   return 0;
      // });
    };
    const getMousePosOffsetX = (): number => {
      if (cubeViewType.value == "horizon") {
        return (!isFrontViewMoving.value == !isAltView.value) ? X_OFFSET : 0;
      }
      return 0;
    };
    const getMousePosOffsetY = (): number => {
      if (cubeViewType.value == "vertical") {
        return (!isFrontViewMoving.value == !isAltView.value) ? Y_OFFSET : 0;
      }
      return 0;
    };
    const getCurrentAxisPosIdx = (x: number, y:number): number => {
      const wkOX = getMousePosOffsetX();
      const wkOY = getMousePosOffsetY();
      // subroutine
      const isInAxisInfo = (info: AxisAreaInfo): boolean => {
        // console.log(`info.minx=${info.minx}, wkOX=${wkOX}, x=${x}, isFrontViewMoving.value=${isFrontViewMoving.value}`);
        return (info.minx + wkOX <= x) && (x <= info.maxx + wkOX) && (info.miny + wkOY <= y) && (y <= info.maxy + wkOY);
      };

      const wkAxisInfoList = AXISAREA_INFO_LIST_SET[getInfoSetIndex()];
      if (axisInfoIdx.value >= 0 && isInAxisInfo(wkAxisInfoList[axisInfoIdx.value])) {
        return axisInfoIdx.value;
      }

      let retIdx = -1;
      wkAxisInfoList.forEach((info, index) => {
        if (retIdx >= 0) return;
        if (isInAxisInfo(info)) {
          retIdx = index;
          return;
        }
      });
      return retIdx;
    };
    const getCurrentPosIdx = (x: number, y:number): number => {
      const wkOX = getMousePosOffsetX();
      const wkOY = getMousePosOffsetY();
      const isInInfo = (info: SurfaceInfo): boolean => {
        return (info.minx + wkOX <= x) && (x <= info.maxx + wkOX) && (info.miny + wkOY <= y) && (y <= info.maxy + wkOY);
      };

      const wkSurfaceInfoList = SURFACE_INFO_LIST_SET[getInfoSetIndex()];
      if (infoIdx.value >= 0 && isInInfo(wkSurfaceInfoList[infoIdx.value])) {
        return infoIdx.value;
      }

      let retIdx = -1;
      wkSurfaceInfoList.forEach((info, index) => {
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
      //
      btnsetPosStyle,
      getFloatingPnlStyle,
      getAxixBtnStyle,
      getPanelClass,
      isAltView,
      isBtnActive,
      //
      toggleAltView,
      onMouseMove,
      onTouchEnd,
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
    cubeViewType: {type: String, required: true}
  },
})
</script>

<style>
.wasm-container, .wasm-container>input {
  position: absolute;
}
.wasm-sub-container {
  position: absolute;
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
.wasm-screen_altviewbtn {
  position: absolute;
  top: 45px;
  left: 15px;
}
.wasm-screen_floatingpnl{
  position: absolute;
}
.wasm-screen_floatingpnl>div{
  position: absolute;
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

.wasm-screen_xaxis_p, .wasm-screen_xaxis_n{
  transform: scale(1.7, 1.7) rotateY(60deg);
}
.wasm-screen_xaxis_n {
  top: 12px;
  left: 16px;
}
.wasm-screen_zaxis_p, .wasm-screen_zaxis_n {
  transform: scale(1.7, 1.7) rotateY(60deg);
}
.wasm-screen_zaxis_n {
  top: -10px;
  left: 18px;
}
.wasm-screen_yaxis_p, .wasm-screen_yaxis_n{
  transform: scale(1.5, 1);
  left: -12px;
}
.wasm-screen_yaxis_n{
  left: -12px;
  top: 20px;
}

.altview.horizon .wasm-front-view {
  left: 415px;
}
.altview.vertical .wasm-front-view {
  top: 415px;
}
.altview.alone .wasm-front-view {
  display: none;
}

.norview.horizon .wasm-back-view{
  left: 415px;
}
.norview.vertical .wasm-back-view{
  top: 415px;
}
.norview.alone .wasm-back-view{
  display: none;
}
</style>