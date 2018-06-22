#!/bin/bash

SETSTACKCOMMAND="'C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.12.25827\bin\Hostx64\x64\editbin.exe' /STACK:2000000 'c:\Users\Tobias van Driessel\Projects\Github\mcpd-rust\RayTrace\parallel_ray\target\release\main.exe'"
RUNCOMMAND="./target/release/main.exe > result" #$VERSION.txt"
RUNCOMMANDB="./target/release/main.exe > resultST_NOAVX.txt"
BUILDCOMMAND="cargo build --release"

#awk 'NR==174 {$0="    run_timings(&mut event_pump, &mut cam, &pool, &mut canvas, &mut texture, &mut screen_buffer, &primitives, &triangles, &lights, true);"} 1' src/lib.rs

#awk 'NR==174 {$0="    run_timings(&mut event_pump, &mut cam, &pool, &mut canvas, &mut texture, &mut screen_buffer, &primitives, &triangles, &lights, false);"} 1' src/lib.rs > src/lib_new.rs

SETST="sed -i '174s/.*/	   run_timings(\&mut event_pump, \&mut cam, \&pool, \&mut canvas, \&mut texture, \&mut screen_buffer, \&primitives, \&triangles, \&lights, false);/' src/lib.rs"

SETMT="sed -i '174s/.*/	   run_timings(\&mut event_pump, \&mut cam, \&pool, \&mut canvas, \&mut texture, \&mut screen_buffer, \&primitives, \&triangles, \&lights, true);/' src/lib.rs"

SETAVX="sed -i '2s/.*/rustflags = \[\"-C\", \"target-cpu=native\"\]/' .cargo/config && sed -i '3s/.*/#rustflags = \[\"-C\", \"no-vectorize-loops\", \"-C\", \"no-vectorize-slp\"\]/' .cargo/config"

SETNOAVX="sed -i '2s/.*/#rustflags = \[\"-C\", \"target-cpu=native\"\]/' .cargo/config && sed -i '3s/.*/rustflags = \[\"-C\", \"no-vectorize-loops\", \"-C\", \"no-vectorize-slp\"\]/' .cargo/config"


#First we do all the MT stuff AVX
eval $SETMT
eval $SETAVX
eval $BUILDCOMMAND
eval $SETSTACKCOMMAND
eval "${RUNCOMMAND}MT_AVX.txt"

#then NOAVX
eval $SETMT
eval $SETNOAVX
eval $BUILDCOMMAND
eval $SETSTACKCOMMAND
eval "${RUNCOMMAND}MT_NO_AVX.txt"

#Then ST AVX
eval $SETST
eval $SETAVX
eval $BUILDCOMMAND
eval $SETSTACKCOMMAND
eval "${RUNCOMMAND}ST_AVX.txt"

#Then ST NOAVX
eval $SETST
eval $SETNOAVX
eval $BUILDCOMMAND
eval $SETSTACKCOMMAND
eval "${RUNCOMMAND}ST_NO_AVX.txt"


#eval $SETAVX

#eval $RUNCOMMAND

#eval $BUILDCOMMAND

#eval $SETSTACKCOMMAND

#eval $RUNCOMMANDB
