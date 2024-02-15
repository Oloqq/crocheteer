// 1. Initialization
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

// Initial camera position
camera.position.set(5, 5, 5);
camera.lookAt(scene.position);

// 2. Adding a Cube
const geometry = new THREE.BoxGeometry();
const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
const cube = new THREE.Mesh(geometry, material);
scene.add(cube);

// 3. Adding Grids
const gridSize = 10;
const gridDivisions = 10;
const gridHelperXY = new THREE.GridHelper(gridSize, gridDivisions);
const gridHelperYZ = new THREE.GridHelper(gridSize, gridDivisions);
const gridHelperXZ = new THREE.GridHelper(gridSize, gridDivisions);

gridHelperYZ.rotation.z = Math.PI / 2;
gridHelperXZ.rotation.x = Math.PI / 2;

scene.add(gridHelperXY);
scene.add(gridHelperYZ);
scene.add(gridHelperXZ);

// 4. Camera Controls
let isDragging = false;
let isCtrlPressed = false;
let previousMousePosition = {
    x: 0,
    y: 0
};

document.addEventListener('mousemove', handleMouseMove, false);
document.addEventListener('mousedown', () => isDragging = true, false);
document.addEventListener('mouseup', () => isDragging = false, false);
document.addEventListener('keydown', (e) => isCtrlPressed = e.ctrlKey, false);
document.addEventListener('keyup', (e) => isCtrlPressed = e.ctrlKey, false);
document.addEventListener('wheel', handleMouseWheel, false);

function handleMouseMove(event) {
  if (!isDragging) {
      return;
  }

  const deltaMove = {
      x: event.clientX - previousMousePosition.x,
      y: event.clientY - previousMousePosition.y
  };

  if (isCtrlPressed) {
      // Implement camera rotation
      const rotateSpeed = 0.005; // Adjust rotation speed as needed
      camera.rotation.y += deltaMove.x * rotateSpeed;
      camera.rotation.x += deltaMove.y * rotateSpeed;
  } else {
      // Implement camera translation
      const translateSpeed = 0.01; // Adjust translation speed as needed
      camera.position.x -= deltaMove.x * translateSpeed;
      camera.position.y += deltaMove.y * translateSpeed;
  }

  previousMousePosition = {
      x: event.clientX,
      y: event.clientY
  };
}

function handleMouseWheel(event) {
    // Implement zooming
}

// 5. Animation Loop
function animate() {
    requestAnimationFrame(animate);
    renderer.render(scene, camera);
}

animate();
