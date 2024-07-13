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
    cloud = create_pointcloud(sys.argv[1], number_of_points=1000)
    points = np.asarray(cloud.points)
    # interactive_visualization(cloud)

    points = points / 4 + np.array([0.8, 0, 0])
    points = [[p[0], p[2], p[1]] for p in list(points)] # y and z are swapped in this open3d

    with open("pointcloud.json", "w") as f:
        json.dump(points, f)