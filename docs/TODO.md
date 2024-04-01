# Next
- update documentation relating to hook and plushie
- allow all control sequences in the human readable pattern

# Backlog

- documentation
  - clean up images (into a folder)

- plushie
  - investigate plushie rotating
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
  - heroku https://elements.heroku.com/buildpacks/emk/heroku-buildpack-rust
  - should be $5 per month