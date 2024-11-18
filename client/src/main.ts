import { GuiData, initGui } from "./gui.ts";
import * as render from "./render3d.ts";
import World from "./World.ts";
import * as leftPanel from "./leftPanel/panel.ts";
import { send } from "./comms.ts";

document.addEventListener("DOMContentLoaded", () => {
  const editor = leftPanel.init();
  const display3d = render.init();
  let guiData = new GuiData(
    () => editor.getValue(),
    (pattern: string) => editor.setValue(pattern)
  );

  const sendPattern = () => {
    send(`pattern flow ${guiData.getPattern()}`);
  };

  let world = new World("ws://127.0.0.1:8080", display3d, guiData, sendPattern);
  initGui(display3d, guiData, world);

  const visualizeButton = document.getElementById("visualize-button");
  visualizeButton?.addEventListener("click", sendPattern);

  const exportButton = document.getElementById("export-button");
  exportButton?.addEventListener("click", () => {
    send(`export-pointcloud`);
  });
});
