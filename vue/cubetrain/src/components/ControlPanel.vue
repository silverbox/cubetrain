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
  </v-container>
</template>
<script lang="ts">
import { defineComponent, toRefs, ref } from "vue";

export default defineComponent({
  name: "ControlPanel",
  setup(props: any, context: any){
    const { defspeed, defscramblestep } = toRefs(props)
    const speed = ref<number>(defspeed.value);
    const scramblestep = ref<number>(defscramblestep.value);

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

    //
    return {
      speed,
      scramblestep,
      //
      controlAction,
    }
  },
  props: {
    defspeed: {type: Number, required: true},
    defscramblestep: {type: Number, required: true},
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