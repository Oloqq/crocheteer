import * as THREE from "three";

export function destroy(scene: THREE.Scene, thing: THREE.Mesh) {
  if (thing.geometry) thing.geometry.dispose();
  if (thing.material) (thing.material as THREE.Material).dispose();
  scene.remove(thing);
}

export function destroyGroup(scene: THREE.Scene, garbage: THREE.Group) {
  garbage.traverse((protochild) => {
    // I don't have time for retroactively designed type systems
    let child = protochild as any;
    if (child.isMesh) {
      if (child.geometry) {
        child.geometry.dispose();
      }

      if (child.material) {
        if (Array.isArray(child.material)) {
          child.material.forEach((material: any) => material.dispose());
        } else {
          child.material.dispose();
        }
      }
    }
  });
  scene.remove(garbage);
}

export function sphere(
  scene: THREE.Scene,
  coords: any,
  r: any,
  color: THREE.Color
) {
  const geometry = new THREE.SphereGeometry(r);
  const material = new THREE.MeshBasicMaterial({
    color: color,
  });
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
  color: THREE.Color
) {
  let curve = new THREE.CatmullRomCurve3([point1, point2]);
  let tubeGeometry = new THREE.TubeGeometry(curve, 20, width, 8, false);

  // Step 4: Create a Material and Mesh
  let material = new THREE.MeshBasicMaterial({
    color: color,
  });
  let mesh = new THREE.Mesh(tubeGeometry, material);
  scene.add(mesh);

  return mesh;
}

export function arrowFromAngles(
  scene: THREE.Scene,
  origin: any,
  theta: number,
  phi: number,
  length: any,
  color: THREE.Color,
  shaftRadius = 0.05,
  headLength = 0.2,
  headRadius = 0.1
) {
  const arrowGroup = new THREE.Group();

  // Normalize the direction
  const dir = new THREE.Vector3(
    Math.sin(phi) * Math.cos(theta),
    Math.sin(phi) * Math.sin(theta),
    Math.cos(phi)
  ).normalize();

  // Calculate shaft length
  const shaftLength = length - headLength;

  // Create the shaft geometry and material
  const shaftGeometry = new THREE.CylinderGeometry(
    shaftRadius, // Top radius
    shaftRadius, // Bottom radius
    shaftLength,
    12 // Radial segments for smoother appearance
  );
  const shaftMaterial = new THREE.MeshBasicMaterial({
    color: color,
  });
  const shaft = new THREE.Mesh(shaftGeometry, shaftMaterial);

  // Position the shaft
  shaft.position.copy(origin).add(dir.clone().multiplyScalar(shaftLength / 2));

  // Align the shaft with the direction
  shaft.quaternion.setFromUnitVectors(new THREE.Vector3(0, 1, 0), dir);

  // Create the head geometry and material
  const headGeometry = new THREE.ConeGeometry(headRadius, headLength, 12);
  const headMaterial = new THREE.MeshBasicMaterial({
    color: shaftMaterial.color,
  });
  const head = new THREE.Mesh(headGeometry, headMaterial);

  // Position the head
  head.position
    .copy(origin)
    .add(dir.clone().multiplyScalar(shaftLength + headLength / 2));

  // Align the head with the direction
  head.quaternion.copy(shaft.quaternion);

  // Add shaft and head to the group
  arrowGroup.add(shaft);
  arrowGroup.add(head);

  // Add the group to the scene
  scene.add(arrowGroup);

  return arrowGroup;
}

export function arrow(
  scene: THREE.Scene,
  origin: any,
  direction: any,
  length: any,
  color: THREE.Color,
  shaftRadius = 0.05,
  headLength = 0.2,
  headRadius = 0.1
) {
  const arrowGroup = new THREE.Group();

  // Normalize the direction
  const dir = direction.clone().normalize();

  // Calculate shaft length
  const shaftLength = length - headLength;

  // Create the shaft geometry and material
  const shaftGeometry = new THREE.CylinderGeometry(
    shaftRadius, // Top radius
    shaftRadius, // Bottom radius
    shaftLength,
    12 // Radial segments for smoother appearance
  );
  const shaftMaterial = new THREE.MeshBasicMaterial({
    color: color,
  });
  const shaft = new THREE.Mesh(shaftGeometry, shaftMaterial);

  // Position the shaft
  shaft.position.copy(origin).add(dir.clone().multiplyScalar(shaftLength / 2));

  // Align the shaft with the direction
  shaft.quaternion.setFromUnitVectors(new THREE.Vector3(0, 1, 0), dir);

  // Create the head geometry and material
  const headGeometry = new THREE.ConeGeometry(headRadius, headLength, 12);
  const headMaterial = new THREE.MeshBasicMaterial({
    color: shaftMaterial.color,
  });
  const head = new THREE.Mesh(headGeometry, headMaterial);

  // Position the head
  head.position
    .copy(origin)
    .add(dir.clone().multiplyScalar(shaftLength + headLength / 2));

  // Align the head with the direction
  head.quaternion.copy(shaft.quaternion);

  // Add shaft and head to the group
  arrowGroup.add(shaft);
  arrowGroup.add(head);

  // Add the group to the scene
  scene.add(arrowGroup);

  return arrowGroup;
}

export function disk(
  scene: THREE.Scene,
  x: number,
  y: number,
  z: number,
  angleX: number,
  angleY: number,
  radius: number,
  color: THREE.Color,
  opacity: number
) {
  const segments = 32; // Number of segments for smoothness
  const geometry = new THREE.CircleGeometry(radius, segments);
  const material = new THREE.MeshBasicMaterial({
    color: color,
    side: THREE.DoubleSide,
    transparent: true,
    opacity: opacity,
  });
  const circle = new THREE.Mesh(geometry, material);

  circle.position.set(x, y, z);
  circle.rotation.x = angleX;
  circle.rotation.y = angleY;

  scene.add(circle);

  return circle;
}
