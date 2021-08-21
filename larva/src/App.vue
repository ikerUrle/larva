<template>
  <div class="page">
    <div class="page__nav">
    <img class="page__nav__logo" src="/larva.svg" />
    <div class="page__nav__title">
      <h1>Larva</h1>
      <h2>SVG Image Converter</h2>
    </div>
    </div>
    <div class="page__main">
    <div class="card main" v-bind="getRootProps()">
      <input v-bind="getInputProps()" >
      <p v-if="loading">Converting...</p>
      <p v-else>Drop a file, or click here to convert to SVG</p>
    </div>
    <img class="card preview" :src="previewUrl" alt="Result will be shown here">
    <div class="card download" @click="onDownloadClick">Download</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { convert } from 'larva_rs'
import { ref } from 'vue'
import { useDropzone } from 'vue-use-dropzone'

const loading = ref(false)
const previewUrl = ref('')

let resultSvg = ''
let dropPath = 'result.svg'

const onDrop = (acceptFiles:any, rejectReasons:any) => {
  let file = acceptFiles[0]
  if (file.type !== 'image/png') return alert('Only pngs please')
  dropPath = file.path.replace('.*','.svg')
  loading.value = true
  var reader = new FileReader()
  reader.readAsDataURL(file)
  reader.onload =  () => {
    //Remove data:... part from string
    let img = reader.result?.toString().split(',')[1]!
    var svg = convert(img)
    resultSvg = svg.trim()
    var blob = new Blob([svg], {type:"image/svg+xml;charset=utf-8"})
    var domURL = self.URL || self.webkitURL || self
    previewUrl.value = domURL.createObjectURL(blob)
    loading.value = false
  }
  reader.onerror = (error) => {
    loading.value = false
    console.log('Error: ', error);
  }
}
const { getRootProps, getInputProps, ...rest } = useDropzone({ onDrop })

const onDownloadClick= () => {
  var element = document.createElement('a')
  element.setAttribute('href', 'data:image/svg+xml;charset=utf-8,' + encodeURIComponent(resultSvg))
  element.setAttribute('download', dropPath)
  element.click()
}
</script>

<style>
#app {
  font-family: "Quicksand", Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: var(--primary-color);
  --primary-color: rgb(255, 255, 255);
  --secondary-color: rgb(217, 216, 218);
  --accent-color: rgb(52, 189, 125);
  --background: #000;
  background: var(--background);
  height: 100%;
}
html,body{
  margin:0;
  height: 100%;
  background: var(--background);
}
.page{
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height:100%;
  background-image: linear-gradient(to bottom, #323031, #312f31, #2f2f30, #2e2f30, #2d2e2f, #2c3033, #2a3236, #273439, #1f3a3d, #193f3d, #1a4437, #25482e);
}
.page__main {
  display:flex;
  gap: 4rem;
  align-items: center;
}
.page__nav {
  position: absolute;
  top:0;
  left:0;
  display:flex;
  gap: .4rem;
  align-items: center;
  padding: 0 .4rem;
}
.page__nav img {
  height: 7rem;
}
.page__nav__title{
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  justify-content: center;
}
.page__nav__title > * {
  margin:0;
}
.page__nav h2 {
  color: var(--secondary-color);
  font-size: 1.4rem;
}
.card {
  padding: 2rem;
  border: .1rem solid rgb(102, 102, 102);
  display:flex;
  align-items: center;
  justify-content: center;
  font-size: 1.4rem;
  border-radius: 100%;
  height: 10rem;
  width: 10rem;
}
.preview {
  color: var(--secondary-color);
  padding: 0;
  height: 14rem;
  width: 14rem;
}
.main {
  height: 20rem;
  width: 20rem;
}
</style>
