<!-- TODO make sure current implementation is mentioned as Crocheteer in the introduction -->

# Representing connected stitches as a graph

## Overview
Once the pattern is parsed into a list of actions, those actions need to be transformed into a graph, that will later have forces applied to it. In this graph, nodes shall represent singular stitches or more precisely, spots where a human performs "pull over" (as shown in figure X and [[1]] page 7, under "Single Crochet", step 2).
<!-- TODO figure num -->

![alt text](images/image-4.png)


For example, action `sc` would produce one node in the graph, `dec` would produce one node as well, and `inc` would produce two nodes.

Edges in the graph represent the fabric connecting those stitches. A typical stitch is connected to the previously created stitch, and to one or more stitches from the last round. Details and exceptions are described throughout this section.

The procedure takes a list of stitches and actions produced by ACL parser and outputs a tuple describing nodes and edges of the graph:
$$(E, C, P)$$
Where
<!-- TODO need help with math notation -->
$$
I: \text{valid node indexes}
\\
E: I \rightarrow \{ j: \text{nodes } i \text{ and } j \text{ are connected } \land i > j \}; i,j \in I
\\
C: I \rightarrow [0, 255]^3
\\
P: I \rightarrow \{ \text{null, root, tip, FLO, BLO} \}
$$

- $E$ represents the edges of the graph. Note that the condition $i > j$ is introduced to ensure the representation is unamiguous.
- $C$ represents the color of each node. Crocheteer works with 8 bit RGB values.
- $P$ is a dictionary storing peculiarities of graph nodes. The significance of $P$ will be exlained in section 5 describing the forces acting on the graph. These peculiarites include nodes that are
  - a virtual node at the start of a piece (root)
  - virtual nodes created by an FO (tip)
  - attached to only the front loop or the back loop (FLO/BLO)

## Graph construction
In Crocheteer[[3]], graph construction is handled by struct `Hook`, and the output tuple is defined as `HookResult`.

`Hook` iterates over the list of stitches and actions produced by ACL parser and tries to apply each of them, while looking for semantic errors. These include misplacing a MR, duplicating a `mark`, `goto` to an undefined `mark`, and others.

As `Hook` constructs the graph, it assigns an incrementing index to each stitch.
For each stitch, `Hook` creates a new entry in $E$. That entry is a set of node indexes that the new stitch is connected to.

### State management
`Hook` stores two kinds of internal state. First one is related to the whole creation; call it global state. The second kind of state describes the currently worked round; call it `Moment`. This separation is crucial to implement `mark`, `goto`, and `attach`, that is actions that allow user to alter the standard order of creating stitches. The global state stores a mapping from `mark` labels to `Moment`.

#### Moments
`Moment` is defined as the following structure:
```rs
struct Moment {
    cursor: usize,
    anchors: Queue<usize>,
    round_count: usize,
    round_left: usize,
    working_on: WorkingLoops,
}

enum WorkingLoops {
    Both,
    Back,
    Front,
}
```
- `cursor` is the next node index to be used.
- `anchors` is a queue storing indexes of stitches from the previous round.
- `round_count` and `round_left` store number of stitches produced for the current round and number of stitches still available from the previous round. See `round_spans` in global state.
- `working_on` keeps track of FLO/BLO.

<!-- NOTE there is no error checking on mistake in FLO BLO and gotos -->

#### Global state
The global state is stored directly in `Hook` instance.
```rs
struct Hook {
    edges: Edges,
    peculiar: HashMap<usize, Peculiarity>,
    now: Moment,
    /// Contains first and last stitch of each round. Treated as a range, both extremes are inclusive.
    round_spans: Vec<(usize, usize)>,
    /// Storage of index -> it's anchor, used for single loop forces
    parents: Vec<Option<usize>>,
    /// Storage of spots for Mark and Goto
    labels: HashMap<Label, Moment>,
    /// Current color/yarn. Not stored in Moment as typically yarn changes happpen independently of switching positions.
    color: colors::Color,
    /// Storage of index -> it's color
    colors: Vec<colors::Color>,
    // Previous stitch might need to be overwritten after a Goto
    override_previous_stitch: Option<usize>,
    /// Last stitch created (not counting actions like mark, goto)
    last_stitch: Option<Action>,
    /// Was the last action a mark?
    last_mark: Option<Action>,
}

struct Edges {
    edges: Vec<Vec<usize>>,
}

enum Peculiarity {
    Root,
    Tip,
    BLO(PointsOnPushPlane),
    FLO(PointsOnPushPlane),
    Constrained(Vector3),
}

pub type Label = usize;
```

- `edges` directly corresponds to $E$ from the procedure output. Type `Edges` is introduced, instead of directly using `Vec<Vec<usize>>`, so the type can implement error checking and enforce the condition $i > j$ of $E$.
- `peculiar` directly corresponds to $P$ from the procedure output. `Peculiarity` enum is described in section 4.X<!-- TODO Handling front-loop-only and back-loop-only -->
- `now` stores the current `Moment`
- `round_spans` is used to keep track of where rounds begin and end. This is mostly for setting initial positions of nodes described in section 5, but may also be used to verify the round counts annotated in ACL.
- `parents` is relevant to single loop forces and is described in section 4.X<!-- TODO Handling front-loop-only and back-loop-only -->
- `labels` stores the `Moments` created with ACL's `mark`. Notice that `Label` here is an unsigned integer, as ACL parser already indexed the marks.
-

### Handling basic stitches

### Handling front-loop-only and back-loop-only
```rs
pub enum Peculiarity {
    Root,
    Tip,
    BLO(PointsOnPushPlane),
    FLO(PointsOnPushPlane),
    Constrained(V),
}

pub type PointsOnPushPlane = (usize, usize, usize);
```
[1]: https://www.montana.edu/extension/blaine/4-h/4h_documents/CrochetMadeEasy.pdf
[3]: https://github.com/Oloqq/crocheteer
