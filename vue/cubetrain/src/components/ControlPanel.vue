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
        <!-- <v-text-field
          label="回転速度"
          hide-details="auto"
          density="compact"
          :value="speed"
          :rules="rules"
        ></v-text-field> -->
      </v-col>
      <v-col md="6">
        <v-btn color="primary" block @click="controlAction('speed')">
          回転速度変更
          <v-icon>mdi-shuffle</v-icon>
        </v-btn>
      </v-col>    </v-row>
    <v-row>
      <v-col md="1" offset-md="2" class="item-center">
        <v-label class="input-label">手数</v-label>
      </v-col>
      <v-col md="1" class="item-center">
        <input v-model="scramblestep" class="control-input" type="number">
        <!-- <v-text-field
          label="手数"
          hide-details="auto"
          density="compact"
          :value="scramblestep"
          :rules="rules"
        ></v-text-field> -->
      </v-col>
      <v-col md="6">
        <v-btn color="primary" block @click="controlAction('scramble')">
          スクランブル
          <v-icon>mdi-shuffle</v-icon>
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="6" offset-md="4">
        <v-btn color="primary" block>
          リセット
          <v-icon>mdi-autorenew</v-icon>
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
        <v-btn block>
          x
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
          y
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
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
        <v-btn block>
          x'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
          y'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
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
        <v-btn block>
          R
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
          U
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
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
        <v-btn block>
          R'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
          U'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
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
        <v-btn block>
          M
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
          E
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
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
        <v-btn block>
          M'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
          E'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
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
        <v-btn block>
          L
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
          D
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
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
        <v-btn block>
          L'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
          D'
        </v-btn>
      </v-col>
      <v-col md="3">
        <v-btn block>
          B'
        </v-btn>
      </v-col>
    </v-row>  </v-container>
</template>
<script lang="ts">
import { defineComponent, toRefs, ref } from "vue";

export default defineComponent({
  name: "ControlPanel",
  setup(props: any, context: any){
    const { defspeed, defscramblestep } = toRefs(props)
    console.log(defspeed.value)
    const speed = ref<number>(defspeed.value);
    const scramblestep = ref<number>(defscramblestep.value);
    // const speed = ref<String>("sss");
    // const scramblestep = ref<String>("wwwe");
    // const rules = [];

    const controlAction = (type: string) => {
      let cfgvalue = 0;
      if (type == "speed") {
        cfgvalue = speed.value;
      } else {
        cfgvalue = scramblestep.value;
      }
      context.emit("controlAction", type, cfgvalue);
    };
    return {
      speed,
      scramblestep,
      // rules,
      //
      controlAction
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