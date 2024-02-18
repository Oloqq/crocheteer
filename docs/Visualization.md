# Requirements

Code name for the visualized entity is `Plushie`.

Plushie must support
- writing to STL
- inserting / removing / changing stitches and adapting to the changes
  - `Pattern` and `Pattern -> Plushie` conversion must be defined first
- variable pressure (stuffing amount)
- fixed points (one is the bare minimum)

# Implementation

Location of vertices is calculated with a variation of a [force-directed graph](https://en.wikipedia.org/wiki/Force-directed_graph_drawing).

## Attraction / repulsion
Connections (edges) between stitches (nodes) are the most basic source of tension in a `Plushie`. Each two connected nodes will attract on each other with force along the axis going through them, with magnitude given by
$$f(x)=\frac{(x-d)^{3}}{(\frac{x}{2}+d)^{3}}\{x\ge0\}$$
where
- x: distance between nodes
- d: desired distance between nodes
- positive $f(x)$ means attraction, negative means repulsion

![](2024-02-14-07-22-51.png)
*attraction function for d=1*

## Stuffing
Another force acting on a plushie is the stuffing pushing it's walls out. The first attempt at implementing it in this project is called **per-round stuffing**. (Linear time).

### Per-round stuffing
In this approach
- calculate the center of each round
- calculate the desired distance between points and center for each round
- push out points that are too close to the center

Center of a round is calculated as the average of positions of points.

#### Desired distance
- Let $R$ be a round with $N$ stitches, and let $d$ be the desired distance between stitches.
- With realistic stuffing and no other rounds acting on $R$, it's points should be approximating a circle of circumference $Nd$
- Therefore, we can take the radius $r=\frac{Nd}{2\pi}$ of that circle as the desired distance between points and round center, possibly scaled by a constant
- If the distance is less then desired, the node is pushed by a unit vector along the axis between center and itself

![](2024-02-17-23-50-01.png)
*ball generated with attraction + per-round stuffing*

### Failed attempts
There was an attempt to just repulse nodes from line X=0, Z=0. However if to many nodes were pulled to one quadrant of the XZ plane, the whole structured would get skewed in an absurd way