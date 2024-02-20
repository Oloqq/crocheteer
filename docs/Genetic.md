# Prepare user's file for comparison

user
-> stl
-> take triangle vertexes
-> put points there
-> `Shape` with interpolation and resampling for high density

https://github.com/evanchodora/stl-slicer


# Fitness function

genetic algorithm
-> pattern
-> plushie
-> relaxing/stuffing
-> points
-> `Shape`

- the points form a 3D model
- let `Shape` be a sliced 3D object (list of `Slice`) (slicing as in 3d printing)
- convert the points to a `Shape`
  - but points are on arbitrary heights, so how to slice?
  - create the `Slice` like:
    - choose a height
    - choose all points in certain vertical distance
      - dont't let this operation become O(nm) n-nodes, m-levels
        - utilize the fact that points should have gradually increasing height
        - or apply a function O(n) that will assign nodes to buckets
    - for each `Slice` we now have a set of points on the perimeter
- ✅comparing `Slice`s
  - ✅not considering folds
    - what about smth like Hu Moments? (like in image processing)
      - https://learnopencv.com/shape-matching-using-hu-moments-c-python/
    - it's probably reasonable to assume that points will be distributed somewhat uniformly across the perimeter of the slice
      - if not, resampling would be needed and that's a whole another problem
    - ✅ we can just compare each point to position of the point closest to it
      - ⏳ POLAR COORDINATES allow some kind of point sorting
      - sounds like some matrix operation could speed that up?
      - with this approach, slices from user's shape MUST be uniform and dense
  - considering folds (not now)
    - now we gotta figure out what's inside and outside
      - region growing like in image processing?
    - does shape matching still work?
    - image processing approach
      - grow region
      - binary operations on expected and actual
      - count pixels as fitness


