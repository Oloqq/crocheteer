import * as THREE from 'three';
import * as dat from 'dat.gui';

import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

export const scene = new THREE.Scene();
export const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
export const renderer = new THREE.WebGLRenderer();

const gridSize = 10;
const gridDivisions = 10;
const gridHelperXY = new THREE.GridHelper(gridSize, gridDivisions);
const gridHelperYZ = new THREE.GridHelper(gridSize, gridDivisions);
const gridHelperXZ = new THREE.GridHelper(gridSize, gridDivisions);

export const controls = new OrbitControls(camera, renderer.domElement);
export const gui = new dat.GUI();

export function init() {
  initScene();
  initGui();
}

function initScene() {
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
}

function initGui() {
  const guiControls = {
    showXYGrid: true,
    showYZGrid: false,
    showXZGrid: false,
  };

  function resetCamera() {
    controls.reset();
  }

  function togglePause() {
  }

  gui.add({ togglePause }, 'togglePause').name('Pause/resume');
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
}