<template>
  <v-app>
    <v-main>
      <v-container class="grey lighten-5">
        <v-row>
          <v-col md="4">
            <ControlPanel
              :defspeed=40
              :defscramblestep=24
              @controlAction="onControlAction"
              @rotateAction="onRotateAction"
            />
          </v-col>
          <v-col md="8">
            <WasmScreen
              id="wasmelemid"
              ref="wasm"
            />
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import ControlPanel from './components/ControlPanel.vue'
import WasmScreen from './components/WasmScreen.vue'

export default defineComponent({
  name: 'App',
  setup(){
    const wasm = ref();
    const onControlAction = (type: string, val: number) => {
      if (wasm.value != null) {
        wasm.value.setConfig(type, val);
      }
    };
    const onRotateAction = (axis: string, layer: string, dir: string) => {
      if (wasm.value != null) {
        wasm.value.rotate(axis, layer, dir);
      }
    };
    return {
      wasm,
      onControlAction,
      onRotateAction
    };
  },
  components: {
    ControlPanel,
    WasmScreen
  },
})
</script>
