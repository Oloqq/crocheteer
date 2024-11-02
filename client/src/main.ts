import "./style.css";
import { GuiCallbacks, setupGui } from "./gui.ts";
import * as render from "./render3d.ts";
import * as comms from "./comms";
import World from "./World.ts";

const display3d = render.init();
let world = new World("ws://127.0.0.1:8080", display3d);

let callbacks: GuiCallbacks = {
  paused(value) {
    if (value) {
      comms.send("pause");
    } else {
      world.plushie?.clearLinks();
      comms.send("resume");
    }
  },
  step() {
    world.plushie?.clearLinks();
    comms.send("advance");
  },
  showEdges(val) {
    val ? world.plushie?.drawLinks() : world.plushie?.clearLinks();
  },
};

setupGui(display3d, callbacks);
