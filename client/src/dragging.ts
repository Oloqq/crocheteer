// import { app } from "./init";
import * as THREE from "three";
import { Display } from "./render3d";
import { PlushieMember, PlushieMemberId } from "./PlushieBody";
import World from "./World";
const raycaster = new THREE.Raycaster();
const mouse = new THREE.Vector2();

let display: Display;
let world: World;
let selectedObject: any & PlushieMemberId = null;
let draggedPosition: THREE.Vector3 | null = null;

function selectObject(event: MouseEvent) {
  // Calculate mouse position in normalized device coordinates (-1 to +1) for both components
  mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
  mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

  // Update the picking ray with the camera and mouse position
  raycaster.setFromCamera(mouse, display.camera);

  // Calculate objects intersecting the picking ray
  // const intersects = raycaster.intersectObjects(display.scene.children);
  const intersects = raycaster.intersectObjects(world.plushie!.nodes);

  for (let i = 0; i < intersects.length; i++) {
    if ((intersects[i].object as any).geometry.type === "SphereGeometry") {
      selectedObject = intersects[i].object;
      // console.log(selectedObject);
      break; // Assuming you want to select the first sphere intersected
    }
  }

  if (selectedObject) {
    // console.log("caught a thing");
    updateDragPlane();
    // draggedPosition = new THREE.Vector3();
    // draggedPosition.copy();
    display.controls.enabled = false;
  }
}

const dragPlane = new THREE.Plane();
const planeNormal = new THREE.Vector3(); // The plane's normal

function updateDragPlane() {
  planeNormal.copy(display.camera.position).normalize();
  dragPlane.setFromNormalAndCoplanarPoint(planeNormal, selectedObject.position);
}

function dragObject(event: MouseEvent) {
  if (!selectedObject) return;

  // Update mouse position
  mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
  mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

  // Update the raycaster
  raycaster.setFromCamera(mouse, display.camera);

  // Find intersection with the plane
  const intersection = new THREE.Vector3();
  if (raycaster.ray.intersectPlane(dragPlane, intersection)) {
    selectedObject.position.copy(intersection);
    draggedPosition = new THREE.Vector3();
    draggedPosition.copy(intersection);
    world.onThingMoved(
      selectedObject.kind,
      selectedObject.index,
      draggedPosition
    );
  }
}

function releaseObject(_event: MouseEvent) {
  if (selectedObject) {
    const x = selectedObject as PlushieMember;
    if (draggedPosition) {
      world.onThingMoved(x.kind, x.index, draggedPosition);
    } else {
      console.error("bruh we messed up dragging");
      return;
    }
    selectedObject = null;
    draggedPosition = null;
    display.controls.enabled = true;
  }
}

export function init(pWorld: World) {
  display = pWorld.display;
  world = pWorld;
  window.addEventListener("mousedown", selectObject);
  window.addEventListener("mousemove", dragObject);
  window.addEventListener("mouseup", releaseObject);
}
