// import { mount } from '@vue/test-utils'
import { shallowMount } from '@vue/test-utils'
import WasmScreen from '@/components/WasmScreen.vue'
import flushPromises from 'flush-promises'

describe('WasmScreen.vue', () => {
  it('success to connect with wasm module', async () => {
    const wait = async (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
    const wrapper = shallowMount(WasmScreen, {
      props: {
        id: 'unittest-id'
      }
    })
    await flushPromises();
    await wait(2000);

    expect(wrapper).toBeDefined();
    expect(wrapper.vm.onWasmAnimation()).toBe(0);
  })
})
