> [!WARNING]
> This repository is archived due to a security vulnerability in a dependency. To temporarily get around this, I will archive the repo.

# Simple Ray Tracer

## Installation
In this Rust version of my simple Ray Tracer, parallel\_ray, the project can be build and run with cargo run --release. Before this will succesfully run, Rust and sdl2 need to be installed on the system. Rust can be installed via official channels, below you can find how to install sdl2 and how to fix a known issue on Windows.

## Installation
Follow the steps on https://github.com/Rust-SDL2/rust-sdl2 to install SDL2 on your system, after this the used Rust wrapper library should be working and compilation should succeed.

### Error on Windows 
On Windows I got a stack size exception, to fix that I had to increase the stack size. I used the following command to increase the stack size: 

```
Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.12.25827\bin\Hostx64\x64\editbin.exe' /STACK:2000000 'xxx\parallel_ray\target\release\main.exe'"
```
To do so, Visual Studio is required. I'm not sure if there's an option without using Visual Studio.

## Configuration
I tested the versions on different scenes with and without auto vectorization on, in the script this happens automatically, but can be easily done manually by simply commenting out one of the two lines in .cargo/config. In src/lib.rs, a variable named FRAMES\_NR determines the amount of frames every scene will run and on line 174 the boolean passed last to the function determines whether it's multithreaded.



