# Verifying feasibility

- Create a model with Fusion360
- save as STL, move to `model_preprocessing/models`
- prepare the pointcloud: `cd model_preprocessing && python preprocess.py models/<file.stl>`
- confirm that the pointcloud is properly scaled and centered
  - `cargo run ws --secondary model_preprocessing/models/<file.json>`
- handcraft a solution as a flow in `examples.rs`, register in macro `generate_get_example`
  - compare it to the pointcloud, adjust both so they fit:
    - `cargo run ws -l <example> --secondary model_preprocessing/models/<file.json>`
- adjust parameters for the simulation
  - select a parameter set from `src\plushie\params.rs`
    - if need to create a new one, register it in `generate_get_handpicked`
  - `cargo run ws -l <example> -m <paramset> --secondary model_preprocessing/models/<file.json>`
  - click `Animate`, the handcrafted thing must fit the pointcloud AND STOP simulating
  - most notable: max relaxing iterations and max tension


- adjust evolver
  - number of actions
  - genetic parameters
- start ranking server: `cargo run --release rank -g model_preprocessing/models/<file.json> -p <paramset>`
  - release is significantly faster
- start the evolution in `evolver`
- inspect the population with `cargo run inspect ...`