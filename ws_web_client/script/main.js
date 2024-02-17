import Plushie from './plushie';
import Ball from './ball';
import "./lib/interaction";

import * as simulator from "./lib/simulation";

function main() {
  simulator.init();
  const simulationWorld = new Ball();
  simulator.connect("ws://127.0.0.1:8080", simulationWorld);
}

main();