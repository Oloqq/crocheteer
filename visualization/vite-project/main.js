import * as THREE from 'three';
import { init, scene, controls, renderer, camera } from "./init";
import { LineSegmentsGeometry } from 'three/examples/jsm/lines/LineSegmentsGeometry.js';
import { LineMaterial } from 'three/examples/jsm/lines/LineMaterial.js';
import { LineSegments2 } from 'three/examples/jsm/lines/LineSegments2.js';

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

function createLink(start, end, width, color) {
  const points = [start, end];
  const geometry = new LineSegmentsGeometry().setPositions(points.flat());

  const material = new LineMaterial({
    color: color,
    linewidth: width, // This width is in pixels.
    worldUnits: false, // Set to true if you want the width to be in world units (meters).
  });
  material.resolution.set(window.innerWidth, window.innerHeight);


  const line = new LineSegments2(geometry, material);
  line.computeLineDistances();

  scene.add(line);
}

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


function main() {
  init();

  const s1pos = [1, 0.5, 0];
  const s2pos = [1, 0.5, 3];
  const s1 = sphere(s1pos, 0.5);
  const s2 = sphere(s2pos, 0.5);

  const link = createLink(s1pos, s2pos, 5, 0xff0000);

  animate();
}

main();