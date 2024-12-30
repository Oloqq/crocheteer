import * as dat from "dat.gui";
import { Display, restoreDefaultView } from "./render3d";
import * as comms from "./comms";
import World from "./World";
import { setupTooltip } from "./utils/tooltip";

export class GuiData {
  world?: World = undefined;
  updateDisplay: () => void = () => {
    console.error("display wasn't ready for update");
  };

  paused: boolean = true;
  showEdges: boolean = true;
  getPattern: () => string;
  setPattern: (pattern: string) => void;
  params: crapi.Params = paramsThatInitializeDatGuiWithCorrectTypes;
  statusMessage: string = "Status messages will be displayed here";

  constructor(getPattern: () => string, setPattern: (pattern: string) => void) {
    this.getPattern = getPattern;
    this.setPattern = setPattern;
  }

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

  // Skeleton research
  inspectCluster: number = 0;
  clusterChanged: (val: number) => void = (_) => {};
}

export function initGui(
  display3d: Display,
  data: GuiData,
  world: World
): dat.GUI {
  const gui = new dat.GUI({ hideable: false });
  data.world = world;
  data.updateDisplay = () => {
    gui.updateDisplay();
  };

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

  const params = gui.addFolder("Params");
  {
    const sendParams = () => {
      comms.send(`setparams ${JSON.stringify(data.params)}`);
    };

    params.open();
    const centroids = params.addFolder("Centroid stuffing");
    centroids.open();
    {
      centroids
        .add(data.params.centroids, "force", 0)
        .name("Force")
        .onChange(sendParams);
      centroids
        .add(data.params.centroids, "min_nodes_per_centroid", 0)
        .name("Nodes per centroid")
        .onChange(sendParams);
      centroids
        .add(data.params.centroids, "number", 0, 20, 1)
        .onChange((_val) => {
          sendParams();
        });
    }

    setupTooltip(
      params
        .add(data.params, "desired_stitch_distance")
        .name("DSD")
        .onChange(sendParams).domElement,
      () => "Desired stitch distance"
    );

    params.add(data.params, "floor").name("Floored").onChange(sendParams);
    params
      .add(data.params, "keep_root_at_origin")
      .name("Rooted")
      .onChange(sendParams);
    setupTooltip(
      params
        .add(data.params, "single_loop_force", 0)
        .name("SLF")
        .onChange(sendParams).domElement,
      () => "Single loop force"
    );
    // Reversing time does not work in this simulation
    params
      .add(data.params, "timestep", 0.1, 1.7)
      .name("Timestep")
      .onChange(sendParams);
    params
      .add(data.params, "minimum_displacement")
      .name("Min displacement")
      .onChange(sendParams);
  }

  const skeleton = gui.addFolder("Skeletonization");
  const skeletonExperiments = {
    calculateNormals: () => {
      comms.send(`calculate-normals`);
    },
    clustering: () => {
      comms.send(`do-clustering`);
    },
    initialCrossSections: () => {
      comms.send(`initial-cross-sections`);
    },
    growing: () => {
      comms.send(`growing`);
    },
    cost: () => {
      comms.send(`cost`);
    },
    optimparts: () => {
      comms.send(`optimparts`);
    },
  };

  const skeletonData = {
    enable: false,
    centroid_number: 50,
    must_include_points: 0.95,
    allowed_overlap: 5.0,
    newskelet: () => {
      comms.send(`newskelet`);
    },
  };
  const sendSkelet = () => {
    comms.send(`skeletparams ${JSON.stringify(skeletonData)}`);
  };

  skeleton.open();
  {
    const experiment = skeleton.addFolder("Experiment");
    // experiment.open();
    {
      experiment
        .add(skeletonExperiments, "calculateNormals")
        .name("Calculate normals (takes time)");

      experiment.add(skeletonExperiments, "clustering").name("Perform kmeans");

      experiment
        .add(skeletonExperiments, "initialCrossSections")
        .name("Initial cross sections (takes time)");

      experiment
        .add(data, "inspectCluster", 0)
        .onChange((val) => data.clusterChanged(val));

      experiment.add(skeletonExperiments, "growing").name("Growing");
      experiment.add(skeletonExperiments, "cost").name("Calculate cost");
      experiment.add(skeletonExperiments, "optimparts").name("Select parts");
    }

    skeleton
      .add(skeletonData, "enable")
      .name("Skeleton stuffing")
      .onChange(sendSkelet);

    skeleton.add(skeletonData, "newskelet").name("New skelet");
    skeleton
      .add(skeletonData, "centroid_number", 1, 200, 1)
      .onChange(sendSkelet);
    skeleton
      .add(skeletonData, "must_include_points", 0, 1)
      .onChange(sendSkelet);
    skeleton.add(skeletonData, "allowed_overlap", 0, 10).onChange(sendSkelet);
  }

  return gui;
}

const paramsThatInitializeDatGuiWithCorrectTypes: crapi.Params = {
  centroids: {
    force: 0.1,
    min_nodes_per_centroid: 0,
    number: 0,
  },
  desired_stitch_distance: 0.1, // decimal point here enables displaying decimal points later
  floor: false,
  gravity: 0.1,
  keep_root_at_origin: false,
  single_loop_force: 0.1,
  timestep: 1.1,
  minimum_displacement: 0.001,
  initializer: "Cylinder",
  hook_leniency: "NoMercy",
  autostop: {
    max_relaxing_iterations: 50,
    acceptable_tension: 0.1,
  },
};
