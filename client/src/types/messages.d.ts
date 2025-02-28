/// Crochet API
namespace crapi {
  type Point = [number, number, number];
  type RGB = [number, number, number];
  type EdgeTo = number;
  type integer = number;

  interface Initialize {
    nodes: Nodes;
    edges: EdgeTo[][];
    centroids: Centroids;
  }

  type PushPlane = [number, number, number];
  type Peculiarities = {
    [node: integer]: "Locked" | "Tip" | { FLO: PushPlane } | { BLO: PushPlane };
  };

  interface Nodes {
    points: Point[];
    peculiarities: Peculiarities;
    colors: RGB[];
  }

  interface Centroids {
    points: Point[];
  }

  interface Update {
    centroids: Centroids;
    points: Point[];
  }

  interface Params {
    timestep: number;
    desired_stitch_distance: number;
    centroids: {
      number: integer;
      force: number;
      min_nodes_per_centroid: integer;
    };
    single_loop_force: number;
    initializer: "Cylinder" | "OneByOne";
    minimum_displacement: number;
    track_performance: boolean;
    skelet_stuffing: {
      enable: boolean;
      cluster_number: integer;
      must_include_points: number;
      allowed_overlap: number;
      autoskelet: bool;
      interval: integer;
    };
  }

  interface ChangeColors {
    standard: RGB[];
    variable?: RGB[][];
  }
}
