<template>
  <v-app>
    <v-app-bar
      app
      color="deep-purple accent-4"
      dense
      dark
      priority=0
    >
      <v-app-bar-nav-icon></v-app-bar-nav-icon>

      <v-toolbar-title>Page title</v-toolbar-title>

      <v-spacer></v-spacer>

      <v-btn icon>
        <v-icon>mdi-heart</v-icon>
      </v-btn>

      <v-btn icon>
        <v-icon>mdi-magnify</v-icon>
      </v-btn>

      <v-btn
        icon
        @click="(showHistory = !showHistory)"
      >
        <v-icon>mdi-dots-vertical</v-icon>
        <!-- <v-menu
          left
          bottom
          activator="parent"
          anchor="start"
        >
          <v-list>
            <v-list-item
              v-for="n in 5"
              :key="n"
              @click="() => {}"
            >
              <v-list-item-title>Option {{ n }}</v-list-item-title>
            </v-list-item>
          </v-list>
        </v-menu> -->
      </v-btn>
    </v-app-bar>

    <v-navigation-drawer permanent>
      <!-- -->
    </v-navigation-drawer>

    <v-navigation-drawer permanent position="right" v-if="showHistory">
      <v-list density="compact">
        <v-list-subheader>操作履歴</v-list-subheader>
        <v-list-item
          v-for="(step, i) in rotateStepList"
          :key="i"
          :value="step"
          active-color="primary"
        >
          <v-list-item-title v-text="step.symbol"></v-list-item-title>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <!-- Provides the application the proper gutter -->
        <v-container class="grey lighten-5" fluid>
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
import { RotateStep, RotateStepManager } from '@/class/rotateStepManager'

export default defineComponent({
  name: 'App',
  setup(){
    const wasm = ref();
    const stepManager: RotateStepManager = new RotateStepManager();
    const showHistory = ref<boolean>(true);
    //
    const rotateStepList = ref<Array<RotateStep>>([]);
    //
    const onControlAction = (type: string, val: number) => {
      if (wasm.value != null) {
        wasm.value.setConfig(type, val);
      }
    };
    const onRotateAction = (axis: string, layer: string, dir: string) => {
      if (wasm.value != null) {
        wasm.value.rotate(axis, layer, dir);
        rotateStepList.value.push(stepManager.addStep(axis, layer, dir));
      }
    };
    return {
      wasm,
      showHistory,
      rotateStepList,
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
