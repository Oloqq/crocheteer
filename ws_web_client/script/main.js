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
const loadExampleButton = document.getElementById("example-btn");
const loadExampleText = document.getElementById("example-name");
const status = document.getElementById("status");

function sendPattern() {
  let text = pattern.value;
  status.innerText = "sending...";
  simulator.send(`pattern ${text}`);
}

updateButton.addEventListener("click", sendPattern);
loadExampleButton.addEventListener("click", () => {
  let text = loadExampleText.value;
  console.log(text);
  status.innerText = "asking for example...";
  simulator.send(`load_example ${text}`);
});

function main() {
  const app = simulator.init();
  const gui = app.gui;

  const simulationWorld = new Plushie(status, customGui, gui, pattern);

  gui.add(customGui, 'advance').name("Advance 1 step");
  gui.add(customGui, 'edgesVisible').name("Display edges (expensive)").onChange((_value) => {
    simulationWorld.toggleLinks();
  });
  gui.add(customGui, 'gravity').name("Gravity").onChange((value) => {
    simulator.send(`gravity ${value}`)
  });
  gui.add(customGui, 'stuffing', { None: 'None', Centroids: 'Centroids' }).onChange((val) => {
    simulator.send(`stuffing ${val}`);
    simulator.send(`centroid.amount ${customGui.centroids.amount}`);
  });

  var _ = gui.addFolder('PerRound stuffing config');

  var centroids = gui.addFolder('Centroid stuffing config');
  let amount = centroids.add(customGui.centroids, "amount", 1, 20, 1).onChange((val) => {
    simulator.send(`centroid.amount ${val}`);
    simulationWorld.setCentroidNum(val);
  })
  amount.domElement.children[1].style.pointerEvents = "none";
  centroids.open();

  simulator.connect("ws://127.0.0.1:8080", simulationWorld);
  status.innerText = "Waiting for websocket connection...";
  initialSendPattern();
}

function initialSendPattern() {
  if (simulator.isWebSocketOpened()) {
    sendPattern();
  } else {
    setTimeout(initialSendPattern, 1000);
  }
}

main();