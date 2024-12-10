# Crocheteer
- Turn 3D models into amigurumi/crochet patterns
  - [x] proof of concept
  - [ ] reliable automatic stopping of the simulation
  - [ ] load arbitrary stl as target
  - [ ] simulation on GPU (gotta go fast)
- Visualize amigurumi/crochet patterns as 3D models
  - [x] simulation + stuffing
  - [x] patterns with or without FO (fasten off)
  - [x] basic stitches: sc, inc, dec
  - [ ] stuffing only on parts of the shape
  - [x] patterns starting from a chain instead of a magic ring
  - [x] working on FLO/BLO (front loop only / back loop only)
    - [ ] inserting hook into arbitrary stitch instead of the next one
    - [ ] include yarn changes in the grammar
  - [x] chains as part of the shape

See more in [docs](./docs/Concept.md)

# Setup
Use vcpkg

Install vcpkg.
Install OpenBLAS via vcpkg:
powershell
Copy code
vcpkg install openblas
Make sure VCPKG_ROOT is set to your vcpkg installation directory:
powershell
Copy code
$Env:VCPKG_ROOT = "C:\path\to\vcpkg"
Ensure the target triplet (like x64-windows) matches what openblas-src expects.
With vcpkg in place, openblas-src should now detect and link against the system OpenBLAS.

# How to run
This manual assumes you already have Rust's `cargo` and node's `npm` installed.

1. Open a terminal in this project's root folder.
2. Run `cargo run -- ws` to start the simulation server
   - see `cargo run -- help` for other options
3. Open another terminal, navigate to `client`
4. Run `npm install`. You only have to do this once.
5. Run `npm run dev`. You should see a link like `http://localhost:5173/` in the terminal.
6. Open the link in your browser.
7. A brief summary of the GUI is provided as comments in the sample pattern.