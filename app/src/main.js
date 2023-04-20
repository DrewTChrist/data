import { createApp } from 'vue'
import App from './App.vue'
import naiveUI from 'naive-ui'
import router from './router'

//import './assets/main.css'

const app = createApp(App)

app.use(router)
app.use(naiveUI)

app.mount('#app')
