import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import PrimeVue from 'primevue/config'
import Aura from '@primeuix/themes/aura'
import Ripple from 'primevue/ripple'
import ToastService from 'primevue/toastservice';
import App from './App.vue'
import router from './router'

const app = createApp(App)
const pinia = createPinia()

app.use(PrimeVue, {
  theme: {
    preset: Aura,
  },
})
app.use(pinia)
app.use(ToastService);
app.use(createPinia())
app.use(router)
app.directive('ripple', Ripple)

app.mount('#app')
