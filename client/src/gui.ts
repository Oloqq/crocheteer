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

  newskelet = () => {
    comms.send(`newskelet`);
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

  const sendParams = () => {
    comms.send(`setparams ${JSON.stringify(data.params)}`);
  };
  const params = gui.addFolder("Params");
  {
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
    const skeleton = params.addFolder("Skeletonization");
    {
      skeleton
        .add(data.params.skelet_stuffing, "enable")
        .name("Skeleton stuffing")
        .onChange(sendParams);
      skeleton
        .add(data.params.skelet_stuffing, "autoskelet")
        .onChange(sendParams);
      skeleton
        .add(data.params.skelet_stuffing, "interval", 1, 100, 1)
        .onChange(sendParams);

      skeleton.add(data, "newskelet").name("Recalc skelet");
      skeleton
        .add(data.params.skelet_stuffing, "cluster_number", 1, 200, 1)
        .onChange(sendParams);
      skeleton
        .add(data.params.skelet_stuffing, "must_include_points", 0, 1)
        .onChange(sendParams);
      skeleton
        .add(data.params.skelet_stuffing, "allowed_overlap", 0, 10)
        .onChange(sendParams);
    }
  }

  const experimental = gui.addFolder("Experimental");
  {
    const experimentalData = {
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
      getperfs: () => {
        comms.send(`getperf`);
      },
    };

    experimental
      .add(experimentalData, "calculateNormals")
      .name("Calculate normals (takes time)");

    experimental.add(experimentalData, "clustering").name("Perform kmeans");

    experimental
      .add(experimentalData, "initialCrossSections")
      .name("Initial cross sections (takes time)");

    experimental
      .add(data, "inspectCluster", 0)
      .onChange((val) => data.clusterChanged(val));

    experimental.add(experimentalData, "growing").name("Growing");
    experimental.add(experimentalData, "cost").name("Calculate cost");
    experimental.add(experimentalData, "optimparts").name("Select parts");

    experimental
      .add(data.params, "track_performance")
      .name("Track performance")
      .onChange(sendParams);
    experimental.add(experimentalData, "getperfs").name("Get performance");
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
  single_loop_force: 0.1,
  timestep: 1.1,
  minimum_displacement: 0.001,
  initializer: "Cylinder",
  track_performance: false,
  skelet_stuffing: {
    enable: false,
    autoskelet: true,
    interval: 50,
    cluster_number: 50,
    must_include_points: 0.95,
    allowed_overlap: 5.0,
  },
};
