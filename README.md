### WARNING: This repository is archived due to a security vulnerability in a dependency. To temporarily get around this, I will archive the repo.

In this Rust version of my simple Ray Tracer, parallel\_ray, the project can be build and run with cargo run --release. Before this will succesfully run, Rust and sdl2 need to be installed on the system. Rust can be installed via official channels, in readme.txt you can read how to install sdl2 and one other step: On Windows, the stack size of the program exceeds the maximum size, therefore some additional steps are described in win\_assign\_bigger\_stack.txt to allow for this bigger stack. On Linux, this problem is not encountered.

Using lilscript.sh, one can do the tests I did. As can be read in the report, I tested the versions on different scenes with and without auto vectorization on, in the script this happens automatically, but can be easily done manually by simply commenting out one of the two lines in .cargo/config. In src/lib.rs, a variable named FRAMES\_NR determines the amount of frames every scene will run and on line 174 the boolean passed last to the function determines whether it's multithreaded.

