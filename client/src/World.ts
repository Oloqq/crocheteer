import { connect } from "./comms";
import { GuiData } from "./gui";
import PlushieBody from "./PlushieBody";
import { Display } from "./render3d";

export default class World {
  ws: WebSocket;
  display: Display;
  plushie: PlushieBody | undefined;
  readonly guiData: GuiData;
  // ghosts: GhostRenderer[];

  constructor(url: string, display: Display, guiData: GuiData) {
    this.ws = connect(url, this);
    this.display = display;
    this.guiData = guiData;
  }

  parseMessage(key: string, data: any) {
    console.log(key, data);
    switch (key) {
      case "ini":
        this.plushie = new PlushieBody(this.display, data, this.guiData);
        break;
      case "upd":
        this.plushie ?? panic("Plushie is not initialied");
        this.plushie!.update(data);
        break;
      default:
        console.error("unhandled message", key);
    }
  }
}

function panic(msg: string) {
  throw msg;
}
