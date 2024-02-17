import * as THREE from 'three';
import { init, scene, controls, renderer, camera } from "./init";
import { sphere, createLink, setScene } from "./objects";

setScene(scene);

function animate() {
  requestAnimationFrame(animate);
  controls.update();
  renderer.render(scene, camera);
}

const raycaster = new THREE.Raycaster();
const mouse = new THREE.Vector2();
let selectedObject = null; // To keep track of the selected sphere

function onMouseDown(event) {
  // Calculate mouse position in normalized device coordinates (-1 to +1) for both components
  mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
  mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

  // Update the picking ray with the camera and mouse position
  raycaster.setFromCamera(mouse, camera);

  // Calculate objects intersecting the picking ray
  const intersects = raycaster.intersectObjects(scene.children);

  for (let i = 0; i < intersects.length; i++) {
    if (intersects[i].object.geometry.type === 'SphereGeometry') {
      selectedObject = intersects[i].object;
      console.log(selectedObject);
      break; // Assuming you want to select the first sphere intersected
    }
  }

  if (selectedObject) {
    updateDragPlane(); // Update the plane for dragging
    controls.enabled = false;
  }
}
window.addEventListener('mousedown', onMouseDown);

const dragPlane = new THREE.Plane();
const planeNormal = new THREE.Vector3(); // The plane's normal

function updateDragPlane() {
  planeNormal.copy(camera.position).normalize();
  dragPlane.setFromNormalAndCoplanarPoint(planeNormal, selectedObject.position);
}

function onMouseMove(event) {
  if (!selectedObject) return;

  // Update mouse position
  mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
  mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

  // Update the raycaster
  raycaster.setFromCamera(mouse, camera);

  // Find intersection with the plane
  const intersection = new THREE.Vector3();
  if (raycaster.ray.intersectPlane(dragPlane, intersection)) {
    selectedObject.position.copy(intersection);
  }
}
window.addEventListener('mousemove', onMouseMove);

function onMouseUp(event) {
  selectedObject = null; // Clear the selection
  controls.enabled = true;
}
window.addEventListener('mouseup', onMouseUp);

const s1 = sphere([0, 0, 0], 0.5);

const ws = new WebSocket("ws://127.0.0.1:8080");

ws.onopen = function (event) {
  console.log("Connected to WebSocket server");
};

ws.onmessage = function (event) {
  console.log("Position data received: ", event.data);
  const s1pos = JSON.parse(event.data);
  s1.position.x = s1pos[0];
  s1.position.y = s1pos[1];
  s1.position.z = s1pos[2];
};

ws.onerror = function (error) {
  console.log("WebSocket error: ", error);
};

function main() {
  init();

  // const s1pos = [1, 0.5, 0];
  // const s2pos = [1, 0.5, 3];
  // const s1 = sphere(s1pos, 0.5);
  // const s2 = sphere(s2pos, 0.5);

  // const link = createLink(s1pos, s2pos, 5, 0xff0000);

  animate();
}

main();