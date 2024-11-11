import * as THREE from "three";

export function sphere(
  scene: THREE.Scene,
  coords: any,
  r: any,
  color: crapi.RGB,
) {
  const geometry = new THREE.SphereGeometry(r);
  const material = new THREE.MeshBasicMaterial({ color: new THREE.Color(color[0], color[1], color[2]) });
  const result = new THREE.Mesh(geometry, material);
  result.position.x = coords[0];
  result.position.y = coords[1];
  result.position.z = coords[2];
  scene.add(result);
  return result;
}

export function link(
  scene: THREE.Scene,
  point1: any,
  point2: any,
  width: any,
  color: any
) {
  let curve = new THREE.CatmullRomCurve3([point1, point2]);
  let tubeGeometry = new THREE.TubeGeometry(curve, 20, width, 8, false);

  // Step 4: Create a Material and Mesh
  let material = new THREE.MeshBasicMaterial({ color: color });
  let mesh = new THREE.Mesh(tubeGeometry, material);
  scene.add(mesh);

  return mesh;
}
