import * as THREE from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";

interface Display {
  scene: THREE.Scene;
  camera: THREE.PerspectiveCamera;
  renderer: THREE.WebGLRenderer;
  controls: OrbitControls;
}

export function init(): Display {
  const camera = new THREE.PerspectiveCamera(
    75,
    window.innerWidth / window.innerHeight,
    0.1,
    1000
  );

  const renderer = new THREE.WebGLRenderer();
  renderer.setSize(window.innerWidth, window.innerHeight);
  document.body.appendChild(renderer.domElement);

  const display = {
    scene: new THREE.Scene(),
    camera: camera,
    renderer: renderer,
    controls: new OrbitControls(camera, renderer.domElement),
  };
  saveDefaultView(display);

  cube(display);
  display.scene.add(grids.xy);
  // display.scene.add(gridHelperYZ);
  // display.scene.add(gridHelperXZ);

  return display;
}

export function restoreDefaultView(display: Display) {
  display.controls.reset();
}

function saveDefaultView(display: Display) {
  display.camera.position.set(5, 5, 5);
  display.camera.lookAt(display.scene.position);
  display.controls.saveState();
}

const grids = (() => {
  const gridSize = 10;
  const gridDivisions = 10;

  return {
    xy: new THREE.GridHelper(gridSize, gridDivisions),
    yz: (() => {
      const grid = new THREE.GridHelper(gridSize, gridDivisions);
      grid.rotation.z = Math.PI / 2;
      return grid;
    })(),
    xz: (() => {
      const grid = new THREE.GridHelper(gridSize, gridDivisions);
      grid.rotation.x = Math.PI / 2;
      return grid;
    })(),
  };
})();

function cube(display: any) {
  const geometry = new THREE.BoxGeometry(1, 1, 1);
  const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
  const cube = new THREE.Mesh(geometry, material);
  display.scene.add(cube);

  display.camera.position.z = 5;

  function animate() {
    cube.rotation.x += 0.01;
    cube.rotation.y += 0.01;
    display.renderer.render(display.scene, display.camera);
  }
  display.renderer.setAnimationLoop(animate);
}
