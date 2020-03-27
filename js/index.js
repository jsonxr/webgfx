import { BoxGeometry, Mesh, MeshBasicMaterial, PerspectiveCamera, Scene, WebGLRenderer } from "webgfx";
console.log('Hello from js')
//import { Mesh, MeshBasicMaterial, PerspectiveCamera, WebGLRenderer, Scene } from 'webgfx';

const callback = (canvas) => {console.log('hello callback: ', canvas)}

// 1. Build the Scene
const geometry = new BoxGeometry(1, 1, 1);
const material = new MeshBasicMaterial();
const cube = new Mesh(geometry, material);
const scene = new Scene();
// scene.add(cube);

// 2. Render the Scene statically (not in an animation loop)
const camera = new PerspectiveCamera({ x: 0, y: 100, z: 0});
console.log(JSON.stringify(camera), camera.position);
const canvas = document.getElementById('canvas');
const renderer = new WebGLRenderer({ canvas, callback:"my callback" });
renderer.setCanvas(canvas);
renderer.render(scene, camera);
