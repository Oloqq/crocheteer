/// Crochet API
namespace crapi {
  interface Initialize {
    nodes: Nodes;
    edges: Edge[];
    centroids: CentroidsWrapper;
  }

  interface Nodes {
    points: Point[];
    peculiarities: any; // TODO
    colors: any; // TODO
  }

  type Point = any; // TODO

  interface Edge {}

  // FIXME
  interface CentroidsWrapper {
    centroids: Centroids;
  }

  interface Centroids {}
}
