<template>
  <v-app>
    <v-main>
      <ControlPanel/>
      <WasmScreen id="wasmelemid"/>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import ControlPanel from './components/ControlPanel.vue'
import WasmScreen from './components/WasmScreen.vue'

export default defineComponent({
  name: 'App',

  components: {
    ControlPanel,
    WasmScreen
  },

  data () {
    return {
      //
    }
  },
})
</script>
