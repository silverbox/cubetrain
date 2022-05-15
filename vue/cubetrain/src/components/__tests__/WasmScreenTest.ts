import { mount } from '@vue/test-utils'

// The component to test
const MessageComponent = {
  template: '<p>{{ msg }}</p>',
  props: ['msg']
}

describe('HelloWorld.vue', () => {
  it('renders props.msg when passed', () => {
    const wrapper = mount(MessageComponent, {
      props: {
        msg: 'Hello world'
      }
    })
  
    // Assert the rendered text of the component
    expect(wrapper.text()).toContain('Hello world')
  })
})
