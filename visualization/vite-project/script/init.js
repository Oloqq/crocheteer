import * as THREE from 'three';
import * as dat from 'dat.gui';

import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer();
export const app = {
  scene: new THREE.Scene(),
  camera: camera,
  renderer: renderer,
  controls: new OrbitControls(camera, renderer.domElement),
  gui: new dat.GUI(),

  init: function () {
    initScene();
    initGui();
  }
};

const gridSize = 10;
const gridDivisions = 10;
const gridHelperXY = new THREE.GridHelper(gridSize, gridDivisions);
const gridHelperYZ = new THREE.GridHelper(gridSize, gridDivisions);
const gridHelperXZ = new THREE.GridHelper(gridSize, gridDivisions);


function initScene() {
  app.renderer.setSize(window.innerWidth, window.innerHeight);
  document.body.appendChild(renderer.domElement);
  app.camera.position.set(5, 5, 5);
  app.camera.lookAt(app.scene.position);
  app.controls.saveState();

  gridHelperYZ.rotation.z = Math.PI / 2;
  gridHelperXZ.rotation.x = Math.PI / 2;
  app.scene.add(gridHelperXY);
  app.scene.add(gridHelperYZ);
  app.scene.add(gridHelperXZ);

  app.controls.screenSpacePanning = true;
  app.controls.maxPolarAngle = Math.PI / 2; // Limit the camera's angle so it can't go below the ground

  app.controls.rotateSpeed = 1.0;
  app.controls.zoomSpeed = 1.2;
  app.controls.panSpeed = 0.8;
}

function initGui() {
  const guiControls = {
    showXYGrid: true,
    showYZGrid: false,
    showXZGrid: false,
  };

  function resetCamera() {
    app.controls.reset();
  }

  function togglePause() {
  }

  const gui = app.gui;

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