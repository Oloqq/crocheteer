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