import Plushie from './plushie';
import "./lib/interaction";

import * as simulator from "./lib/simulation";
import Simulation from './simulation';

let world = undefined;

const customGui = {
  edgesVisible: true,
  secondaryVisible: true,
  advance: function () {
    world.onAdvance();
    simulator.send("advance");
  },
  animate: function () {
    world.onAdvance();
    simulator.send("animate");
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
const exportButton = document.getElementById("export-btn");
const importButton = document.getElementById("import-btn");
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

exportButton.addEventListener("click", () => {
  simulator.send(`export-pointcloud`);
});

importButton.addEventListener("click", () => {
  var input = document.createElement('input');
  input.type = 'file';

  input.onchange = e => {

    // getting a hold of the file reference
    var file = e.target.files[0];

    // setting up the reader
    var reader = new FileReader();
    reader.readAsText(file, 'UTF-8');

    // here we tell the reader what to do when it's done reading...
    reader.onload = readerEvent => {
      var content = readerEvent.target.result; // this is the content!
      simulator.send(`import-pointcloud ${content}`);
    }

  }

  input.click();
  // simulator.send(`export`);
});

function initParamsGui(gui, world) {
  const sendParams = () => {
    console.log("sending params: ", world.params);
    simulator.send(`setparams ${JSON.stringify(world.params)}`);
  }
  const removeSlider = (numberInput) => {
    numberInput.domElement.children[1].style.pointerEvents = "none";
  }
  const p = world.params;
  const folder = gui.addFolder("Params");
  folder.open();

  folder.add(p, "desired_stitch_distance").name("DSD").onChange(sendParams);
  folder.add(p, "floor").name("Floored").onChange(sendParams);
  folder.add(p, "keep_root_at_origin").name("Rooted").onChange(sendParams);
  folder.add(p, "single_loop_force", 0).name("SLF").onChange(sendParams);
  // Reversing time does not work in this simulation
  folder.add(p, "timestep", 0.1, 1.7).name("Timestep").onChange(sendParams);

  var centroids = folder.addFolder("Centroid stuffing");
  centroids.open();
  {
    centroids.add(p.centroids, "force", 0).name("Force").onChange(sendParams);
    centroids.add(p.centroids, "min_nodes_per_centroid", 0).name("Nodes per centroid").onChange(sendParams);
    removeSlider(centroids.add(p.centroids, "number", 0, 20, 1).onChange((val) => {
      world.setCentroidNum(val);
      sendParams();
    }));

  }
}

function main() {
  const app = simulator.init();
  const gui = app.gui;
  app.status = status;

  const mainPlushie = new Plushie(status, gui, pattern, true);
  const plushie2 = new Plushie(status, gui, pattern, false);
  const simulation = new Simulation(status, gui, pattern, mainPlushie, [plushie2]);
  world = simulation;

  gui.add(customGui, 'advance').name("Advance 1 step");
  gui.add(customGui, 'animate').name("Animate");
  gui.add(customGui, 'edgesVisible').name("Display edges (expensive)").onChange((_value) => {
    mainPlushie.toggleLinks();
  });
  gui.add(customGui, 'secondaryVisible').name("Show secondary").onChange((_value) => {
    plushie2.toggleVisibility()
  });

  initParamsGui(gui, world);

  status.innerText = "Waiting for websocket connection...";
  simulator.connect("ws://127.0.0.1:8080", simulation);

  if (sendPatternAtStart) {
    initialSendPattern();
  } else {
    initialGetParams();
  }
}

function initialSendPattern() {
  if (simulator.isWebSocketOpened()) {
    sendPattern();
    simulator.send(`getparams`);
  } else {
    setTimeout(initialSendPattern, 1000);
  }
}

function initialGetParams() {
  if (simulator.isWebSocketOpened()) {
    simulator.send(`getparams`);
  } else {
    setTimeout(initialGetParams, 100);
  }
}

main();