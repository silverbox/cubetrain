<template>
  <v-app>
    <v-main>
      <v-container class="grey lighten-5">
        <v-row>
          <v-col md="4">
            <ControlPanel/>
          </v-col>
          <v-col md="8">
            <WasmScreen id="wasmelemid"/>
          </v-col>
        </v-row>
      </v-container>
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
