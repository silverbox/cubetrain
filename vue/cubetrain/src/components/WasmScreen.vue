<template>
  <div :id="id"></div>
</template>
<script lang="ts">
import { defineComponent, toRefs, onMounted, ref } from 'vue';
import init from '@/wasm/package.js';
import { start } from '@/wasm/package.js';

export default defineComponent({
  name: "WasmScreen",
  setup(props){
    const { id } = toRefs(props)
    const interfaceSetConfig = ref<any>(() => {});
    const setConfig = (type: string, val: number) => {
      console.log(type)
      console.log(val)
      const unitedstr = type + " " + String(val)
      interfaceSetConfig.value(unitedstr);
    };
    const onMountedOperation = () => {
      init('/wasm/package_bg.wasm').then(() => {
        const [set_config] = start(id.value);
        interfaceSetConfig.value = set_config
      });
    }
    onMounted(onMountedOperation);
    return {
      setConfig
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