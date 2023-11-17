# About

Island map procedural generation using Rust and Bevy engine.
The generator is based on Fractal Brownian motion.

# How to launch

Firstly, install [bevy dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)

Then clone this repository and launch :
~~~
cargo run --release
~~~

Note: if you want to launch as dev, make sure to add the following feature to decrease compilation time :
~~~
cargo run --features bevy/dynamic_linking 
~~~ 