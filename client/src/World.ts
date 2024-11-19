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

  constructor(url: string, display: Display, guiData: GuiData, onconnect: any) {
    this.ws = connect(url, this, onconnect);
    this.display = display;
    this.guiData = guiData;
  }

  parseMessage(key: string, data: any) {
    switch (key) {
      case "initialize":
        const dataInit = data as crapi.Initialize;
        this.plushie?.destroy();
        this.plushie = new PlushieBody(this.display, dataInit, this.guiData);
        const centroidNum = dataInit.centroids.points.length;
        this.guiData.params.centroids.number = centroidNum;
        this.guiData.updateDisplay();
        break;
      case "update":
        this.plushie ?? panic("Plushie is not initialied");
        this.plushie!.update(data);
        break;
      case "params":
        const RECURSIVE = true;
        jq.extend(RECURSIVE, this.guiData.params, JSON.parse(data));
        this.guiData.updateDisplay();
        break;
      case "status":
        console.info(`status: ${data}`);
        this.guiData.statusMessage = data;
        const statusButton = document.getElementById("status-box")!;
        if (this.guiData.statusMessage.startsWith("Couldn't parse")) {
          statusButton.innerText = "Error";
          statusButton.classList.add("box-highlight");
        } else {
          statusButton.innerText = "OK";
          statusButton.classList.remove("box-highlight");
        }
        break;
      case "export":
        download(data, "plushie.json", "json"); // TODO pcd
        break;
      default:
        console.error("unhandled message", key);
    }
  }
}

function panic(msg: string): void {
  throw msg;
}
