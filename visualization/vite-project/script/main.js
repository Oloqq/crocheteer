import * as THREE from 'three';
import { app } from "./init";
import * as create from "./create";

function animate() {
  requestAnimationFrame(animate);
  app.controls.update();
  app.renderer.render(app.scene, app.camera);
}

function main() {
  app.init();
  create.bindToScene(app.scene);

  const s1 = create.sphere([0, 0, 0], 0.5);


  // const s1pos = [1, 0.5, 0];
  // const s2pos = [1, 0.5, 3];
  // const s1 = sphere(s1pos, 0.5);
  // const s2 = sphere(s2pos, 0.5);

  // const link = createLink(s1pos, s2pos, 5, 0xff0000);

  animate();
}

main();