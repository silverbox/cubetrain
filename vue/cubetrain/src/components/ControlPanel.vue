<template>
  <v-container class="grey lighten-5">
    <v-row>
      <v-col md="8" class="item-center">
        <v-label class="input-label">回転速度</v-label>
      </v-col>
      <v-col md="4">
        <input v-model="speed" class="controlpanel_speedval control-input" type="number">
      </v-col>
    </v-row>
    <v-row>
      <v-col md="8" offset-md="2" class="item-center">
        <v-btn color="normal" block @click="controlAction('speed')" class="controlpanel_speed">
          <v-icon>mdi-refresh</v-icon>
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="8" class="item-center">
        <v-label class="input-label">スクランブル</v-label>
      </v-col>
      <v-col md="4">
        <input v-model="scramblestep" class="controlpanel_scramblestep control-input" type="number">
      </v-col>
    </v-row>
    <v-row>
      <v-col md="8" offset-md="2" class="item-center">
        <v-btn color="normal" block @click="controlAction('scramble')" class="controlpanel_scramble">
          <v-icon>mdi-shuffle</v-icon>
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="8" class="item-center">
        <v-label class="input-label">リセット</v-label>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="8" offset-md="2" class="item-center">
        <v-btn color="normal" block @click="controlAction('reset')">
          <v-icon>mdi-autorenew</v-icon>
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="12" class="item-center">
        <v-divider/>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="8" class="item-center">
        <v-label class="input-label">表示切り替え</v-label>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="8" offset-md="1" class="item-center">
        <v-checkbox
          v-model="isButtonPanelVisible"
          label="操作ボタン"
          hide-details
          @change="onChangeButtonPanelVisible"
        ></v-checkbox>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="8" offset-md="1" class="item-center">
        <v-radio-group v-model="cubeViewType" @change="onChangeCubeViewType">
          <v-radio
            v-for="cubeViewTypeInfo in cubeViewTypeInfoList"
            :key="cubeViewTypeInfo.value"
            :label="cubeViewTypeInfo.label"
            :value="cubeViewTypeInfo.value"
          ></v-radio>
        </v-radio-group>
      </v-col>
    </v-row>
  </v-container>
</template>
<script lang="ts">
import { defineComponent, toRefs, ref } from "vue";
import { CubeViewType } from '@/class/types';

interface CubeViewTypeInfo{
  label: string,
  value: CubeViewType
}
const CUBE_VIE_TYPE_INFO: CubeViewTypeInfo[] = [{
  label: "正面＆背面（横）",
  value: "horizon"
},{
  label: "正面＆背面（縦）",
  value: "vertical"
},{
  label: "片方向",
  value: "alone"
}];

export default defineComponent({
  name: "ControlPanel",
  setup(props: any, context: any){
    const { defspeed, defscramblestep, defIsButtonPanelVisible, defCubeViewType } = toRefs(props)

    const speed = ref<number>(defspeed.value);
    const scramblestep = ref<number>(defscramblestep.value);
    const isButtonPanelVisible = ref<boolean>(defIsButtonPanelVisible.value);
    const cubeViewType = ref<CubeViewType>(defCubeViewType.value);
    const cubeViewTypeInfoList = CUBE_VIE_TYPE_INFO;
    //
    const controlAction = (type: string) => {
      let cfgvalue = 0;
      if (type == "speed") {
        cfgvalue = speed.value;
      } else if (type == "scramble") {
        cfgvalue = scramblestep.value;
      } else {
        cfgvalue = 0;
      }
      context.emit("controlAction", type, cfgvalue);
    };
    const onChangeButtonPanelVisible = () => {
      context.emit("changeButtonPanelVisible", isButtonPanelVisible.value);
    };
    const onChangeCubeViewType = () => {
      context.emit("changeCubeViewType", cubeViewType.value);
    };

    //
    return {
      cubeViewTypeInfoList,
      //
      cubeViewType,
      speed,
      scramblestep,
      isButtonPanelVisible,
      //
      controlAction,
      onChangeButtonPanelVisible,
      onChangeCubeViewType,
    }
  },
  props: {
    defspeed: {type: Number, required: true},
    defscramblestep: {type: Number, required: true},
    defIsButtonPanelVisible: {type: Boolean, required: true},
    defCubeViewType: {type: String, required: true},
  }
})
</script>
<style scoped>
.input-label {
  height: 100%;
}
.item-center {
  justify-content:center;
  text-align:center;
}
.control-input {
  height: 90%;
  width: 40px;
}
</style>