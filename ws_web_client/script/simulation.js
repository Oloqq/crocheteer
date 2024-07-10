export default class Simulation {
  constructor(status, gui, pattern, mainPlushie, plushies) {
    this.status = status;
    this.gui = gui;
    this.pattern = pattern;
    // those will be immediately overwritten with serialized params from the server
    // but they have to be initialized here so gui can be constructed
    this.params = {
      "centroids": {
        "force": 0.1,
        "min_nodes_per_centroid": 0,
        "number": 0,
      },
      "desired_stitch_distance": 0.1, // decimal point here enables displaying decimal points later
      "floor": false,
      "gravity": 0.1,
      "keep_root_at_origin": false,
      "single_loop_force": 0.1,
      "timestep": 1.1,
    };
    this.mainPlushie = mainPlushie;
    this.plushies = plushies;
  }

  parseMessage(key, data) {
    switch (key) {
      case "upd":
        this.mainPlushie.update(data);
        break;
      case "ini":
        this.mainPlushie.init(data);
        break;
      case "ini2":
        if (data.length != 2) {
          this.status.innerText = "error 3423";
          console.error(`tried to initialize ${data.length} plushies`);
        }
        this.mainPlushie.init(data[0]);
        this.plushies[0].init(data[1]);
        break;
      case "multiini":
        break;
      case "status":
        console.log("response", data);
        this.status.innerText = data;
        break;
      case "pattern_update":
        console.log("new pattern");
        this.pattern.value = data;
        break;
      case "params":
        const RECURSIVE = true;
        jQuery.extend(RECURSIVE, this.params, JSON.parse(data))
        console.log("got params: ", this.params);
        this.gui.updateDisplay();
        break;
      default:
        console.error(`Unrecognized key: ${key}`);
    }
  }

  onAdvance() {
    this.mainPlushie.onAdvance();
  }

  onResume() {
    this.mainPlushie.clearLinks();
  }

  onPause() {
    this.mainPlushie.updateLinks();
  }
}