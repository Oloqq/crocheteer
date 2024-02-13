# Concept
- user provides
  - 3D model (STL) of the desired shape
    - (0, 0, 0) is the starting point
    - starting vector is on the X axis
    - model forms along the Z axis
  - yarn thickness
  - scale factor or a desired height
- tool generates a pattern
- pattern can be exported to an STL for visualization

# Components
- https://docs.rs/stl_io/latest/stl_io/
- STL preprocessor
- pattern generator (one of)
  - genetic
    - fitness function -- how to compare two 3d shapes?
  - procedural
- pattern visualizer

# Pattern visualizer
First thing to implement.

At the beginning may work with any trash represantation of a pattern.

## Problem
- fabric is not stiff
- a packed / sparse region may affect the whole object
- stuffing will push the walls out

## [Force graphs](./Visualization.md)
- each stitch
  - is a vertex
  - is connected
     - to the previous stitch in a row
     - $[1, ∞]$ (theoreticaly) stitches from the previous (chains with no connection to the previous round are not supported -- important for visualization)
  - it's connections are edges
- the origin of the starting circle has to be grounded (unmovable)
- "stuffing force" should be applied radialy, coming from a pillar standing in the center of the starting circle
  - assumption: no wall is allowed to cross the pillar

- hot reload in a viewer [[1]] [[2]] should allow easy monitoring of the process
  - 3D Viewer for vscode seems fine for that

# Tools

<!-- [1]: (3D Viewer for VSCode) -->
[1]: https://marketplace.visualstudio.com/items?itemName=slevesque.vscode-3dviewer
1. 3D Viewer for VSCode
  - Id: slevesque.vscode-3dviewer
  - uses 3rd coord as height, while Fusion uses 2nd coordinate as height
  - configuring Fusion will be probably easier than the extension -> stick with 3rd as height

[2]: https://github.com/bddap/watch-stl-rust


