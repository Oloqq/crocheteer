import * as dat from "dat.gui";
import { Display, restoreDefaultView } from "./render3d";

export interface GuiData {
  paused: boolean;
  showEdges: boolean;
}

export interface GuiCallbacks {
  paused: (val: boolean) => void;
  step: () => void;
  showEdges: (val: boolean) => void;
}

export function setupGui(display3d: Display, callbacks: GuiCallbacks): GuiData {
  const gui = new dat.GUI();
  const data: GuiData = {
    paused: true,
    showEdges: true,
  };

  gui.add(data, "paused").name("Pause").onChange(callbacks.paused);
  gui.add(callbacks, "step").name("Step");

  const display = gui.addFolder("Display");
  {
    display.open();
    display
      .add({ _: () => restoreDefaultView(display3d) }, "_")
      .name("Reset camera");

    display
      .add(data, "showEdges")
      .name("Show links")
      .onChange(callbacks.showEdges);
  }

  // let counter = 0;
  // const setCounter = (count: number) => {
  //   counter = count;
  //   element.innerHTML = `count is ${counter}`;
  // };
  // element.addEventListener("click", () => setCounter(counter + 1));
  // setCounter(0);

  return data;
}
