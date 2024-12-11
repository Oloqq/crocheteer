# Next
- create a Y shape

# Then

- ACL refactors
  - currently, metadata in ACL cannot save arbitrary strings or floats, just read until end of line there
  - allow color before starter
  - metadata -> parameters
  - operations -> actions

- include chains in the tutorial


# Backlog (need to do for thesis)

- skeleton stuffing
  - scale checking
  - select optimal parts

- debug mode that shows peculiarities

- jump the creation to root when rooted changes values from false to true
  - verify parameters displayed in the gui all make sense

- toggle centroids visibility

# After-inzynierka

- deployment
  - make rust serve the index.html
    - provide a binary on github
  - heroku https://elements.heroku.com/buildpacks/emk/heroku-buildpack-rust
  - should be $5 per month

- R*Tree project uses https://github.com/sebcrozet/kiss3d, which seems nice for a visualization client

- rewrite token_args macro as procedural

- plushie
  - single loop force is the thing causing rotation
    - anchoring at least one more point next to root should fix that
      - how to determine when it should become stationary?
    - can this be fixed somehow by playing with vectors?
  - attaching a chain to a set point
  - add option to force aligment in the Y axis via calculating center of mass, then rotating points around origin

- websocket server/client
  - add a `<select>` with sample patterns instead of the text input
  - allow changing simulation speed (corresponds to time variable in step)
    - also include other parameters in the gui
  - save pattern source on update
  - stop panicks after moving a centroid

- frontend
  - highlight points when cursor is on the text area
  - display a closed shape (mesh)

- bugs:
  - `rooted` fully disables gravity
  - timestep has no effect
  - there is no real reason why plushie should be unable to recover from this ![](images/2024-04-29-22-22-27.png)


- adding some inertia/weight to a point after it is moved by a user would make nudging actually useful (possible with mutable Constraints)
  - need to reenable nudging first