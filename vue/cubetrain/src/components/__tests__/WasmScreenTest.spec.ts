import { shallowMount } from '@vue/test-utils'
import WasmScreen from '@/components/WasmScreen.vue'

jest.mock('@/wasm/package.js');
import init, { start, on_animation } from '@/wasm/package.js';

(init as any).mockResolvedValue();
(start as any).mockImplementation((id: string) => [jest.fn(), jest.fn()]);
(on_animation as any).mockImplementation(() => 0);

describe('WasmScreen.vue', () => {
  it('success to connect with wasm module', async () => {
    const wrapper = shallowMount(WasmScreen, {
      props: {
        id: 'unittest-id',
        isBackViewVisible: true
      }
    });

    expect(wrapper).toBeDefined();
    expect(wrapper.vm.onWasmAnimation()).toBe(0);
  })
})
