`Hook` is an essential part of the project, responsible for transforming a list of stitch names into a fabric sheet (`HookResult`) that can then be simulated (using `Plushie`).

# currently
`Plushie` requires
- edges in the graph
- initial positions of vertices in space
- map of vertices that require special treatment (peculiarities)
- approximate height of the plushie for generating centroids

determining initial positions becomes a major problem
- requires information where rounds start and end
  - this complicates internal Hook logic
- leads to prolonged simulation, as each height level in the plushie has to rotate
- becomes even weirder for shapes with more than 1 generalized cylinder

# refactor
- `Plushie` requires
  - edges in the graph
  - peculiarities
- Plushie handles adding the points to simulation by itself
  - new position
    - more than 1 link => based on average of linked nodes
    - 1 link => take it's position, translate by a random vector
  - gotta assure referenced nodes already exist
- centroids also gotta be added by Plushie itself along the way