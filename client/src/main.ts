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

  const visualizeButton = document.getElementById("visualize-button")!;
  visualizeButton.addEventListener("click", sendPattern);

  const exportButton = document.getElementById("export-button")!;
  exportButton.addEventListener("click", () => {
    send(`export-pointcloud`);
  });

  setupStatusTooltip(guiData);
});

function setupStatusTooltip(guiData: GuiData) {
  const statusButton = document.getElementById("status-box")!;
  const tooltip = document.createElement("div");
  tooltip.className = "tooltip";
  document.body.appendChild(tooltip);

  const showTooltip = (event: MouseEvent) => {
    tooltip.style.opacity = "1";
    tooltip.textContent = guiData.statusMessage;
    positionTooltip(event);
  };

  const moveTooltip = (event: MouseEvent) => {
    positionTooltip(event);
  };

  const hideTooltip = () => {
    tooltip.style.opacity = "0";
  };

  // Function to position the tooltip near the cursor
  const positionTooltip = (event: MouseEvent) => {
    const tooltipRect = tooltip.getBoundingClientRect();
    const offset = 10; // Distance between cursor and tooltip

    let x = event.pageX + offset;
    let y = event.pageY + offset;

    // Prevent tooltip from going off the right edge
    if (x + tooltipRect.width > window.innerWidth) {
      x = event.pageX - tooltipRect.width - offset;
    }

    // Prevent tooltip from going off the bottom edge
    if (y + tooltipRect.height > window.innerHeight) {
      y = event.pageY - tooltipRect.height - offset;
    }

    tooltip.style.left = `${x}px`;
    tooltip.style.top = `${y}px`;
  };

  // Attach event listeners to the status button
  statusButton.addEventListener("mouseenter", showTooltip);
  statusButton.addEventListener("mousemove", moveTooltip);
  statusButton.addEventListener("mouseleave", hideTooltip);
}
