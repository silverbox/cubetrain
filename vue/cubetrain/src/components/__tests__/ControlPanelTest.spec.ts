// import { mount } from '@vue/test-utils'
import { shallowMount } from '@vue/test-utils'
import ControlPanel from '@/components/ControlPanel.vue'

describe('Test ControlPanel.vue', () => {

  it('success to fire scramble', async () => {
    const STEP = 20
    const wrapper = shallowMount(ControlPanel, {
      props: {
        defspeed: 40,
        defscramblestep: STEP,
        defIsButtonPanelVisible: true,
        defIsBackViewVisible: true
      }
    })
    expect(wrapper).toBeDefined();

    const scramblebtn = wrapper.find('.controlpanel_scramble');
    expect(scramblebtn).toBeDefined();
    await scramblebtn.trigger('click')

    // console.log(wrapper.emitted().controlAction[0])
    // { controlAction: [ [ 'scramble', 20 ] ], click: [ [ [MouseEvent] ] ] }
    expect(wrapper.emitted().controlAction[0]).toStrictEqual(['scramble', STEP]);
  })
})
