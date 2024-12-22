## Simulation overview
Two kinds of forces need to act on a Plushie. First one, *link force*, shall try to keep connected stitches at a specific distance from one another. Second one, *stuffing force*, shall simulate the effect of stuffing inside the piece on the fabric. The first force is trivial, but for the second one, we need to get a notion on where is the inside of the Plushie.

Two approaches are explored for *stuffing force*, that we call *centroid stuffing* and *skeleton stuffing*.

## Initial node positions
Before forces can be applied to the graph, it's nodes have to be assigned some initial positions, that is $N: I \rightarrow \R^3$.

Crocheteer implements two position initializers that the user can select using ACL's parameter `initializer`.

### Cylinder initializer
The default initializer in Crocheteer is cylinder initializer. It can also be explicitly selected selected by using:
```
@initializer = cylinder
```

In this method, each round (as per `round_spans` from section 4) is arranged in a ring laying parallel to the horizontal plane, and every round is placed higher along the vertical axis. The resulting shape is a generalized cylinder




### One by one initializer
```
@initializer = obo
```

## Link force

## Centroid stuffing

## Skeleton-based stuffing

[2]: https://arxiv.org/pdf/1912.11932