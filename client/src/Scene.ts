import { connect } from "./comms";
import PlushieRenderer from "./PlushieRenderer";

export default class Scene {
  ws: WebSocket;
  plushie: PlushieRenderer;
  // ghosts: GhostRenderer[];

  constructor(url: string, plushie: PlushieRenderer) {
    this.ws = connect(url, this);
    this.plushie = plushie;
  }

  parseMessage(key: string, data: any) {
    console.log(key, data);
    switch (key) {
      case "ini":
        // this.plushie.init(data);
        break;
      // case "upd":
      //   // this.plushie.init(data);
      //   break;
      default:
        console.error("unhandled message", key);
    }
  }
}
