import "./style.css";
import { GuiCallbacks, setupGui } from "./gui.ts";
import * as render from "./render3d.ts";
import * as comms from "./comms";
import World from "./World.ts";

const display3d = render.init();
let world = new World("ws://127.0.0.1:8080", display3d);

let callbacks: GuiCallbacks = {
  paused(value) {
    value ? comms.send("pause") : comms.send("resume");
  },
  step() {
    comms.send("advance");
  },
  showEdges(_val) {},
};

setupGui(display3d, callbacks);
