<template>
  <v-container class="grey lighten-5">
    <v-row>
      <v-col md="2" class="item-center">
        <v-label class="input-label">設定</v-label>
      </v-col>
      <v-col md="1" class="item-center">
        <v-label class="input-label">速度</v-label>
      </v-col>
      <v-col md="1" class="item-center">
        <input v-model="speed" class="control-input" type="number">
      </v-col>
      <v-col md="6">
        <v-btn color="normal" block @click="controlAction('speed')">
          回転速度変更
          <v-icon>mdi-refresh</v-icon>
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="1" offset-md="2" class="item-center">
        <v-label class="input-label">手数</v-label>
      </v-col>
      <v-col md="1" class="item-center">
        <input v-model="scramblestep" class="control-input" type="number">
      </v-col>
      <v-col md="6">
        <v-btn color="normal" block @click="controlAction('scramble')">
          スクランブル
          <v-icon>mdi-shuffle</v-icon>
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="6" offset-md="4">
        <v-btn color="normal" block @click="controlAction('reset')">
          リセット
          <v-icon>mdi-autorenew</v-icon>
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="6" offset-md="4">
        <v-btn :color="keyOperationColor" block @keypress="onKeyOperation" @focus="keyOperation = true" @blur="keyOperation = false">
          キーボード操作
          <v-icon v-if="keyOperation">mdi-play</v-icon>
          <v-icon v-else>mdi-pause</v-icon>
        </v-btn>
      </v-col>
    </v-row>
    <v-row style="height: 40px;">
      <v-col md="6" offset-md="3">
        <v-label class="input-label"></v-label>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="2" class="item-center">
        <v-label class="input-label">操作</v-label>
      </v-col>
      <v-col md="3" class="item-center">
        <v-label class="input-label">x軸</v-label>
      </v-col>
      <v-col md="3" class="item-center">
        <v-label class="input-label">y軸</v-label>
      </v-col>
      <v-col md="3" class="item-center">
        <v-label class="input-label">z軸</v-label>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="2">
        <v-label class="input-label">
          全体(正)
        </v-label>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('x', 'all', 'n')">
          x
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('y', 'all', 'n')">
          y
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('z', 'all', 'n')">
          z
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="2">
        <v-label class="input-label">
          全体(逆)
        </v-label>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('x', 'all', 'p')">
          x'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('y', 'all', 'p')">
          y'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('z', 'all', 'p')">
          z'
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="2">
        <v-label class="input-label">
          前/上(正)
        </v-label>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('x', 'pos', 'n')">
          R
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('y', 'pos', 'n')">
          U
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('z', 'pos', 'n')">
          F
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="2">
        <v-label class="input-label">
          前/上(逆)
        </v-label>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('x', 'pos', 'p')">
          R'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('y', 'pos', 'p')">
          U'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('z', 'pos', 'p')">
          F'
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="2">
        <v-label class="input-label">
          中(正)
        </v-label>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('x', 'neu', 'n')">
          M
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('y', 'neu', 'p')">
          E
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('z', 'neu', 'n')">
          S
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="2">
        <v-label class="input-label">
          中(逆)
        </v-label>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('x', 'neu', 'p')">
          M'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('y', 'neu', 'n')">
          E'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('z', 'neu', 'p')">
          S'
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="2">
        <v-label class="input-label">
          奥/下(正)
        </v-label>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('x', 'neg', 'p')">
          L
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('y', 'neg', 'p')">
          D
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('z', 'neg', 'p')">
          B
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="2">
        <v-label class="input-label">
          奥/下(逆)
        </v-label>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('x', 'neg', 'n')">
          L'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('y', 'neg', 'n')">
          D'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block @click="onButtonOperation('z', 'neg', 'n')">
          B'
        </v-btn>
      </v-col>
    </v-row>  </v-container>
</template>
<script lang="ts">
import { defineComponent, toRefs, ref, computed } from "vue";
import { RotateInfo, Axis, Layer, Dir, cubeutils } from '@/class/cubeutils';
const { getRotateInfo, getSymbolFromKey } = cubeutils();

export default defineComponent({
  name: "ControlPanel",
  setup(props: any, context: any){
    const { defspeed, defscramblestep } = toRefs(props)
    const speed = ref<number>(defspeed.value);
    const keyOperation = ref<boolean>(false);
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
    const rotateAction = (rotateInfo: RotateInfo) => {
      const { axis, layer, dir } = rotateInfo;
      context.emit("rotateAction", axis, layer, dir);
    };
    const onButtonOperation = (axis: Axis, layer: Layer, dir: Dir) => {
      rotateAction({ axis: axis, layer: layer, dir: dir });
    };
    const onKeyOperation = (e: KeyboardEvent) => {
      const wkSymbol = getSymbolFromKey(e.key);
      if (wkSymbol != undefined){
        rotateAction(getRotateInfo(wkSymbol, e.shiftKey));
      }
    };

    //
    const keyOperationColor = computed((): string => {
      if (keyOperation.value) return "primary"; else return "normal";
    });
    return {
      speed,
      scramblestep,
      keyOperation,
      //
      controlAction,
      rotateAction,
      onKeyOperation,
      onButtonOperation,
      //
      keyOperationColor
    }
  },
  props: {
    defspeed: {type: Number, required: true},
    defscramblestep: {type: Number, required: true},
  },
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