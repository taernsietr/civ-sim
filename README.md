**Hey!**

This project is going to be a simulator for tribes / peoples moving around a
procedurally generated map and generally interacting with each other. This idea
was something I wanted to do for a while, but was heavily influenced by a
YouTube Shorts video I saw on Hunnic migrations!  

This is also very much a Rust study project; before this, I had never dealt with
image generation, noise functions or multithreading, and I expect to deal with
other concepts moving forward.  

This idea is based on two modules of sorts - a map generator, which I am
currently working on, and the simulator itself, which will use a randomly
generated map.  

## Running (as-is)

If you want to try the project in its current shabby form, you'll need Rust
installed on your machine. The project itself can be run directly with `cargo
run`, which will use default values, but you can specify a bunch of stuff with
command line arguments (courtesy of the `clap` crate!). You can do `cargo run
-- --help` to see what arguments are available so far.  

## Ideas

- Progress bar for generation
- Knobs file (for controlling parameters)
- Multiple algorithms to determine different world features
- Post-noise-based generation processing and feature adding
- Voronoi division for area desirability? (greater concentration of cells in more desirable zones)

