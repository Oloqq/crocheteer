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

- k-means
  - visualization of additional points in different colors
  - new stuffing enum variant
  - kmeans needs a weight system, where weight is dependent on distance between point and centroid, so that placing centroids on the wall is discouraged