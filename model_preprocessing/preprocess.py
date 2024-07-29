import open3d as o3d
import numpy as np
import json
import sys

def create_pointcloud(path, number_of_points=10000):
    # Load the STL file
    stl_mesh = o3d.io.read_triangle_mesh(path)

    # Ensure the mesh is valid
    if stl_mesh.is_empty():
        print("Couldn't load the mesh")
        exit()

    cloud = stl_mesh.sample_points_uniformly(number_of_points)
    return cloud

def interactive_visualization(cloud):
    # Visualize the point cloud
    o3d.visualization.draw_geometries([cloud])

    # Save the point cloud to a file (optional)
    # o3d.io.write_point_cloud("sampled_point_cloud.ply", cloud)


if __name__ == "__main__":
    if len(sys.argv) <= 1:
        print("provide input file")
    source_path = sys.argv[1]

    cloud = create_pointcloud(source_path, number_of_points=1000)
    points = np.asarray(cloud.points)
    # interactive_visualization(cloud)

    # points = points + np.array([5, 0, 0])
    points = points / 4
    points = [[p[0], p[2], p[1]] for p in list(points)] # y and z are swapped in this open3d

    save_path = source_path.rpartition(".stl")[0] + ".json"
    with open(save_path, "w") as f:
        json.dump(points, f)