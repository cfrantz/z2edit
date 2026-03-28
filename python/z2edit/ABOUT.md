# About the `python` subdirectory

This subdirectory contains the python code for the z2edit editor.

- app.py: The main python shell which starts the application.
- assembler.py: A 6502 assembler and disassembler written in python and integrated into the editor to allow assembling code into a romhack project.
- debug.py: Some interactive debugging functions (like hexdump).
- emulator: A subdirectory with the python components of the NES emulator.
- experimental: Some experimental romhacks as python extensions to the editor.
- fix.py: Some "misc fixes" hacks to improve the Zelda 2 ROM layout for the editor.
- hacks: Some miscellaneous hacks that can be used by the editor.
- __main__.py: Main python program initialization.
- __init__.py: Module initialization.
- util.py: Some helpful python functions that can be employed by romhack projects.

You may ignore the `__pycache__` subdirectory or any files named `*.so`.  These files are a result of executing the program or from the "maturin" build system.
