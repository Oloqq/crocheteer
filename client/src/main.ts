import * as THREE from "three";
import { GuiData, initGui } from "./gui.ts";
import * as render from "./render3d.ts";
import World from "./World.ts";
import * as leftPanel from "./leftPanel/panel.ts";
import { send } from "./comms.ts";
import { setupTooltip } from "./utils/tooltip.ts";
import * as dragging from "./dragging.ts";

document.addEventListener("DOMContentLoaded", () => {
  const editor = leftPanel.init();
  const display3d = render.init();
  let guiData = new GuiData(
    () => editor.getValue(),
    (pattern: string) => editor.setValue(pattern)
  );

  const sendPattern = () => {
    send(`pattern ${guiData.getPattern()}`);
  };

  const syncPatternAndParams = () => {
    sendPattern();
  };

  let world = new World(
    "ws://127.0.0.1:8080",
    display3d,
    guiData,
    syncPatternAndParams
  );
  initGui(display3d, guiData, world);
  dragging.init(world);

  const visualizeButton = document.getElementById("visualize-button")!;
  visualizeButton.addEventListener("click", sendPattern);

  const updateButton = document.getElementById("update-button")!;
  updateButton.addEventListener("click", () => {
    send(`update ${guiData.getPattern()}`);
  });

  const exportButton = document.getElementById("export-button")!;
  exportButton.addEventListener("click", () => {
    send(`export-pointcloud`);
  });

  setupTooltip(
    document.getElementById("status-box")!,
    () => guiData.statusMessage
  );

  setupTooltip(
    document.getElementById("help")!,
    () => `
In the editor:
  CTRL+Enter: update pattern
  CTRL+P: pause/unpause
  `
  );

  function call(method: string, ...args: any[]) {
    if (method === "move") {
      let id = args[0];
      let x = args[1];
      let y = args[2];
      let z = args[3];
      world.onThingMoved("skin", id, new THREE.Vector3(x, y, z));
      return null;
    } else if (method === "pos") {
      let id = args[0];
      return world.plushie!.nodes[id].position;
    } else {
      console.warn("Unrecognized: ", method, args);
    }
  }

  (window as any).call = call;
});
