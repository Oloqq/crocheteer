import { app } from "./init";
import Plushie from './plushie';
import { controlViaWebsocket } from './websocket';
import "./interaction";

function animate() {
  requestAnimationFrame(animate);
  app.controls.update();
  app.renderer.render(app.scene, app.camera);
}

function main() {
  app.init();
  const plushie = new Plushie();
  controlViaWebsocket("ws://127.0.0.1:8080", plushie);

  animate();
}

main();