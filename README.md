# Crocheteer
- Visualize amigurumi/crochet patterns as 3D models
  - [x] simulation + stuffing
  - [x] patterns with or without FO (fasten off)
  - [x] basic stitches: sc, inc, dec
  - [x] patterns starting from a chain instead of a magic ring
  - [x] working on FLO/BLO (front loop only / back loop only)
  - [x] inserting hook into arbitrary stitch instead of the next one
  - [x] include yarn changes in the grammar
  - [x] chains as part of the shape
  - [ ] multiple starting points (MR) that get connected later
    - [ ] stuffing only on parts of the shape
  - [ ] slip stitch
  - [ ] ...
- Turn 3D models into amigurumi/crochet patterns
  - [x] proof of concept (genetic algo is in git history)
  - [ ] reliable automatic stopping of the simulation
  - [ ] load arbitrary stl as target
  - [ ] simulation on GPU (gotta go fast)

This project was used as an engineering thesis (*Fabric Simulation for Three-Dimensional Crochet Designs*), the thesis serves as a good documentation of how the visualization works. The final pdf is going to be linked in the repo soon; see the draft in [docs](./docs/draft).

See sample patterns that Crochetter understands in [patterns](./patterns/).

# How to run
This manual assumes you already have Rust's `cargo` and node's `npm` installed.

1. Open a terminal in this project's root folder.
2. Run `cargo run dev` to start the simulation server
3. Open another terminal, navigate to `client`
4. Run `npm install`. You only have to do this once.
5. Run `npm run dev`. You should see a link like `http://localhost:5173/` in the terminal.
6. Open the link in your browser.
7. A brief summary of the GUI is provided as comments in the sample pattern.

If you want to play with skeletonization you also need to put a CloudCompare executable in [CloudCompare](./CloudCompare/) (see [CloudCompare\cloud_compare_normals.bat](CloudCompare\cloud_compare_normals.bat)).
