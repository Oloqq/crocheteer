# Next

# Backlog
- update documentation relating to hook and plushie
- allow color before starter
- foam proves the coloring is wrong now
- R*Tree project uses https://github.com/sebcrozet/kiss3d, which seems nice for a visualization client
- display requested vs actual centroids in the gui
- jump the creation to root when rooted changes values from false to true
- bugs:
  - `rooted` fully disables gravity
  - timestep has no effect
  - there is no real reason why plushie should be unable to recover from this ![](images/2024-04-29-22-22-27.png)

- adding some inertia/weight to a point after it is moved by a user would make nudging actually useful (possible with mutable Constraints)

- rewrite token_args macro as procedural

- plushie
  - single loop force is the thing causing rotation
    - anchoring at least one more point next to root should fix that
      - how to determine when it should become stationary?
    - can this be fixed somehow by playing with vectors?
  - attaching a chain to a set point
  - add option to force aligment in the Y axis via calculating center of mass, then rotating points around origin

- genetics
  - stl to shape conversion
    - shape refactor using kd trees

- websocket server/client
  - add a `<select>` with sample patterns instead of the text input
  - allow changing simulation speed (corresponds to time variable in step)
    - also include other parameters in the gui
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
    - provide a binary on github
  - heroku https://elements.heroku.com/buildpacks/emk/heroku-buildpack-rust
  - should be $5 per month