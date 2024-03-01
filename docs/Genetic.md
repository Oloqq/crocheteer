# Fitness function

## Prepare user's file for comparison
user
-> stl
-> `Shape`

## Genetic algorithm
-> pattern
-> plushie
-> relaxing/stuffing
-> points
-> `Shape`

## Shape
Shape must be a represantation that is easy to calculate fitness for.
Some options are:
- point cloud
  - comparison with kd-trees, octotrees or smth
  - this is the closest representation to a plushie
- volumetric representation
  - split the space into cubes, for each cube specify if it's in, out or on the edge
  - check matching in/out/edge states in the generated and original shape
  - might be less expensive
  - the "point cloud" is still needed for `Plushie`, so an efficient conversion scheme must be defined

# Evolutionary operators
## Per node (per stitch)
use a weighted index, it would make sense to switch up the weights depending on overall volume difference.

calculating volume of point cloud: https://www.atlantis-press.com/proceedings/icca-16/25847693

another approach: make the plushie have way too much volume at the start, and promote shrinking

### Mutation

### Duplication

### Removal

## Crossover
is it even worth implementing?



https://docs.rs/genevo/latest/genevo/
