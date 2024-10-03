import * as dat from "dat.gui";
import * as comms from "./comms";
import { Display, restoreDefaultView } from "./render3d";

export let data = {
  paused: true,
};

export function setupGui(display3d: Display): void {
  const gui = new dat.GUI();

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

  const display = gui.addFolder("Display");
  {
    display.open();
    display
      .add({ _: () => restoreDefaultView(display3d) }, "_")
      .name("Reset camera");
    display.add({""}, 0);
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
}
