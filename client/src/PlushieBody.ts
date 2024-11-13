import { Mesh } from "three";
import { Display } from "./render3d";
import * as create from "./utils/create";
import * as THREE from "three";
import { GuiData } from "./gui";

export default class PlushieBody {
  scene: THREE.Scene;
  nodes: Mesh[];
  links: Mesh[] = [];
  edges: number[][];
  nodeColors: crapi.RGB[];
  peculiarities: crapi.Peculiarities;
  displayMode: "normal" | "debug" = "normal";

  constructor(display: Display, data: crapi.Initialize, guiData: GuiData) {
    this.scene = display.scene;
    this.nodeColors = data.nodes.colors;
    this.peculiarities = data.nodes.peculiarities;
    this.nodes = [];
    for (let [i, point] of data.nodes.points.entries()) {
      let newSphere = create.sphere(this.scene, point, 0.1, this.nodeColor(i));
      this.nodes.push(newSphere);
    }
    this.edges = data.edges;
    if (guiData.showEdges) {
      this.drawLinks();
    }
  }

  destroy() {
    for (let sph of this.nodes) {
      if (sph.geometry) sph.geometry.dispose();
      if (sph.material) (sph.material as THREE.Material).dispose();
      this.scene.remove(sph);
    }
    this.clearLinks();
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

  drawLinks() {
    const width = 0.05;

    for (let from = 0; from < this.edges.length; from++) {
      for (let to of this.edges[from]) {
        if (from >= this.nodes.length || to >= this.nodes.length) {
          continue;
        }
        const color = this.linkColor(from, to);
        let link = create.link(
          this.scene,
          this.nodes[from].position,
          this.nodes[to].position,
          width,
          color
        );
        this.links.push(link);
      }
    }
  }

  clearLinks() {
    for (let link of this.links) {
      if (link.geometry) link.geometry.dispose();
      if (link.material) {
        // hack: link.material is (Material | Material[]). it's always Material here
        const x: any = link.material;
        x.dispose();
      }
      this.scene.remove(link);
    }
    this.links = [];
  }

  nodeColor(index: number): crapi.RGB {
    return this.nodeColors[index];
  }

  linkColor(_from: any, to: any) {
    if (this.displayMode == "debug") {
      return 0x343330;
    }

    let [r, g, b] = this.nodeColors[to];
    return `rgb(${r}, ${g}, ${b})`;
  }
}
