> [!WARNING] 
> This repository is archived due to a security vulnerability in a dependency. To temporarily get around this, I will archive the repo.

# Simple Ray Tracer

## Installation
In this Rust version of my simple Ray Tracer, parallel\_ray, the project can be build and run with cargo run --release. Before this will succesfully run, Rust and sdl2 need to be installed on the system. Rust can be installed via official channels, in README.md you can read how to install sdl2 and how to fix a known issue on Windows.

## Configuration
I tested the versions on different scenes with and without auto vectorization on, in the script this happens automatically, but can be easily done manually by simply commenting out one of the two lines in .cargo/config. In src/lib.rs, a variable named FRAMES\_NR determines the amount of frames every scene will run and on line 174 the boolean passed last to the function determines whether it's multithreaded.

