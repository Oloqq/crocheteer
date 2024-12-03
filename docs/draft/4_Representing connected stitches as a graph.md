# Representing connected stitches as a graph

Once the pattern is parsed into a list of actions, those actions need to be transformed into a graph, that will later have forces applied to it. In this graph, nodes shall represent singular stitches or more precisely, spots where a human performs "pull over". That is step 3 in the figure.

![alt text](image-1.png)
https://hobbii.com/blog/single-crochet

For example, action `sc` would produce one node in the graph, `dec` would produce one node as well, and `inc` would produce two nodes.

Edges in the graph represent the fabric connecting those stitches. Each stitch is connected to the previously created stitch (unless `goto` has been used). Each stitch <!-- excluding chains, gotta see if chains make it to final version --> is also connected to one (for `sc` and `inc`) or more (`dec`) stitches from the last round. Stitches from the first round in this case are connected to a virtual stitch representing the center of the magic ring (`MR`).

<!-- TODO one by one vs cyllinder initializer -->

<!-- TODO link the concrete files implementing this -->
The procedure takes a list of actions produced by ACL parser and outputs structures
$$(N_i, E_{i}, C_i, P_i)$$
Where
$$
N_i = \{(x_i,y_i,z_i)\}
\\
E_i = \{ j: N_i \text{ and } N_j \text{ are connected } \land i > j \}
\\
C_i
\\
P_i
$$

- $N$ stores 3D vectors representing the initial positions of nodes of the graph. The positions are arbitrary at this stage, and initialized in a cylindrical shape. Nodes/stitches at lower indexes are created earlier.
- $E$ represents the edges of the graph. Note that the condition $i > j$ is introduced to ensure the representation is unamiguous


<!-- TODO pseudocode -->
```

```