import { connect } from "./comms";
import { GuiData } from "./gui";
import PlushieBody from "./PlushieBody";
import { Display } from "./render3d";
import jq from "jquery";
import { download } from "./utils/download";

// TEMP for skeletonization research
import * as create from "./utils/create";

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
      case "normals":
        this.plushie?.displayNormals(JSON.parse(data));
        break;
      // Skeletonization research
      case "change-colors":
        if (this.plushie) {
          const d = JSON.parse(data) as crapi.ChangeColors;
          this.plushie.clearLinks();
          this.plushie.restoreColors = this.plushie?.nodeColors;
          this.plushie.variableNodeColors = d.variable;
          this.plushie.updateColors(d.standard);
        }
        break;
      case "change-centroids":
        if (this.plushie) {
          const d = JSON.parse(data);
          const centroids = d["centroids"];
          const colors = d["colors"];
          while (this.plushie.centroids.length > 0) {
            create.destroy(this.plushie.scene, this.plushie.centroids.pop()!);
          }
          this.plushie.centroidColors = colors;
          if (d["angles"]) this.plushie.centroidAngles = d["angles"];
          this.plushie.updateCentroids(centroids);
        }
        break;
      default:
        console.error("unhandled message", key);
    }
  }
}

function panic(msg: string): void {
  throw msg;
}
