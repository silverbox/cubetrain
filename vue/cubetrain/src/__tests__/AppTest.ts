import { mount, VueWrapper } from '@vue/test-utils'
import flushPromises from 'flush-promises'
import App from '@/App.vue'
jest.mock('@/wasm/package.js');
import init, { start, on_animation } from '@/wasm/package.js';

(init as any).mockResolvedValue();
(start as any).mockImplementation((id: string) => [jest.fn(), jest.fn()]);
(on_animation as any).mockImplementation(() => 0);

const setStepValueAndScramble = async (wrapper: VueWrapper<any>, step: number) => {
  // set valute to step
  const scramblestep = wrapper.find('.controlpanel_scramblestep');
  expect(scramblestep).toBeDefined();
  scramblestep.setValue(step);

  // fire scramble
  const scramblebtn = wrapper.find('.controlpanel_scramble');
  expect(scramblebtn).toBeDefined();
  await scramblebtn.trigger('click');
  await flushPromises();
  await wrapper.vm.waitForAnimation();
}

const playBackStep = async (wrapper: VueWrapper<any>, step: number) => {
  const playbackbtn = wrapper.find('.app_playbackonestep');
  expect(playbackbtn).toBeDefined();
  for (let i = 0; i < step; i++) {
    await playbackbtn.trigger('click');
  }
  await flushPromises();
  await wrapper.vm.waitForAnimation();
}

const clickHistoryItem = async (wrapper: VueWrapper<any>, historyIndex: number) => {
  const historyItemList = wrapper.findAll('.app_history-list > .app_history-item');
  const historyItem = historyItemList[historyIndex];
  expect(historyItem).toBeDefined();
  await historyItem.trigger('click');
}

const executeMenuAction = async (wrapper: VueWrapper<any>, menuId: string) => {
  const menuBtn = wrapper.find('.app_history-menu-btn');
  expect(menuBtn).toBeDefined();
  await menuBtn.trigger('click');
  
  const tgtMenuItem = wrapper.find('.app_history-menu-item-' + menuId);
  expect(tgtMenuItem).toBeDefined();
  await tgtMenuItem.trigger('click');
}

describe('Test App.vue', () => {

  it('success to fire scramble and number of history', async () => {
    const wrapper = mount(App, {
      props: {}
    })
    expect(wrapper).toBeDefined();

    const SCRAMMLE_STEP = 20;
    const PLAY_BACK_STEP = 3;

    // set speed
    const speed_input = wrapper.find('.controlpanel_speedval');
    expect(speed_input).toBeDefined();
    speed_input.setValue(100);
    const speed_btn = wrapper.find('.controlpanel_speed');
    expect(speed_btn).toBeDefined();
    await speed_btn.trigger('click');

    // do scramble
    await setStepValueAndScramble(wrapper, SCRAMMLE_STEP);
    
    // check number of steps
    expect(wrapper.vm.rotateStepList.length).toBe(SCRAMMLE_STEP);

    // check playback status
    await playBackStep(wrapper, PLAY_BACK_STEP);
    // console.log(wrapper.vm.rotateStepList);
    expect(wrapper.vm.rotateStepList[SCRAMMLE_STEP - PLAY_BACK_STEP - 1].rotateStatus).toBe('done');
    expect(wrapper.vm.rotateStepList[SCRAMMLE_STEP - PLAY_BACK_STEP].rotateStatus).toBe('bef');
  })

  it('playback steps and revert step', async () => {
    const wrapper = mount(App, {
      props: {}
    })
    expect(wrapper).toBeDefined();
    const SCRAMMLE_STEP = 20;
    const PLAY_BACK_STEP = 3;
    const REVERT_IDX = 15;
    const RESULT_CNT = REVERT_IDX + 1;
    await setStepValueAndScramble(wrapper, SCRAMMLE_STEP);
    await playBackStep(wrapper, PLAY_BACK_STEP);
    await clickHistoryItem(wrapper, REVERT_IDX);
    await executeMenuAction(wrapper, 'revert');

    expect(wrapper.vm.rotateStepList.length).toBe(RESULT_CNT);
  })
})
