import * as THREE from 'three';
import * as dat from 'dat.gui';

import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer();

const gridSize = 10;
const gridDivisions = 10;
const gridHelperXY = new THREE.GridHelper(gridSize, gridDivisions);
const gridHelperYZ = new THREE.GridHelper(gridSize, gridDivisions);
const gridHelperXZ = new THREE.GridHelper(gridSize, gridDivisions);

const controls = new OrbitControls(camera, renderer.domElement);

(function init() {
  renderer.setSize(window.innerWidth, window.innerHeight);
  document.body.appendChild(renderer.domElement);
  camera.position.set(5, 5, 5);
  camera.lookAt(scene.position);
  controls.saveState();

  gridHelperYZ.rotation.z = Math.PI / 2;
  gridHelperXZ.rotation.x = Math.PI / 2;
  scene.add(gridHelperXY);
  scene.add(gridHelperYZ);
  scene.add(gridHelperXZ);

  controls.screenSpacePanning = true;
  controls.maxPolarAngle = Math.PI / 2; // Limit the camera's angle so it can't go below the ground

  controls.rotateSpeed = 1.0;
  controls.zoomSpeed = 1.2;
  controls.panSpeed = 0.8;
})();

function resetCamera() {
  controls.reset();
}

(function initGui() {
  const gui = new dat.GUI();
  const guiControls = {
    showXYGrid: true,
    showYZGrid: false,
    showXZGrid: false,
  };

  gui.add({ resetCamera }, 'resetCamera').name('Reset camera');
  gui.add(guiControls, 'showXYGrid').name('Show XY Grid').onChange((value) => {
    gridHelperXY.visible = value; // Toggle visibility based on the GUI control
  });
  gui.add(guiControls, 'showYZGrid').name('Show YZ Grid').onChange((value) => {
    gridHelperYZ.visible = value; // Toggle visibility based on the GUI control
  });
  gui.add(guiControls, 'showXZGrid').name('Show XZ Grid').onChange((value) => {
    gridHelperXZ.visible = value; // Toggle visibility based on the GUI control
  });

  // Initially set the gridHelperYZ visibility
  gridHelperXY.visible = guiControls.showXYGrid;
  gridHelperYZ.visible = guiControls.showYZGrid;
  gridHelperXZ.visible = guiControls.showXZGrid;
})();

function sphere(coords, r) {
  const geometry = new THREE.SphereGeometry(r);
  const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
  const result = new THREE.Mesh(geometry, material);
  result.position.x = coords[0];
  result.position.y = coords[1];
  result.position.z = coords[2];
  scene.add(result);
  return result;
}

function animate() {
  requestAnimationFrame(animate);
  controls.update();
  renderer.render(scene, camera);
}

function main() {
  const s1pos = [1, 0, 0];
  const s2pos = [1, 0, 3];
  const s1 = sphere(s1pos, 0.5);
  const s2 = sphere(s2pos, 0.5);

  animate();
}

main();