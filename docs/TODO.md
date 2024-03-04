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

- websocket server/client
  - allow human readable patterns in textarea
  - display a pattern even when plushie was loaded from a file?

- handling bigger shapes
  - stuff gets squished at Y=0, once some of the centroids go up
  - need to unlock Y coord of origin point, then translate the whole shape for the sake of visualization