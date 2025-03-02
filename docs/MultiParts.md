# Problem
The experimental pattern is the heart.

To build it, first one hump is crocheted, then the user starts working on the second hump from a new magic ring, and joins those 2 together.

Allowing multiple MRs introduces problems:
- which node should be the root?
- should there even be a root?
  - yes, Plushies may fly offscreen otherwise
  - but another way is to lock at least one point in space

"Keep root at origin" makes no sense with MultiPart. But we need to lock some points in space. ACL should allow to designate specific points to be locked in space. ACL should allow setting precise coordinates for the points. There should be a mechanism to unlock points, e.g. when a certain another stitch has been completed. This could be implemented using marks.

Allow user to move the locked points in the GUI. Display new coordinates in a way that a user can copy them.

# Locking
Simple and naive locking of points without redirecting the forces acting on the locked points may lead to artifacts (the locked point "collapses" inwards). The forces should be redirected just as they are when plushie is @rooted. In this case Peculiarity::Root becomes redundant. Peculiarity::Constrained does not make sense anymore (why would be want to slow down movement of a node).
Refactor Peculiarity::Constrained into Peculiarity::Locked, and merge the logic of Peculiarity::Root and Peculiarity::Locked (remove Peculiarity::Root).

@multipart = true
@floored = false
@reflect_locked = true
with a single part should be equivalent to
@multipart = false
@floored = false
@rooted = true

reflect_locked does not work when two parts are not connected

Let's set aside locking arbitrary points now and stick to MR roots.

## Dragging
Gui should allow to disable the dragging behavior so it does not mess up camera controls when not needed.

Dragging a node far away messes up the simulation

Need a way to set a specific position programatically

Dragging does not really work with @reflect_locked = true

# Stuffing
Centroids need to be restricted to certain parts.

Global @centroids does not make sense with @multipart, unless the global number of centroids is distributed between the parts
- easy mode: based on node number
- hard mode: distributed dynamically based on sum of weight for centroids in parts?

Let's introduce a concept of a `Limb`, that is, a piece of Plushies skin (skin = nodes = stitches) with their own centroids (since we were already calling centroids bones in the context of skeletonization, I feel that Limb is more expressive that Part)

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

# FO
I see no way to distinguish in-round FO from the tip FO
I was already dubious of creating the tip
Disable tip, make FO essentialy a noop
do it under @tip_from_fo

# New in backlog
Cylinder initializer could be adjusted for multipart
Leniency is really obsolete
ChainOfZero (and probably others) does not display line number
Make attach use size 0 by default (make argument optional)

# ACL clarification
the annotated round count, speaking precisely, shall be the number of anchors available at a given point. It is too complex to be checked in the parser. It must happen in Hook. There is no reason to restrict it to the end of the round.
If we accept that anchor number tracking is too complex for parser, the "around" keyword has to be handled in Hook.

# Attach
Attach currently does not create a node by itself.
Some assumptions are therefore made in hook.
If user puts `attach(..), goto(..)` the state may get corrupted.

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