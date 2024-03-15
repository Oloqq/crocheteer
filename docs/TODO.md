# Next
- Pattern/Plushie refactor
  - Pattern
    - COULD it hold a map (two way) between text and nodes (for highlighting in the simulation)
      - only for from_human_readable
      - doesn't make sense for genetic
  - genetic optimizations
    - MUST allow inserting / deleting stitches at arbitrary positions mid simulation
    - maybe it's sufficient if we generate the plushie from the ground up, but assign 3D positions from the old shape, that way minimal relaxing should be required
  - handle dec overflowing into the next round properly
    - this is a hack in builder.round_like
  - MUST have non-uniform stuffing
    - allow marking stitches for stuffing/no stuffing in the pattern
      - only for human-readable
      - genetic could have it working with
      - detect nans during simulation

  - chains
  - allow starting from a chain
    - allow locking the starting chain in place
  - reversing working direction
  - FLO/BLO
    - creating a vertex B on the backloop:
      - B anchors on the previous round (stitch P1)
      - P1 itself is anchored on an earlier round to stitch P2
        - getting P2 would be non trivial and would involve a search either through all points, or storing round information and a search through one round
      - P1 is attached to previous round in it's round (P0)
        - index(P0) = index(P1) - 1
      - P1 attaches to the next point in it's round (P2)
        - index(P2) = index(P2) + 1
      - create a plane through P0, P1, P2
        - take a normal to that plane, push B along the normal (+/- depending on back or front loop)
      - cross product
        - let v1 = P0 -> P1
        - let v2 = P1 -> P2
        - v1 cross v2 gives a normal right?
  - allow highlighting a specific round within the pattern

- Base representation is just a list of stitches (like in genetic currently)
- New conversions
  - Human -> Plushie
    - class `Pattern` is only applicable to human readable
    - and has a `Flow` inside
    - collapse the rounds into list of actions (like genetic), call it `FlatFlow`
  - Genetic -> Plushie
    - `Program = FlatFlow`
    - build the list of points directly
  - Genetic -> Human
    - not important yet

- `Flow` consists of other `Flow`s
-  `Action` is the most basic instance of a Flow
- Builder works on ~~stitches~~ `RepeatedAction`:
  - `RepeatedAction` :=
    - repeats: usize
    - action: Action
    - Action -> RepeatedAction conversion is defined with repeats = 1
    - Action * usize -> RepeatedAction conversion is defined
  - Action
    - Sc
    - Inc
    - Dec
    - Ch(X), ..., Attach(Y) -> Chain of X, attach on the shape Y anchors after the chain started (min 1) (max round length)
      - must appear in pair
    - Reverse
    - FLO
    - BLO
    - Both loops
    - Switch working position
      - Goto
      - Mark - Save position to go back to later

# Backlog

- complex patterns
  - working the back or the front loop only (BLO/FLO), with possibility to switch mid round
  - switching "working position" like working the front loop, then going back and working the back loop
  - chains
  - attaching a chain to a set point

- genetics
  - stl to shape conversion
    - shape refactor using kd trees

- human readable pattern
  - refactor parser so weird subpattern shit is fixed

- websocket server/client
  - add a `<select>` with sample patterns instead of the text input
  - allow changing simulation speed (corresponds to time variable in step)
  - save pattern source on update
  - stop panicks after moving a centroid

- frontend
  - highlight points when cursor is on the text area
  - display line number in the text area + mby auto counting of rounds and stitch numbers
  - configure width and color of the edges
  - display a closed shape (mesh)
  - adjust the GUI to the refactor

- deployment
  - make rust serve the index.html
  - heroku https://elements.heroku.com/buildpacks/emk/heroku-buildpack-rust
  - should be $5 per month