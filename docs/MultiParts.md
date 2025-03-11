# Problem
The experimental pattern is the heart.

To build it, first one hump is crocheted, then the user starts working on the second hump from a new magic ring, and joins those 2 together.

To allow multiple MRs we need to rethink a couple things.

# Limbs
Let's introduce the concept of a `Limb`, that is, a piece of Plushies skin (skin = nodes = stitches) with their own centroids (we were already calling centroids "bones" in the context of skeletonization, and I feel that "Limb" is more expressive than "Part", and explains the typical use case better).
<!-- TODO rename centroids to bones -->

# Root node
The first node (virtual node in the middle of MR) used to be marked as root.
The root node was necessary because with certain techniques (SLO), or when Plushies are not closed, the creation would fly away.
Forces were calculated for the root node in ordinary way. However, Root was a special case as the whole Plushie would be translated by the forces that acted on the Root, so the Root was never allowed to leave origin.

This treatment can only be applied to a single node (as experiments proved).

Theoretically, the first node of the first MR (first `Limb`) could be Root, and the virtual nodes of the next MRs (next `Limbs`) could be treated normally. But this approach only works if the `Limbs` eventually get joined. (We don't want even a single Limb to fly away).

## Peculiarity::Locked
Any Root logic shall be removed. The Plushie will instead be kept on screen by placing `Peculiarity::Locked` on the virtual nodes of each MRs. Locked nodes cannot move from their starting position. Those positions have to be selected somehow, so ACL needs to be extended.

If more than one MR is present in the pattern, each MR must be provided 2 arguments: *ring size* and *limb label*, like so:
```
MR(6, limb_first_hump)
```

<!-- TODO rename part_config to limb_config -->

Then, elsewhere in the pattern the parameters (`limb_config`) for that Limb may be specified:
```
limb_first_hump {
  x = 7,
  y = 0,
  z = 0,
}
```
I would like to keep the ACL parts that are relevant to a human and those only relevant in simulation separate (as much as possible).

The parameters must include:
- x, y, z coordinates
- TODO rotation of the part
  - applied only when creating the first 3 nodes
- TODO centroid number for the part, and possibly other stuffing parameters?

## Draft
There should be a mechanism to unlock points, e.g. when a certain another stitch has been completed, or Limbs got joined. This could be implemented using marks.


# Node dragging
The new `Peculiarity::Locked` allows us to implement sensible dragging.

Allow user to move the locked points in the GUI.

TODO Display new coordinates in a way that a user can copy them (useful for setting `limb_config`).

Gui should allow to disable the dragging behavior so it does not mess up camera controls when not needed.

Dragging a node far away used to mess up the simulation. Max attract force was implemented

Need a way to set a specific position programatically. (see `main.ts` call function)

<!-- TODO reflect_locked is a failed endeavor, clean up -->
Dragging does not really work with @reflect_locked = true

# Stuffing
Centroids need to be restricted to certain parts.

Global @centroids does not make sense with @multipart, unless the global number of centroids is distributed between the parts
- easy mode: based on node number
- hard mode: distributed dynamically based on sum of weight for centroids in parts?

# Hook
Hook is designed to work from a single starter.
- Do we adjust Hook so it handles multiple starters?
- Do we use multiple Hooks in a plushie that are eventually joined into one?
Plushies started from different starters need to be able to get joined.
Plushies have to be able to get simulated without being joined.
Node ids in the simulation must stay unique.
Hook needs to retain mark information between joins

# Slip stitch
The problem pattern uses a slip stitch
- Option 1
  - Be realistic, create another node in the graph
  - Use a different desired stitch distance for the node
    - could be useful for MR and FO (not sure)
      - any other use case?
  - What does "different desired stitch distance for the node" mean
    - say we have node L created as a slip stitch, its predecessor P and anchor A
    - it can't be just a multiplier of link force
      - the shape will still end up the same, just take longer time
    - so we need to start calculating link force individually instead of in pairs
    - there is no reason to disallow slip stitches on one loop, so peculiarities (as they are now) do not work
      - but could work if each node could have a vector of peculiarities
      - but it would still need to be checked outside of normal peculiarity processing
    - but then, if simulation has just L and P, they would just fly off on the vector L->P
      - (the comfort position of P is a "too far" position for L)
    - the "fly off" issue exists anyway (FLO/BLO), so we demand a locked point, so it is acceptable
    - this should not even be a problem since the 2 connected node will balance it out
- Option 2
  - Link the previous node to the next anchor
  - Pretend that anchor was not used (at least round count has to stay realistic)
  - result is less realistic than option 1
  - result is trivially computable
  - since one anchor acts as two, marks could get messed up
  - apart from root/tip nodes (that are on the limits of the plushie, so are easy to ignore) this is the first deviation between real and simulated number of stitches
- going with option 2
- it might mess up part_limits (aka limb_limits)

# Stuffing
<!-- TODO -->
- need to configure centroid number per Limb
  - also need to add rotations

# New in backlog
Leniency is really obsolete
ChainOfZero (and probably others) does not display line number
Make attach use size 0 by default (make argument optional)

# Annotated round counts
the annotated round count, speaking precisely, shall be the number of anchors available at a given point. It is too complex to be checked in the parser. It must happen in Hook.
Hook does not care whether it is at the end of the round. There is no reason to restrict it to the end of the round. Grammar has to be adjusted to allow using it at arbitrary positions
If we accept that anchor number tracking is too complex for parser, the "around" keyword has to be handled in Hook. This can be either done by introducing `Action::Around(Vec<Action>)`, which is problematic since it would prevent Clone, or by introducing `AroundStart` and `AroundEnd` (basically parentheses)

# Attach
Attach currently does not create a node by itself.
Some assumptions are therefore made in hook.
If user puts `attach(..), goto(..)` the state may get corrupted.
This only applies to direct_attach and attach_merge.

# State
Hook does not remove or mark used labels in any way.
Duplicate `goto/attach` may caused undefined behavior.

# Notable other implemented things
- "around" keyword
- "floored", "rooted" as ACL parameters
  - TODO when rooted = false, ensure there is at least one fully locked point, raise error otherwise
- fasten off with tip could be replaced with a few links between the stitches of the last round?
- Constrained -> Locked
- Rooted -> Locked
- Limit of stuffing force has been adjusted from 1000.0 to 1.0
- Introduced limit for link force (1.0)
- Cylinder initializer was trivialized.
  - Round counts are not taken into account anymore
  - Round counting was complex and added massive cognitive load to Hook
  - The initial shape was arbitrary either way
- Tip for FO
  - it was always dubious whether it is useful
  - I see no way to distinguish in-round FO from the tip FO
    - in-round FO is necesasry for multipart, even if it does not matter for the simulator, it does for the human reading
  - @tip_from_fo=true restores building the tip
    - saving for experiments, prolly gonna remove
-