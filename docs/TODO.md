- Pattern/Plushie refactor
  - working the back or the front loop only (BLO/FLO)
  - switching "working position" like working the front loop, then going back and working the back loop
  - chains
  - attaching a chain to a set point
  - how to calculate the anchors now available?
  - non-uniform stuffing
  - creations that are not closed at the top (like the vase)
  - handling heavily folded shapes
  - build Plushie directly from the builder
  - handle dec overflowing into the next round properly
  - prevent twitching on the fastened off point

- websocket server/client
  - add a <select> with sample patterns instead of the text input
  - allow changing simulation speed (corresponds to time variable in step)
  - save pattern source on update

- handling bigger shapes
  - stuff gets squished at Y=0, once some of the centroids go up
  - need to unlock Y coord of origin point, then translate the whole shape for the sake of visualization
  - ending point can get really unstable if connected to many stitches (showing even at 11), a function could be applied to limit the twitching when many nodes affect another node
  - display a closed shape
  - configure width and color of the edges

- human readable pattern
  - require fasten off (FO) to actually fasten off
  - repeats (subpatterns)
  - shorter round repeat notation (R1-R5:... -> 5:...)
  - handle mismatch of actual vs noted stitch counts gracefully

- stl to shape conversion
  - first -> shape refactor using kd trees

- build an open vase
  - require fasten off (FO) to actually fasten off

- frontend
  - highlight points when cursor is on the text area

- deployment
  - make rust serve the index.html
  - heroku https://elements.heroku.com/buildpacks/emk/heroku-buildpack-rust
  - should be $5 per month

- set sitting parameter through gui