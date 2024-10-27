import { connect } from "./comms";
import PlushieBody from "./PlushieBody";
import { Display } from "./render3d";

export default class World {
  ws: WebSocket;
  display: Display;
  plushie: PlushieBody | undefined;
  // ghosts: GhostRenderer[];

  constructor(url: string, display: Display) {
    this.ws = connect(url, this);
    this.display = display;
  }

  parseMessage(key: string, data: any) {
    console.log(key, data);
    switch (key) {
      case "ini":
        this.plushie = new PlushieBody(this.display, data);
        break;
      // case "upd":
      //   // this.plushie.init(data);
      //   break;
      default:
        console.error("unhandled message", key);
    }
  }
}
