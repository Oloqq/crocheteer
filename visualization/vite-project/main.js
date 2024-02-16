import * as THREE from 'three';

import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

// 1. Initialization
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

// Initial camera position
camera.position.set(5, 5, 5);
camera.lookAt(scene.position);

// 2. Adding a Cube
const geometry = new THREE.BoxGeometry();
const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
const cube = new THREE.Mesh(geometry, material);
scene.add(cube);

// 3. Adding Grids
const gridSize = 10;
const gridDivisions = 10;
const gridHelperXY = new THREE.GridHelper(gridSize, gridDivisions);
const gridHelperYZ = new THREE.GridHelper(gridSize, gridDivisions);
const gridHelperXZ = new THREE.GridHelper(gridSize, gridDivisions);

gridHelperYZ.rotation.z = Math.PI / 2;
gridHelperXZ.rotation.x = Math.PI / 2;

scene.add(gridHelperXY);
// scene.add(gridHelperYZ);
// scene.add(gridHelperXZ);

// 4. Camera Controls
const controls = new OrbitControls(camera, renderer.domElement);
controls.screenSpacePanning = false; // Determines panning mode
controls.maxPolarAngle = Math.PI / 2; // Limit the camera's angle so it can't go below the ground

controls.rotateSpeed = 1.0; // Adjust rotation speed
controls.zoomSpeed = 1.2; // Adjust zoom speed
controls.panSpeed = 0.8; // Adjust panning speed

// 5. Animation Loop
function animate() {
  requestAnimationFrame(animate);
  controls.update();
  renderer.render(scene, camera);
}

animate();
