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

      <v-menu
        left
        bottom
      >
        <template v-slot:activator="{ on, attrs }">
          <v-btn
            icon
            v-bind="attrs"
            v-on="on"
          >
            <v-icon>mdi-dots-vertical</v-icon>
          </v-btn>
        </template>

        <v-list>
          <v-list-item
            v-for="n in 5"
            :key="n"
            @click="() => {}"
          >
            <v-list-item-title>Option {{ n }}</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>
    </v-app-bar>

    <v-navigation-drawer permanent>
      <!-- -->
    </v-navigation-drawer>
    <v-navigation-drawer color="grey-darken-2" permanent position="right">

    </v-navigation-drawer>
    <!-- Sizes your content based upon application components -->
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
