# Requirements

Code name for the visualized entity is `Plushie`.

Plushie must support
- writing to STL
- inserting / removing / changing stitches and adapting to the changes
  - `Pattern` and `Pattern -> Plushie` conversion must be defined first
- variable pressure (stuffing amount)
- fixed points (one is the bare minimum)

NOT supported
- shapes that require walls intersecting the vertical axis (source of pressure)
- importing from STL

# Implementation

Location of vertices is calculated with a [force-directed graph](https://en.wikipedia.org/wiki/Force-directed_graph_drawing)


attration x=distance d=desired distance
$f\left(x\right)=\frac{\left(x-d\right)^{3}}{\left(\frac{x}{2}+d\right)^{3}}\left\{x\ge0\right\}$