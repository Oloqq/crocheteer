import { GuiData, setupGui } from "./gui.ts";
import * as render from "./render3d.ts";
import World from "./World.ts";
import * as leftPanel from "./leftPanel/panel.ts";

setupSimulator();
document.addEventListener("DOMContentLoaded", leftPanel.init);

function setupSimulator() {
  const display3d = render.init();
  let guiData = new GuiData();
  let world = new World("ws://127.0.0.1:8080", display3d, guiData);
  setupGui(display3d, guiData, world);
}
