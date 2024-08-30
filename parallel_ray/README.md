## Installation
Follow the steps on https://github.com/Rust-SDL2/rust-sdl2 to install SDL2 on your system, after this the used Rust wrapper library should be working and compilation should succeed.

## Error on Windows 
On Windows I got a stack size exception, to fix that I had to increase the stack size. I used the following command to increase the stack size: 

```
Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.12.25827\bin\Hostx64\x64\editbin.exe' /STACK:2000000 'xxx\parallel_ray\target\release\main.exe'"
```
To do so, Visual Studio is required. I'm not sure if there's an option without using Visual Studio.

