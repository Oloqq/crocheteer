import * as create from "./create";

export default class Plushie {
  constructor() {
    this.pos = [1, 0, 1];
    this.s1 = create.sphere([0, 0, 0], 0.5);
  }
}