import open3d as o3d
import numpy as np
import os
import sys
import imageio

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

def showcase(pcd):
    vis = o3d.visualization.Visualizer()
    vis.create_window(visible=False)
    vis.add_geometry(pcd)

    # Define the camera trajectory
    ctr = vis.get_view_control()
    parameters = ctr.convert_to_pinhole_camera_parameters()

    # Directory to save frames
    output_dir = "frames"
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    print("1")

    # Capture frames while rotating the view
    num_frames = 10
    for i in range(num_frames):
        # print(i)
        angle = 2 * np.pi * i / num_frames
        # print(i)
        # rotation_matrix = o3d.geometry.get_rotation_matrix_from_xyz((0, angle, 0))
        # print(i)
        # pcd.rotate(rotation_matrix, center=(0, 0, 0))

        # print(i)
        ctr.convert_from_pinhole_camera_parameters(parameters)
        # print(i)
        vis.update_geometry(pcd)
        # print(i)
        vis.poll_events()
        print(i)
        vis.update_renderer()

        # Capture frame
        frame = vis.capture_screen_float_buffer(False)
        frame = (255 * np.asarray(frame)).astype(np.uint8)
        # print("brajbfds")
        imageio.imwrite(f"{output_dir}/frame_{i:03d}.png", frame)

    # print("2")


    vis.destroy_window()

    # Create a GIF or MP4 from the captured frames
    with imageio.get_writer('animation.mp4', fps=30) as writer:
        for i in range(num_frames):
            filename = f"{output_dir}/frame_{i:03d}.png"
            image = imageio.imread(filename)
            writer.append_data(image)
    # print("3")


    # Clean up the frames directory
    # for i in range(num_frames):
    #     os.remove(f"{output_dir}/frame_{i:03d}.png")
    # os.rmdir(output_dir)

    print("Animation saved as animation.mp4")


if __name__ == "__main__":
    cloud = create_pointcloud("../models/grzib40.stl")
    interactive_visualization(cloud)
    # showcase(cloud)