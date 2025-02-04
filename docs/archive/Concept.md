# Concept (use cases)
## Generate a pattern that will create a specific shape
- user provides:
  - 3D model (STL) of the desired shape
- crocheteer:
  - generates a pattern that will form the user's shape
  - presents the pattern in a human readable format
  - allows viewing the result as a 3D model

## Show me what shape will that pattern create
- user provides:
  - a pattern in a concrete format, similar to the commonly used informal formats
- crocheteer:
  - creates a 3D visualization

# Limitations
Main focus of the project is amigurumi, that is closed shapes filled with stuffing.

Patterns generating lots of folds (kinda like models of non-euclidean planes) are currently not considered, as they would require a different approach in the force graph stage.

# Components
- STL loader
  - https://docs.rs/stl_io/latest/stl_io/
- [formal grammar of a pattern](./Pattern%20grammar.md)
- pattern generator
  - [genetic](./Genetic.md)
  - procedural?
    - simple to define for shapes with axial symmetry, gets complex for other cases
- pattern to 3D model conversion
- pattern visualization
  - STL export
  - interactive render

# Formal grammar of a pattern
See solution in [Patter grammar.md](./Pattern%20grammar.md)


# Pattern to 3D model conversion
See solution in [Simulation.md](./Simulation.md)

Codename for a converter utilizing a force graph is `Plushie`.

At the beginning may work with any trash represantation of a pattern.

## Problem
- fabric is not stiff
- a packed / sparse region may affect the whole object
- stuffing will push the walls out
- force graphs seem like the ideal solution
  - each stitch
    - is a node
    - is connected (has edges)
       - to the previous stitch in the current row
       - to $[1, ∞]$ (theoreticaly) stitches from the previous row(chains with no connection to the previous round (aka skips) are not supported -- gotta change STL exporting for that to work)
  - the origin of the starting circle has to be grounded (unmovable)

## Viewing the model
### STL
`Plushie` determines the stitches position in a 3D model. Those can be exported to an `.stl` file, and viewed with any program that supports those files. Extension for vscode `slevesque.vscode-3dviewer` works very well for development.

NOTE: normals of triangles are set basically randomly, so **view the models as wireframe** if you value sanity

### Interacive
Crocheteer can act as a websocket server that allows relaxing the force graph one step at a time. A browser based [client](../ws_web_client/) has been implemented. It works way slower than expected, but there's room for optimization. Implementing this visualization directly in rust binary may be easier at this point though.

Three.js inspired crate may be the solution. https://docs.rs/three/latest/three/

# Tools

<!-- [1]: (3D Viewer for VSCode) -->
[1]: https://marketplace.visualstudio.com/items?itemName=slevesque.vscode-3dviewer
1. 3D Viewer for VSCode
  - Id: slevesque.vscode-3dviewer
  - uses 3rd coord as height, while Fusion uses 2nd coordinate as height
  - configuring Fusion will be probably easier than the extension -> stick with 3rd as height

[2]: https://github.com/bddap/watch-stl-rust


