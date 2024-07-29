import { app } from "./init";
import * as THREE from 'three';
const raycaster = new THREE.Raycaster();
const mouse = new THREE.Vector2();
let selectedObject = null;

const ENABLE_DRAGGING = false;

function selectObject(event) {
  // Calculate mouse position in normalized device coordinates (-1 to +1) for both components
  mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
  mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

  // Update the picking ray with the camera and mouse position
  raycaster.setFromCamera(mouse, app.camera);

  // Calculate objects intersecting the picking ray
  const intersects = raycaster.intersectObjects(app.scene.children);

  for (let i = 0; i < intersects.length; i++) {
    if (intersects[i].object.geometry.type === 'SphereGeometry') {
      selectedObject = intersects[i].object;
      console.log(selectedObject);
      break; // Assuming you want to select the first sphere intersected
    }
  }

  if (selectedObject) {
    updateDragPlane(); // Update the plane for dragging
    app.controls.enabled = false;
    // send("pause");
  }
}

const dragPlane = new THREE.Plane();
const planeNormal = new THREE.Vector3(); // The plane's normal

function updateDragPlane() {
  planeNormal.copy(app.camera.position).normalize();
  dragPlane.setFromNormalAndCoplanarPoint(planeNormal, selectedObject.position);
}

function dragObject(event) {
  if (!selectedObject) return;

  // Update mouse position
  mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
  mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

  // Update the raycaster
  raycaster.setFromCamera(mouse, app.camera);

  // Find intersection with the plane
  const intersection = new THREE.Vector3();
  if (raycaster.ray.intersectPlane(dragPlane, intersection)) {
    selectedObject.position.copy(intersection);
    if (app.world) {
      world.mouseMove(selectedObject);
    }
  }
}


function releaseObject(event) {
  if (selectedObject) {
    if (app.world) {
      app.world.mouseUp(selectedObject);
    }
    selectedObject = null; // Clear the selection
    app.controls.enabled = true;
    // send("resume");
  }
}

if (ENABLE_DRAGGING) {
  window.addEventListener('mousedown', selectObject);
  window.addEventListener('mousemove', dragObject);
  window.addEventListener('mouseup', releaseObject);
}
