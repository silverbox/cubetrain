<template>
  <v-container class="grey lighten-5">
    <v-row>
      <v-col md="6" sm="6">
        <!-- <v-input
          label="回転速度"
          hide-details="auto"
          dense
          height="24px"
          :value="speed"
        ></v-input> -->
        <v-text-field
          label="回転速度"
          hide-details="auto"
          height="24"
          density="compact"
          :value="speed"
        ></v-text-field>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="6">
        <v-btn color="primary" block>
          リセット
          <v-icon>mdi-autorenew</v-icon>
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="6">
        <v-btn color="primary" block>
          スクランブル
          <v-icon>mdi-shuffle</v-icon>
        </v-btn>
      </v-col>
      <v-col md="4">
        <v-text-field
          label="手数"
          hide-details="auto"
          density="compact"
          :value="scramblestep"
        ></v-text-field>
      </v-col>
    </v-row>
    <v-row>
      <v-col md="3" class="item-center">
        <v-label class="input-label">
        </v-label>
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
      <v-col md="3">
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
      <v-col md="3">
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
      <v-col md="3">
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
      <v-col md="3">
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
      <v-col md="3">
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
      <v-col md="3">
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
      <v-col md="3">
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
      <v-col md="3">
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
import { defineComponent, toRefs } from 'vue';

export default defineComponent({
  name: "ControlPanel",
  setup(props: any){
    const { speed } = toRefs(props)
    console.log(speed)
    return {
    }
  },
  props: {
    speed: {type: Number, required: true},
    scramblestep: {type: Number, required: true},
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
</style>