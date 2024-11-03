/// Crochet API
namespace crapi {
  type Point = [number, number, number];
  type EdgeTo = number;

  interface Initialize {
    nodes: Nodes;
    edges: EdgeTo[][];
    centroids: CentroidsWrapper;
  }

  interface Nodes {
    points: Point[];
    peculiarities: any; // TODO
    colors: any; // TODO
  }

  // FIXME later so legacy client can still work for a while
  interface CentroidsWrapper {
    centroids: Centroids;
  }

  interface Centroids {}

  interface Update {
    centroids: CentroidsWrapper;
    points: Point[];
  }

  interface Params {}
}
