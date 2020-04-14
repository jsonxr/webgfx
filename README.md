# webgfx

A graphics library inspired by three.js compiled to WebAssembly. The goal of this library is to be as fast as possible to write 3d graphics in a browser from javascript. This library can not achieve this goal until the interface-types proposal (formally known as host bindings) is implemented. Without this, there is a lot of marshalling between js and WebAssembly.


# How to install

```sh
npm install webgfx
```

# How to use

### index.html
```html
<html>
<body>
  <script module="index.js">
  <canvas id="canvas"></canvas>
</body>
</html>
```

### index.js
```javascript
import { BoxGeometry, Mesh, MeshBasicMaterial, PerspectiveCamera, Renderer, Scene } from 'webgfx';

// 1. Build the Scene
const geometry = new BoxGeometry(1, 1, 1);
const material = new MeshBasicMaterial();
const cube = new Mesh(geometry, material);
const scene = new Scene();
scene.add(cube);

// 2. Render the Scene statically (not in an animation loop)
const camera = new PerspectiveCamera();
let renderer = new WebGLRenderer({ canvas: document.getElementById('canvas') });
//     let renderer = new WebGLRenderer();

renderer.render(scene, camera);
```

# Future

This library probably isn't usable until the following are implemented fully:
  
- Wasm Host Bindings

  allow RUST to talk directly to Web APIs without the overhead of making js shims. Allow garbage collection to work in Javascript so don't have to manually free Rust created objects.

  - https://github.com/WebAssembly/proposals
  - https://fitzgen.github.io/wasm-cg-wasm-bindgen/#51
  - https://github.com/WebAssembly/interface-types
  - https://github.com/WebAssembly/reference-types
  - https://github.com/WebAssembly/gc

- WebGPU

  This is a new spec that dramatically decreases the CPU overhead of sending a scene to the GPU. WebGL is a deprecated API that only allows for a small number of calls in each frame. Until this is official, will be rendered as WebGL.

  - https://gpuweb.github.io/gpuweb/
  - https://github.com/gfx-rs/wgpu-rs/issues/101
  - https://github.com/rust-gamedev/wg/issues/51


Accessing raw memory:
https://egghead.io/lessons/webpack-access-webassembly-memory-directly-from-javascript