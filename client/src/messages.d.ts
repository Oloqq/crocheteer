/// Crochet API
namespace crapi {
  type Point = [number, number, number];

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

  interface Edge {}

  // FIXME
  interface CentroidsWrapper {
    centroids: Centroids;
  }

  interface Centroids {}

  interface Update {
    centroids: CentroidsWrapper;
    points: Point[];
  }
}
