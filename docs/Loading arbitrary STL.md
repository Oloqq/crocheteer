In order to compare a generated shape to the expected result, the file must be transformed into a `Shape`

STL:
- does not form a uniform point cloud
- may be arbitrarily scaled up/down
- may not start at (0,0,0)

Conversion:
- User inputs desired height in cm/inches and yarn size
- A function is defined that translates that real world height to simulation height accounting for yarn size
- Get highest and lowest point in the model
- Translate lowest to (0,0,0)
- Scale all the others (in all dimensions by the same factor) so that highest point lands at desired height
- Resampling
  - as the desired stitch distance is 1.0
  - we need a point cloud where neighbors are at most 1.0 unit away
  - resampling should be done on the surface of triangles in the mesh
    - for each triangle in original mesh
      - calculate it's area, check if it's small enough
        - **no**
        - very smooth triangles can have large distances between vertices and still keep small area