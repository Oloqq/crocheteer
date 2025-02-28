import { Mesh } from "three";
import { Display } from "./render3d";
import * as create from "./utils/create";
import * as THREE from "three";
import { GuiData } from "./gui";

// const peculiarityColors = {
//   "normal": 0x00ff00,
//   "centroid": 0xffa500,
//   "Tip": 0x0000ff,
//   "Root": 0x0000ff,
// }

const normalsColor = new THREE.Color(1, 0, 0);
const planeColor = new THREE.Color(1, 1, 1);
const planeOpacity = 0.5;

export default class PlushieBody {
  scene: THREE.Scene;
  nodes: Mesh[];
  centroids: Mesh[];
  links: Mesh[] = [];
  edges: number[][];
  nodeColors: crapi.RGB[];
  peculiarities: crapi.Peculiarities;
  displayMode: "normal" | "debug" = "normal";
  // Skeletonization research
  disposeAtStep: THREE.Group[] = [];
  restoreColors: crapi.RGB[] | undefined;
  centroidColors: crapi.RGB[] | undefined;
  planeDisk: Mesh | undefined;
  planeArrow: THREE.Group | undefined;
  centroidAngles: [number, number][] | undefined;
  variableNodeColors: crapi.RGB[][] | undefined;
  partToCentroids: number[][] | undefined;

  constructor(display: Display, data: crapi.Initialize, guiData: GuiData) {
    this.scene = display.scene;
    this.nodeColors = data.nodes.colors;
    this.peculiarities = data.nodes.peculiarities;
    this.nodes = [];
    for (let [i, point] of data.nodes.points.entries()) {
      let [radius, color] = this.nodeDisplay(i);
      let newSphere = create.sphere(this.scene, point, radius, color);
      this.nodes.push(newSphere);
    }

    this.centroids = [];
    for (let i in data.centroids.points) {
      let centroidPoint = data.centroids.points[i];
      this.centroids.push(
        create.sphere(
          this.scene,
          centroidPoint,
          0.3,
          this.centroidColor(parseInt(i))
        )
      );
    }

    this.edges = data.edges;
    if (guiData.showEdges) {
      this.drawLinks();
    }

    guiData.clusterChanged = (val) => {
      if (!this.centroidColors) {
        console.log(
          "inspecting clusters only work after they have been calculated"
        );
        return;
      }

      if (this.planeDisk) {
        create.destroy(this.scene, this.planeDisk);
      }
      if (this.planeArrow) {
        create.destroyGroup(this.scene, this.planeArrow);
      }

      if (val < 0 || val >= this.centroids.length) {
        console.log("tried to inspect nonexisting centroid");
        return;
      }
      const centroidPos = this.centroids[val].position;
      if (this.centroidAngles != undefined) {
        const angles = this.centroidAngles[val];
        this.planeDisk = create.disk(
          this.scene,
          centroidPos.x,
          centroidPos.y,
          centroidPos.z,
          angles[0],
          angles[1],
          1,
          planeColor,
          planeOpacity
        );

        this.planeArrow = create.arrowFromAngles(
          this.scene,
          centroidPos,
          angles[0],
          angles[1],
          1,
          planeColor
        );
      }

      if (this.variableNodeColors != undefined) {
        if (val < this.variableNodeColors.length) {
          this.updateColors(this.variableNodeColors![val]);
        }
      }

      if (this.partToCentroids != undefined) {
        if (val < this.partToCentroids.length) {
          for (let i = 0; i < this.centroids.length; i++) {
            this.centroids[i].visible = false;
          }
          for (let i of this.partToCentroids[val]) {
            this.centroids[i].visible = true;
          }
        }
      }
    };
  }

  destroy() {
    for (let sph of this.nodes) {
      if (sph.geometry) sph.geometry.dispose();
      if (sph.material) (sph.material as THREE.Material).dispose();
      this.scene.remove(sph);
    }
    for (let sph of this.centroids) {
      if (sph.geometry) sph.geometry.dispose();
      if (sph.material) (sph.material as THREE.Material).dispose();
      this.scene.remove(sph);
    }
    this.clearLinks();
  }

