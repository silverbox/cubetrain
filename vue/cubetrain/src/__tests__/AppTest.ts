import { mount } from '@vue/test-utils'
import flushPromises from 'flush-promises'
import App from '@/App.vue'

describe('Test App.vue', () => {

  it('success to fire scramble and number of history', async () => {
    const wrapper = mount(App, {
      props: {}
    })
    expect(wrapper).toBeDefined();

    const STEP = 20;

    // set speed
    const speed_input = wrapper.find('.controlpanel_speedval');
    expect(speed_input).toBeDefined();
    speed_input.setValue(100);
    const speed_btn = wrapper.find('.controlpanel_speed');
    expect(speed_btn).toBeDefined();
    await speed_btn.trigger('click');

    // set valute to step
    const scramblestep = wrapper.find('.controlpanel_scramblestep');
    expect(scramblestep).toBeDefined();
    scramblestep.setValue(STEP);

    // fire scramble
    const scramblebtn = wrapper.find('.controlpanel_scramble');
    expect(scramblebtn).toBeDefined();
    await scramblebtn.trigger('click');

    // check number of steps
    expect(wrapper.vm.rotateStepList.length).toBe(STEP);
    await flushPromises();
    await wrapper.vm.waitForAnimation();

    // check playback status
    const playbackbtn = wrapper.find('.app_playbackonestep');
    expect(playbackbtn).toBeDefined();
    await playbackbtn.trigger('click');
    await playbackbtn.trigger('click');
    await playbackbtn.trigger('click');
    await flushPromises();
    await wrapper.vm.waitForAnimation();
    // console.log(wrapper.vm.rotateStepList);
    expect(wrapper.vm.rotateStepList[STEP - 4].rotateStatus).toBe("done");
    expect(wrapper.vm.rotateStepList[STEP - 3].rotateStatus).toBe("bef");
  })
})
