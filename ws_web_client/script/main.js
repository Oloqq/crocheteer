import Plushie from './plushie';
import "./lib/interaction";

import * as simulator from "./lib/simulation";

function advance() {
  simulator.send("advance");
}

function main() {
  const app = simulator.init();
  const gui = app.gui;

  gui.add({ advance }, 'advance').name("Advance 1 step");

  const simulationWorld = new Plushie();
  simulator.connect("ws://127.0.0.1:8080", simulationWorld);
}

main();