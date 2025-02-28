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

## Dragging
Gui should allow to disable the dragging behavior so it does not mess up camera controls when not needed.

Dragging a node far away messes up the simulation

Need a way to set a specific position programatically

Dragging does not really work with @reflect_locked = true

# Hook
Hook is designed to work from a single starter.
- Do we adjust Hook so it handles multiple starters?
- Do we use multiple Hooks in a plushie that are eventually joined into one?
Plushies started from different starters need to be able to get joined.
Plushies have to be able to get simulated without being joined.
Node ids in the simulation must stay unique.
Hook needs to retain mark information between joins

## Solution 1


# Additions
- "around" keyword
- "floored", "rooted" as ACL parameters
  - TODO when rooted = false, ensure there is at least one fully locked point, raise error otherwise
- fasten off with tip could be replaced with a few links between the stitches of the last round?
- Constrained -> Locked
- Rooted -> Locked
- Limit of stuffing force has been adjusted from 1000.0 to 1.0
- Introduced limit for link force (1.0)