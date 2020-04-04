import { createBoxGeometry, Mesh, MeshBasicMaterial, PerspectiveCamera, Scene, WebGLRenderer } from "webgfx";

// 1. Build the Scene
const geometry = createBoxGeometry(1, 1, 1);
console.log('geometry1: ', geometry);
const material = new MeshBasicMaterial();
const cube = new Mesh(geometry, material);
const scene = new Scene();
console.log('scene.len: ', scene.len());
scene.add(cube);
console.log('scene.len: ', scene.len());

// 2. Render the Scene statically (not in an animation loop)
const camera = new PerspectiveCamera({ x: 0, y: 100, z: 0});
const renderer = new WebGLRenderer({canvas: document.getElementById('canvas')});
console.log(renderer);
console.log(geometry);
//renderer.render(scene, camera);
renderer.render(scene);
