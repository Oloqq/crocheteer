# Problem
The experimental pattern is the heart.

To build it, first one hump is crocheted, then the user starts working on the second hump from a new magic ring, and joins those 2 together.

Allowing multiple MRs introduces problems:
- which node should be the root?
- should there even be a root?
  - yes, Plushies may fly offscreen otherwise
  - but another way is to lock at least one point in space

"Keep root at origin" makes no sense with MultiPart. But we need to lock some points in space. ACL should allow to designate specific points to be locked in space. ACL should allow setting precise coordinates for the points. There should be a mechanism to unlock points, e.g. when a certain another stitch has been completed. This could be implemented using marks.

Allow user to move the locked points in the GUI. Display new coordinates in a way that a user can copy them


# Additions
- "around" keyword
- "floored", "rooted" as ACL parameters
  - TODO when rooted = false, ensure there is at least one fully locked point, raise error otherwise