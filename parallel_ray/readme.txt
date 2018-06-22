Volg de stappen op https://github.com/Rust-SDL2/rust-sdl2 om SDL2 te installeren voor jouw systeem. Hierna zou het compileren gewoon moeten werken.

Op Windows kreeg ik een stack size exception, daarvoor moest ik de stack size van het main thread vergroten, hiervoor heb ik de commando's in win_assign_bigger_stack.txt gebruikt. 
Visual Studio moet daarvoor geloof ik wel geïnstalleerd staan. De precieze locatie hangt natuurlijk van jouw systeem af. Idem voor de lcoatie van het gecompileerde Rust programma.