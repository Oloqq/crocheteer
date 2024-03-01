import * as create from "./lib/create";
import * as simulation from "./lib/simulation";

export default class Plushie {
  constructor(status) {
    this.edges = [];
    this.stitchSpheres = [];
    this.stitchPositions = [];
    this.links = [];
    this.dragged = null;
    this.displayEdges = true;
    this.status = status;
  }

  getId(obj) {
    return this.stitchSpheres.findIndex((o) => o == obj);
  }

  drag(obj) {
    let id = this.getId(obj);
    this.dragged = obj;
    simulation.send(`pos ${id} ${obj.position.x} ${obj.position.y} ${obj.position.z}`);
  }

  mouseMove(obj) {
    this.drag(obj);
  }

  mouseUp(obj) {
    this.drag(obj);
  }

  toggleLinks() {
    this.displayEdges = !this.displayEdges;
    this.clearLinks();
    if (this.displayEdges)
      this.drawLinks();
  }

  clearLinks() {
    for (let link of this.links) {
      if (link.geometry) link.geometry.dispose();
      if (link.material) link.material.dispose();
      create.scene.remove(link);
    }
    this.links = [];
  }

  drawLinks() {
    for (let from = 0; from < this.edges.length; from++) {
      for (let to of this.edges[from]) {
        let link = create.link(this.stitchPositions[from], this.stitchPositions[to], 0.02, "red")
        this.links.push(link);
      }
    }
  }

  updateLinks() {
    if (!this.displayEdges)
      return;

    this.clearLinks();
    this.drawLinks();
  }

  parseMessage(key, data) {
    switch (key) {
      case "upd":
        this.update(data);
        break;
      case "ini":
        this.init(data);
        break;
      case "status":
        console.log("response", data);
        this.status.innerText = data;
        break;
      default:
        console.error(`Unrecognized key: ${dict["key"]}`);
    }
  }

  init(data) {
    const points = data["points"];
    this.edges = data["edges"];

    this.stitchPositions = [];
    for (let sph of this.stitchSpheres) {
      create.scene.remove(sph);
    }
    this.clearLinks();
    this.stitchSpheres = [];

    for (let point of points) {
      let sph = create.sphere(point, 0.1);
      this.stitchPositions.push(sph.position);
      this.stitchSpheres.push(sph);
    }
    for (let from = 0; from < this.edges.length; from++) {
      for (let to of this.edges[from]) {
        let link = create.link(this.stitchPositions[from], this.stitchPositions[to], 0.02, "red")
        this.links.push(link);
      }
    }
  }

  update(data) {
    if (data.length != this.stitchSpheres.length) {
      throw "WHYYYYY";
    }

    let id = undefined;
    let save = undefined;
    if (this.dragged) {
      id = this.getId(this.dragged);
      save = {};
      save.x = this.dragged.position.x;
      save.y = this.dragged.position.y;
      save.z = this.dragged.position.z;
    }

    for (let i = 0; i < data.length; i++) {
      this.stitchPositions[i].x = data[i][0]
      this.stitchPositions[i].y = data[i][1]
      this.stitchPositions[i].z = data[i][2]
    }

    if (this.dragged) {
      this.dragged.position.x = save.x;
      this.dragged.position.y = save.y;
      this.dragged.position.z = save.z;
      this.dragged = null;
    }

    // this.updateLinks();
  }
}
