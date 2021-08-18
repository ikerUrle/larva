import { createApp } from 'vue'
import App from './App.vue'
import init from 'larva_rs'

import "./App.css"
// larva wasm init
init()

createApp(App).mount('#app')
