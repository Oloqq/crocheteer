import Plushie from './plushie';
import "./lib/interaction";

import * as simulator from "./lib/simulation";

let world = undefined;
const customGui = {
  edgesVisible: true,
  gravity: 5e-4,
  advance: function () {
    world.onAdvance();
    simulator.send("advance");
  },
  stuffing: 'PerRound',
  centroids: {
    amount: 1
  }
}

const pattern = document.getElementById("pattern");
const updateButton = document.getElementById("update-btn");
const softUpdateButton = document.getElementById("soft-update-btn");
const loadExampleButton = document.getElementById("example-btn");
const loadExampleText = document.getElementById("example-txt");
const status = document.getElementById("status");
const commandButton = document.getElementById("command-btn");
const commandText = document.getElementById("command-txt");
const plushieVersion = "flow";
const sendPatternAtStart = false;

function sendPattern() {
  let text = pattern.value;
  status.innerText = "sending pattern...";
  simulator.send(`pattern ${plushieVersion} ${text}`);
}

function softUpdate() {
  let text = pattern.value;
  status.innerText = "sending pattern (soft update)...";
  simulator.send(`soft_update ${text}`);
}

updateButton.addEventListener("click", sendPattern);
softUpdateButton.addEventListener("click", softUpdate);
loadExampleButton.addEventListener("click", () => {
  let text = loadExampleText.value;
  status.innerText = "asking for example...";
  simulator.send(`load_example ${text}`);
});

commandButton.addEventListener("click", () => {
  let text = commandText.value;
  status.innerText = `sending command... ${text}`;
  simulator.send(`${text}`);
});

function main() {
  const app = simulator.init();
  const gui = app.gui;

  const simulationWorld = new Plushie(status, customGui, gui, pattern);
  world = simulationWorld;

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
  let amount = centroids.add(customGui.centroids, "amount", 0, 20, 1).onChange((val) => {
    simulator.send(`centroid.amount ${val}`);
    simulationWorld.setCentroidNum(val);
  })
  amount.domElement.children[1].style.pointerEvents = "none";
  centroids.open();

  simulator.connect("ws://127.0.0.1:8080", simulationWorld);
  status.innerText = "Waiting for websocket connection...";

  if (sendPatternAtStart) {
    initialSendPattern();
  }
}

function initialSendPattern() {
  if (simulator.isWebSocketOpened()) {
    sendPattern();
  } else {
    setTimeout(initialSendPattern, 1000);
  }
}

main();