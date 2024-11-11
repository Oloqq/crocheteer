/// Crochet API
namespace crapi {
  type Point = [number, number, number];
  type RGB = [number, number, number];
  type EdgeTo = number;
  type integer = number;

  interface Initialize {
    nodes: Nodes;
    edges: EdgeTo[][];
    centroids: CentroidsWrapper;
  }

  type Peculiarities = {
    [node: integer]: "Root" | "Tip";
  };

  interface Nodes {
    points: Point[];
    peculiarities: Peculiarities;
    colors: RGB[];
  }

  // FIXME later so legacy client can still work for a while
  interface CentroidsWrapper {
    centroids: Centroids;
  }

  interface Centroids { }

  interface Update {
    centroids: CentroidsWrapper;
    points: Point[];
  }

  interface Params {
    timestep: number;
    floor: boolean;
    gravity: number;
    desired_stitch_distance: number;
    centroids: {
      number: integer;
      force: number;
      min_nodes_per_centroid: integer;
    };
    autostop: { acceptable_tension: number; max_relaxing_iterations: integer };
    keep_root_at_origin: boolean;
    single_loop_force: number;
    initializer: string; // TODO enum
    hook_leniency: string; // TODO enum
    minimum_displacement: number;
  }

  interface CentroidParams { }
}