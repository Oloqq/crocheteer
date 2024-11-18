import { connect } from "./comms";
import { GuiData } from "./gui";
import PlushieBody from "./PlushieBody";
import { Display } from "./render3d";
import jq from "jquery";
import { download } from "./utils/download";

export default class World {
  ws: WebSocket;
  display: Display;
  plushie: PlushieBody | undefined;
  guiData: GuiData;
  // ghosts: GhostRenderer[];

  constructor(url: string, display: Display, guiData: GuiData, onconnect: any) {
    this.ws = connect(url, this, onconnect);
    this.display = display;
    this.guiData = guiData;
  }

  parseMessage(key: string, data: any) {
    // FIXME use more descriptive names
    switch (key) {
      case "ini": // also handle ini2
        this.plushie?.destroy();
        this.plushie = new PlushieBody(this.display, data, this.guiData);
        const centroidNum = data["centroids"]["centroids"].length;
        this.guiData.params.centroids.number = centroidNum;
        this.guiData.updateDisplay();
        break;
      case "upd":
        this.plushie ?? panic("Plushie is not initialied");
        this.plushie!.update(data);
        break;
      case "params":
        const RECURSIVE = true;
        jq.extend(RECURSIVE, this.guiData.params, JSON.parse(data)); // FIXME shouldn't need json parse here
        this.guiData.updateDisplay();
        break;
      case "status":
        console.info(`status: ${data}`);
        break;
      case "pattern_update": // TODO test
        this.guiData.setPattern(data);
        break;
      case "export": // TODO test
        download(data, "plushie.json", "json");
        break;
      // "relax ended" can be removed on the server side
      default:
        console.error("unhandled message", key);
    }
  }
}

function panic(msg: string): void {
  throw msg;
}
