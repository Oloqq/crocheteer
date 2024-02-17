import { app } from "./init";
import { controlViaWebsocket } from './websocket';
import { setWorld } from "./interaction";

export { send } from "./websocket";

function animate() {
  requestAnimationFrame(animate);
  app.controls.update();
  app.renderer.render(app.scene, app.camera);
}

export function init() {
  app.init();
  return app;
}

export function connect(address, world) {
  setWorld(world);
  controlViaWebsocket(address, world);
  animate();
}