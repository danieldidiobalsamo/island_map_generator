# About

Island map procedural generation using Rust and Bevy engine.
The generator is based on Fractal Brownian motion.

The following parameters can be updated while the simulation is running:
- texture resolution.
- octaves: islands amount of details.
- frequency: higher values create more islands.
- lacunarity: how many details remains for each octave.
- persistence: octaves contribution in the general result. Higher values produce "rough" islands.
- scale: multiplier to amplify the result.
- bias: allows to shift the result.
- map seed.

Press 'Q' to leave.

# How to launch

## Using cargo (recommended)

Install [bevy dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md) and then just launch:
~~~
cargo install island_map_generator_bevy
island_map_generator_bevy
~~~

## Build manually

Install [bevy dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)

Then clone this repository and launch :
~~~
cargo run --release
~~~

Note: if you want to launch as dev, make sure to add the following feature to decrease compilation time :
~~~
cargo run --features bevy/dynamic_linking 
~~~ 