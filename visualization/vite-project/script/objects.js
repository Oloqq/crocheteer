import * as THREE from 'three';
import { LineSegmentsGeometry } from 'three/examples/jsm/lines/LineSegmentsGeometry.js';
import { LineMaterial } from 'three/examples/jsm/lines/LineMaterial.js';
import { LineSegments2 } from 'three/examples/jsm/lines/LineSegments2.js';

let scene = undefined;
export function setScene(s) {
  scene = s;
}

export function sphere(coords, r) {
  const geometry = new THREE.SphereGeometry(r);
  const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
  const result = new THREE.Mesh(geometry, material);
  result.position.x = coords[0];
  result.position.y = coords[1];
  result.position.z = coords[2];
  scene.add(result);
  return result;
}

export function createLink(start, end, width, color) {
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