  disposeGarbage() {
    for (let garbage of this.disposeAtStep) {
      create.destroyGroup(this.scene, garbage);
    }

    if (this.restoreColors != undefined) {
      this.updateColors(this.restoreColors);
      this.restoreColors = undefined;
    }

    if (this.centroidColors != undefined) {
      this.centroidColors = undefined;
      while (this.centroids.length > 0) {
        create.destroy(this.scene, this.centroids.pop()!);
      }
    }

    if (this.planeDisk != undefined) {
      create.destroy(this.scene, this.planeDisk);
      this.planeDisk = undefined;
    }

    if (this.planeArrow != undefined) {
      create.destroyGroup(this.scene, this.planeArrow);
      this.planeArrow = undefined;
    }
  }

  update(data: crapi.Update) {
    this.disposeGarbage();
    this.updateSkin(data.points);
    this.updateCentroids(data.centroids.points);
  }

  updateSkin(points: crapi.Point[]) {
    if (points.length > this.nodes.length) {
      for (let i = this.nodes.length; i < points.length; i++) {
        let point = points[i];
        let [radius, color] = this.nodeDisplay(i);
        let sph = create.sphere(this.scene, point, radius, color);
        this.nodes.push(sph);
      }
    }

    this.updatePoints(this.nodes, points);
  }

  updateCentroids(centroids: crapi.Point[]) {
    while (this.centroids.length < centroids.length) {
      this.centroids.push(
        create.sphere(
          this.scene,
          [0, 0, 0],
          0.3,
          this.centroidColor(this.centroids.length)
        )
      );
    }
    while (this.centroids.length > centroids.length) {
      create.destroy(this.scene, this.centroids.pop()!);
    }

    this.updatePoints(this.centroids, centroids);
  }

  updatePoints(points: Mesh[], newPositions: crapi.Point[]) {
    if (newPositions.length != points.length) {
      console.error("Position data got corrupted");
    }

    for (let i = 0; i < newPositions.length; i++) {
      points[i].position.x = newPositions[i][0];
      points[i].position.y = newPositions[i][1];
      points[i].position.z = newPositions[i][2];
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

  nodeDisplay(index: number): [number, THREE.Color] {
    let radius = 0.1;
    let color = this.nodeColors[index];

    const peculiarity = this.peculiarities[index];
    if (peculiarity == "Locked") {
      radius = 0.3;
      color = [120, 120, 120];
    }

    return [
      radius,
      new THREE.Color(color[0] / 255, color[1] / 255, color[2] / 255),
    ];
  }

  linkColor(_from: any, _to: any): THREE.Color {
    if (this.displayMode == "debug") {
      return new THREE.Color(52 / 255, 51 / 255, 48 / 255);
    }

    const color = this.nodeColors[_from];
    return new THREE.Color(color[0] / 255, color[1] / 255, color[2] / 255);
  }

  centroidColor(i: number): THREE.Color {
    if (this.centroidColors != undefined) {
      const color = this.centroidColors[i];
      return new THREE.Color(color[0] / 255, color[1] / 255, color[2] / 255);
    }
    return new THREE.Color(1, 165 / 255, 0);
  }

  displayNormals(data: crapi.Point[]) {
    // if (data.length != this.nodes.length) {
    //   console.error(`Points and normals must have the same length`);
    //   return;
    // }

    for (let i = 0; i < data.length && i < this.nodes.length; i++) {
      const nodePos = this.nodes[i].position;
      const normal = new THREE.Vector3(data[i][0], data[i][1], data[i][2]);
      const arrowLength = normal.length();

      const shaftRadius = 0.05 * arrowLength;
      const headLength = 0.2 * arrowLength;
      const headRadius = 0.1 * arrowLength;

      let arrow = create.arrow(
        this.scene,
        nodePos,
        normal,
        arrowLength,
        normalsColor,
        shaftRadius,
        headLength,
        headRadius
      );
      this.disposeAtStep.push(arrow);
    }
  }

  updateColors(colors: crapi.RGB[]) {
    this.nodeColors = colors;
    for (let i = 0; i < this.nodes.length; i++) {
      // TODO update radius
      let [_radius, color] = this.nodeDisplay(i);
      (this.nodes[i].material as any).color = color;
    }
  }
}
