# Z2Edit Version 3

## Introduction

This is Z2Edit version 3, a ROM editor for the Nintendo game _Zelda II:
The Adventure of Link_.

Z2Edit allows you to edit the data within the Zelda 2 ROM, thus creating a
new adventure in the classic Zelda II engine.  The editor allows you
to edit the following items:

- CHR graphics banks (still wip)
- Drop probabilities
- Enemy properties
- Experience values, attack strength and spell costs
- Item properties
- Metatile objects (e.g. background graphics)
- Miscellaneous properties (various game delays and speeds).
- Overworld maps and connections
- Color Palettes
- Sideview maps & enemy placements (still wip)
- Game start properties
- Text tables (e.g NPC dialog)

In addition, the editor features a built-in 6502 assembler that can
programmatically apply code changes to the ROM.

Z2Edit saves its edit list as a plain-text JSON file, allowing for:

- Collaborative work on a ROM-hack without having to share intermediate
  ROM images.
- Public publishing of ROM-hack data without publishing ROM images.

## TODOs

- Enhance CHR graphics import/export
- Complete the sideview editor
  - Edit townspeople text IDs
  - Edit townspeople dialog activation bitmasks
  - Pack sideview maps, enemylists, etc back into the ROM
- Adjust widget sizing in the GUI
- Add a help system to the editor
- Write help documentation

## Project Development Setup

This project uses git submodules to import the source code for `imgui-rs`
and `pybind11`.  These two libraries are needed so that I can generate
Python bindings for imgui.

The majority of the source code is Rust which is compiled into a Python
module.  The main program is a small Python shell around the main editor
code.  The Python interpreter is used to provide an interactive command
shell and as a mechanism for extending the editor or adding custom code
to hacks.

```
### Initialize submodules:
$ git submodule update --init --recursive

### Create a python virtual environment:
$ python3 -m venv .venv

### Activate the virtual environment:
$ source .venv/bin/activate

### Install deps:
$ pip install -U -r python-requirements.txt
```

