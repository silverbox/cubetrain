<template>
  <div :id="id"></div>
</template>
<script lang="ts">
import { defineComponent, toRefs, onMounted } from 'vue';
import init from '@/wasm/package.js';
import { start } from '@/wasm/package.js';

export default defineComponent({
  name: "WasmScreen",
  setup(props){
    const { id } = toRefs(props)
    const onMountedOperation = () => {
      console.log("mounted:" + id.value)
      init('/wasm/package_bg.wasm').then(() => {
          start("wasmelemid");
      });
    }
    onMounted(onMountedOperation);
    return {
    }
  },
  props: {
    id: {type: String, required: true}
  },
  // mounted () {
  //   this.id = this._uid
  // }
})
</script>