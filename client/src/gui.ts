import * as dat from "dat.gui";
import * as comms from "./comms";
import { Display, restoreDefaultView } from "./render3d";

export interface GuiData {
  paused: boolean;
  stepCallback: () => void;
}

export function setupGui(display3d: Display): GuiData {
  const gui = new dat.GUI();
  const data: GuiData = {
    paused: true,
    stepCallback: () => {
      comms.send("advance");
    },
  };

  gui
    .add(data, "paused")
    .name("Pause")
    .onChange((value) => {
      if (value) {
        comms.send("pause");
      } else {
        comms.send("resume");
      }
    });

  gui.add(data, "stepCallback").name("Step");

  const display = gui.addFolder("Display");
  {
    display.open();
    display
      .add({ _: () => restoreDefaultView(display3d) }, "_")
      .name("Reset camera");
    // display.add({""}, 0);
    // .name("XY grid")
    // .onChange((value) => {
    //   display3d.grids.xy.visible = value;
    // });
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
