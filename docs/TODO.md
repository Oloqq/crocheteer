# Next
- plushie
  - replace current SITTING constant to a thing set through gui

# Backlog
- Pattern/Plushie refactor
  - non-uniform stuffing
  - build Plushie directly from the builder
  - eliminate Pattern struct
  - handle dec overflowing into the next round properly
  - manipulate stitches mid-simulation

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
  - add a <select> with sample patterns instead of the text input
  - allow changing simulation speed (corresponds to time variable in step)
  - save pattern source on update

- frontend
  - highlight points when cursor is on the text area
  - display line number in the text area + mby auto counting of rounds and stitch numbers
  - configure width and color of the edges
  - display a closed shape (mesh)

- deployment
  - make rust serve the index.html
  - heroku https://elements.heroku.com/buildpacks/emk/heroku-buildpack-rust
  - should be $5 per month