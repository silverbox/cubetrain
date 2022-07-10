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
        <v-checkbox
          v-model="isBackViewVisible"
          label="背面ビュー"
          hide-details
          @change="onChangeBackViewVisible"
        ></v-checkbox>
      </v-col>
    </v-row>
  </v-container>
</template>
<script lang="ts">
import { defineComponent, toRefs, ref } from "vue";

export default defineComponent({
  name: "ControlPanel",
  setup(props: any, context: any){
    const { defspeed, defscramblestep, defIsButtonPanelVisible, defIsBackViewVisible } = toRefs(props)
    const speed = ref<number>(defspeed.value);
    const scramblestep = ref<number>(defscramblestep.value);
    const isButtonPanelVisible = ref<boolean>(defIsButtonPanelVisible.value);
    const isBackViewVisible = ref<boolean>(defIsBackViewVisible.value);

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
    const onChangeBackViewVisible = () => {
      context.emit("changeBackViewVisible", isBackViewVisible.value);
    };

    //
    return {
      speed,
      scramblestep,
      isButtonPanelVisible,
      isBackViewVisible,
      //
      controlAction,
      onChangeButtonPanelVisible,
      onChangeBackViewVisible,
    }
  },
  props: {
    defspeed: {type: Number, required: true},
    defscramblestep: {type: Number, required: true},
    defIsButtonPanelVisible: {type: Boolean, required: true},
    defIsBackViewVisible: {type: Boolean, required: true},
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
</style>defIsBackViewVisible