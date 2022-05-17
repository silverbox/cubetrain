// import { mount } from '@vue/test-utils'
import { shallowMount } from '@vue/test-utils'
import ControlPanel from '@/components/ControlPanel.vue'

describe('Test ControlPanel.vue', () => {

  it('success to fire scramble', async () => {
    const wrapper = shallowMount(ControlPanel, {
      props: {
        defspeed: 40,
        defscramblestep: 20
      }
    })
    expect(wrapper).toBeDefined();

    const scramblebtn = wrapper.find('.controlpanel_scramble');
    expect(scramblebtn).toBeDefined();
    await scramblebtn.trigger('click')

    // console.log(wrapper.emitted().controlAction[0])
    // { controlAction: [ [ 'scramble', 20 ] ], click: [ [ [MouseEvent] ] ] }
    expect(wrapper.emitted().controlAction[0]).toStrictEqual(['scramble', 20]);
  })
})
