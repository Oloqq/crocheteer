import * as dat from "dat.gui";
import { Display, restoreDefaultView } from "./render3d";
import * as comms from "./comms";
import World from "./World";

export class GuiData {
  world?: World = undefined;
  paused: boolean = true;
  showEdges: boolean = true;

  pausedCallback = (val: boolean) => {
    if (!this.world) return;

    if (val) {
      comms.send("pause");
      if (this.showEdges) {
        this.world.plushie?.drawLinks();
      }
    } else {
      this.world.plushie?.clearLinks();
      comms.send("resume");
    }
  };

  step = () => {
    if (!this.world) return;

    this.world.plushie?.clearLinks();
    comms.send("advance");
  };

  showEdgesCallback = (val: boolean) => {
    if (!this.world) return;

    val ? this.world.plushie?.drawLinks() : this.world.plushie?.clearLinks();
  };
}

export function setupGui(
  display3d: Display,
  data: GuiData,
  world: World
): GuiData {
  const gui = new dat.GUI();
  data.world = world;

  gui.add(data, "paused").name("Pause").onChange(data.pausedCallback);
  gui.add(data, "step").name("Step");

  const display = gui.addFolder("Display");
  {
    display.open();
    display
      .add({ _: () => restoreDefaultView(display3d) }, "_")
      .name("Reset camera");

    display
      .add(data, "showEdges")
      .name("Show links")
      .onChange(data.showEdgesCallback);
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
