# Verifying feasibility

- Create a model with Fusion360
- save as STL, move to `model_preprocessing/models/<model>.stl`
- prepare the pointcloud: `cd model_preprocessing && python preprocess.py models/<model.stl>`
- pointcloud should be saved in the same directory as `<model>.json`
- confirm that the pointcloud is properly scaled and centered
  - `cargo run ws --secondary model_preprocessing/models/<model>.json>`
- handcraft a solution as a flow in `examples.rs`
  - call it `<model>`
  -  register in macro `generate_get_example`
  - compare it to the pointcloud, adjust both so they fit:
    - `cargo run ws --preset <model>`
- adjust parameters of stuffing and autostopping the simulation
  - create a parameter set in `src\plushie\params.rs`
    - call it `<model>`
    - register it in `generate_get_handpicked`
  - `cargo run ws --preset <model>`
  - click `Animate`, the handcrafted thing must fit the pointcloud AND STOP simulating

- adjust evolver
  - number of actions
  - genetic parameters
- start ranking server: `cargo run --release rank -g model_preprocessing/models/<model.json> -p <paramset>`
  - release is significantly faster
- start the evolution in `evolver`
- inspect the population with `cargo run inspect ...`