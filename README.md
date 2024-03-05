This (currently, still nameless) project is going to be a simulator for tribes /
peoples moving around a procedurally generated map and generally interacting
with each other. This idea was something I wanted to do for a while, but was
heavily influenced by a YouTube Shorts video I saw on Hunnic migrations!  

This is also very much a Rust study project; before this, I had never dealt with
image generation, noise functions or multithreading, and I expect to deal with
other concepts moving forward.  

This idea is based on two modules of sorts - a map generator, which I am
currently working on, and the simulator itself, which will use a randomly
generated map.  

## Status

As of March 2024, I am still simply generating noise-based maps, without any
advanced image processing techniques. I did begin using
[nannou](https://github.com/nannou-org/nannou) in order to facilitate outputting
an image to screen, whereas before I saved each generated image instead of
displaying it.

[!'Example map in
1920x1080'](https://github.com/taernsietr/civ-sim/tree/master/examples/example-1.png)

Currently I plan on converting the code to run via WebGPU, in order to
facilitate faster iterations, then adding other features such as rivers. This
noise-based approach is still just a study path, however, as I intend to use
other methods to generate realistic, planet-spanning terrain.

## How to Use

If you want to try the project in its current shabby form, you'll need Rust
installed on your machine. The project itself can be run directly with `cargo
run`, which will use default values, but you can specify a bunch of stuff with
command line arguments (courtesy of the `clap` crate!). You can do `cargo run --
--help` to see what arguments are available so far. I haven't tested the code on
anything other than x86-64 Linux, however.

