import { Mesh } from "three";
import { Display } from "./render3d";
import * as create from "./utils/create";

export default class PlushieBody {
  nodes: Mesh[];

  constructor(display: Display, data: crapi.Initialize) {
    this.nodes = [];
    for (let [i, point] of data.nodes.points.entries()) {
      let newSphere = create.sphere(
        display.scene,
        point,
        0.1,
        this.nodeColor(i)
      );
      this.nodes.push(newSphere);
    }
  }

  update(data: crapi.Update) {
    // const centroids = data.centroids.centroids;
    this.updateStitches(data.points);
  }

  updateStitches(stitchPositions: crapi.Point[]) {
    if (stitchPositions.length < this.nodes.length) {
      console.error("Position data got corrupted");
    }

    for (let i = 0; i < stitchPositions.length; i++) {
      this.nodes[i].position.x = stitchPositions[i][0];
      this.nodes[i].position.y = stitchPositions[i][1];
      this.nodes[i].position.z = stitchPositions[i][2];
    }
  }

  // FIXME
  nodeColor(_: any): any {
    return 0x00ff00;
  }
}
