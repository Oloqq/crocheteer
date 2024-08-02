import * as create from "./lib/create";

export default class Ball {
  constructor() {
    this.object = create.sphere([0, 0, 0], 0.5);
  }

  parse(data) {
    this.object.position.x = data[0];
    this.object.position.y = data[1];
    this.object.position.z = data[2];
  }
}