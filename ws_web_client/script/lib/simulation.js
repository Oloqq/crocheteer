import { app } from "./init";
import { controlViaWebsocket } from './websocket';

export { send, isOpened as isWebSocketOpened } from "./websocket";

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
  app.world = world;
  controlViaWebsocket(address, world);
  app.status.innerText = "Connected";
  animate();
}