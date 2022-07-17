import { createApp } from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify'
import { loadFonts } from './plugins/webfontloader'
import { Vue3Mq } from "vue3-mq";

loadFonts()

const app = createApp(App);
app.use(vuetify);
app.use(Vue3Mq, {
  preset: "vuetify"
});
app.mount('#app')
