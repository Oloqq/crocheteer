import * as THREE from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";

export interface Display {
  scene: THREE.Scene;
  camera: THREE.PerspectiveCamera;
  renderer: THREE.WebGLRenderer;
  controls: OrbitControls;
  grids: {
    xy: THREE.GridHelper;
    xz: THREE.GridHelper;
    yz: THREE.GridHelper;
  };
}

export function init(): Display {
  const camera = new THREE.PerspectiveCamera(
    75,
    window.innerWidth / window.innerHeight,
    0.1,
    1000
  );

  let canvasContainer = document.getElementById("main-content");
  if (!canvasContainer) {
    throw "malformed html: missing canvas for three.js";
  }

  const renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setSize(canvasContainer.clientWidth, canvasContainer.clientHeight);
  canvasContainer.appendChild(renderer.domElement);

  const display = {
    scene: new THREE.Scene(),
    camera: camera,
    renderer: renderer,
    controls: new OrbitControls(camera, renderer.domElement),
    grids: grids,
  };
  saveDefaultView(display);

  display.scene.add(grids.xy);
  display.scene.add(grids.xz);
  display.scene.add(grids.yz);

  function animate() {
    requestAnimationFrame(animate);
    display.controls.update();
    display.renderer.render(display.scene, display.camera);
  }
  animate();

  window.addEventListener("resize", () => {
    renderer.setSize(canvasContainer.clientWidth, canvasContainer.clientHeight);
    camera.aspect = canvasContainer.clientWidth / canvasContainer.clientHeight;
    camera.updateProjectionMatrix();
  });

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

export const grids = (() => {
  const cellSize = 1;
  const floorCells = 10;
  const verticalCells = 20;

  return {
    xy: new THREE.GridHelper(floorCells * cellSize, floorCells),
    yz: (() => {
      const grid = new THREE.GridHelper(
        verticalCells * cellSize,
        verticalCells
      );
      grid.rotation.z = Math.PI / 2;
      grid.visible = false;
      return grid;
    })(),
    xz: (() => {
      const grid = new THREE.GridHelper(
        verticalCells * cellSize,
        verticalCells
      );
      grid.rotation.x = Math.PI / 2;
      grid.visible = false;
      return grid;
    })(),
  };
})();
