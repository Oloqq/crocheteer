import { Display } from "./render3d";
import * as create from "./utils/create";

export default class PlushieBody {
  nodes: any[];

  constructor(display: Display, data: crapi.Initialize) {
    const points = data.nodes.points;
    // this.nodePoints = [];
    this.nodes = [];
    for (let [i, point] of points.entries()) {
      let sph = create.sphere(display.scene, point, 0.1, this.nodeColor(i));
      this.nodes.push(sph);
    }
  }

  // FIXME
  nodeColor(_: any): any {
    return 0x00ff00;
  }
}
