import Plushie from './plushie';
import "./lib/interaction";

import * as simulator from "./lib/simulation";

const customGui = {
  edgesVisible: true,
  gravity: 5e-4,
  advance: function () {
    simulator.send("advance");
  },
  stuffing: 'PerRound',
  centroids: {
    amount: 1
  }
}

const pattern = document.getElementById("pattern");
const updateButton = document.getElementById("update");
const status = document.getElementById("status");

updateButton.addEventListener("click", () => {
  let text = pattern.value;
  status.innerText = "sending...";
  simulator.send(`pattern ${text}`);
});

function main() {
  const app = simulator.init();
  const gui = app.gui;

  const simulationWorld = new Plushie(status, customGui);

  gui.add(customGui, 'advance').name("Advance 1 step");
  gui.add(customGui, 'edgesVisible').name("Display edges (expensive)").onChange((_value) => {
    simulationWorld.toggleLinks();
  });
  gui.add(customGui, 'gravity').name("Gravity").onChange((value) => {
    simulator.send(`gravity ${value}`)
  });
  gui.add(customGui, 'stuffing', { None: 'None', PerRound: 'PerRound', Centroids: 'Centroids' }).onChange((val) => {
    simulator.send(`stuffing ${val}`);
    simulator.send(`centroid.amount ${customGui.centroids.amount}`);
  }).listen();

  var _ = gui.addFolder('PerRound stuffing config');

  var centroids = gui.addFolder('Centroid stuffing config');
  centroids.add(customGui.centroids, "amount", 1, 20, 1).onChange((val) => {
    simulator.send(`centroid.amount ${val}`);
    simulationWorld.setCentroidNum(val);
  })
  centroids.open();

  simulator.connect("ws://127.0.0.1:8080", simulationWorld);
}

main();