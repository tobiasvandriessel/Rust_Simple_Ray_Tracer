Follow the steps on https://github.com/Rust-SDL2/rust-sdl2 to install SDL2 on your system, after this the used Rust wrapper library should be working and compilation should succeed.

On Windows I got a stack size exception, to fix that I had to increase the stack size. I used the following command to increase the stack size: 
To do so, Visual Studio is required. I'm not sure if there's an option without using Visual Studio.
