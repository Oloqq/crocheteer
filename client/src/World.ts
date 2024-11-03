import { connect } from "./comms";
import { GuiData } from "./gui";
import PlushieBody from "./PlushieBody";
import { Display } from "./render3d";
import jq from "jquery";

export default class World {
  ws: WebSocket;
  display: Display;
  plushie: PlushieBody | undefined;
  guiData: GuiData;
  // ghosts: GhostRenderer[];

  constructor(url: string, display: Display, guiData: GuiData) {
    this.ws = connect(url, this);
    this.display = display;
    this.guiData = guiData;
  }

  parseMessage(key: string, data: any) {
    // FIXME use more descriptive names
    switch (key) {
      case "ini":
        this.plushie = new PlushieBody(this.display, data, this.guiData);
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
      default:
        console.error("unhandled message", key);
    }
  }
}

function panic(msg: string): void {
  throw msg;
}
