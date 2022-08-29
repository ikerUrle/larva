## Larva_rs

> This project is based on code from [VTracer](https://github.com/visioncortex/vtracer/). 

Larva converts base64 images to SVG.

This module is written in Rust and compiled to WebAssembly using [wasm-pack](https://github.com/rustwasm/wasm-pack).

# Usage

```js
import init, { convert } from 'larva_rs'

// Make sure you run this before "convert"
init()

// Convert accepts only base64 format (without image/png...)
var base64img = ''

// Returns the svg as a string.
var svg = convert(base64img)
```


