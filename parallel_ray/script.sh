#!/bin/bash

SETSTACKCOMMAND="'C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.12.25827\bin\Hostx64\x64\editbin.exe' /STACK:2000000 'c:\Users\Tobias van Driessel\Projects\Github\mcpd-rust\RayTrace\parallel_ray\target\release\main.exe'"
RUNCOMMAND="./target/release/main.exe > resultST_AVX.txt"
RUNCOMMANDB="./target/release/main.exe > resultST_NOAVX.txt"
BUILDCOMMAND="cargo build --release"

eval $RUNCOMMAND

eval $BUILDCOMMAND

eval $SETSTACKCOMMAND

eval $RUNCOMMANDB
