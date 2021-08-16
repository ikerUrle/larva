<template>
  <div class="page">
    <div class="page_nav">
    <h1>Larva - SVG Image converter</h1>
    </div>
    <div class="page__main">
    <div class="page__main__dropzone" v-bind="getRootProps()">
      <input v-bind="getInputProps()" >
      <p>Drop a file, or click here to convert to SVG</p>
    </div>
    <div class="page__main__preview__container" >
      <div id="svg" v-html="svgImage" />
    </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import init, {convert} from 'larva'
import { ref } from 'vue'
import { useDropzone } from 'vue3-dropzone'

const onDrop = (acceptFiles:any, rejectReasons:any) => {
  let file = acceptFiles[0]
  var reader = new FileReader();
  reader.readAsDataURL(file);
  reader.onload =  () => {
    //Remove data:... part from string
    let img = reader.result?.toString().split(',')[1]!
    var svg = convert(img)
    svgImage.value = svg
  }
  reader.onerror = (error) => {
    console.log('Error: ', error);
  }
}
const { getRootProps, getInputProps, ...rest } = useDropzone({ onDrop })
init()
let svgImage = ref("Preview will be shown here")
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
.page__main {
  display:flex;
  max-height: 20rem;
}
#svg{
  max-height: 20rem !important;
}
.page__main__dropzone {
  padding: 2rem;
  border: .2rem dashed lightgray;
  margin: 0 2rem;
}
.page__main__preview__container {
  max-height: 80vh;
  overflow: auto;
}
</style>
