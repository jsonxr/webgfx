# webgfx

A minimal graphics library inspired by three.js written in Rust, compiled to wasm in order to be as fast as possible in the browser. Initially rendered via WebGL with an eye towards WebGPU and modern browser developments. Currently probably slower than using webgl directly from js (see Future section) 

# How to install

```sh
npm install webgfx
```

# How to use

### index.html
```html
<html>
<body>
  <script module="">
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
const canvas = document.getElementById('canvas');
const renderer = new WebGLRenderer({ canvas });
renderer.render(scene, camera);
```

# Future

This library probably isn't usable until the following are implemented fully:
  
- Wasm Host Bindings

  allow RUST to talk directly to Web APIs without the overhead of making js shims. Allow garbage collection to work in Javascript so don't have to manually free Rust created objects.

  - https://github.com/WebAssembly/interface-types
  - https://github.com/WebAssembly/reference-types
  - https://github.com/WebAssembly/gc

- WebGPU

  This is a new spec that dramatically decreases the CPU overhead of sending a scene to the GPU. WebGL is a deprecated API that only allows for a small number of calls in each frame. Until this is official, will be rendered as WebGL.

  - https://gpuweb.github.io/gpuweb/
  - https://github.com/gfx-rs/wgpu-rs/issues/101
  - https://github.com/rust-gamedev/wg/issues/51

