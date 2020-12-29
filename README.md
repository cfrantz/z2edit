# Z2Edit 2.0

Z2Edit 2.0 is a re-write of Z2Edit in Rust.

This is a very experimental work-in-progress and is not in any way useful yet.

## Motivation

The Z2 Hackjam 2020 event demonstrated to me that Z2Edit 1.0 is not a
good tool for collaboration.  The project file maintains a single linear
history of ROM edits and sharing this file between team members requires
an agreed upon system of deciding who currently owns the file.  The single
linear history means that reverting a change means also discarding all
changes after the change you wish to revert.  The project file commits
consist of snapshots of the ROM contents at the time of the commit,
meaning the project file size grows quickly and becomes a problematic
large binary resource for version control systems.  Since the ROM is
Nintendo’s intellectual property, one cannot legally store the project
file in a publicly available version control system such as github.
Should one accidentally commit the file to version control, one then needs
to learn how to expunge the project file from version control history.

## Goals

- Configuration driven: the layout and contents of the ROM are enumerated in
  a configuration structure.  Advanced hacks which do significant re-arraging 
  of the ROM contents should not break the editor.
  
- The project file is an edit description list rather than a single linear
  binary history.

- Edits may be added, removed or re-arranged without adversely affecting
  unrelated edits.

- Python Integration: Z2Edit 1.0 has a command processor which makes writing
  any non-trivial script impossible.


## Development

- Rust
- SDL + Imgui (user interface)
- Ron (Rusty Object Notation for save files)
- PyO3 (for Python Integration)

### Remember to initialize submodules
```
git submodule update --init --recursive
```

### TODO

- Done (or at least a partial implementation)
    - 6502 Assembler
    - Console (python)
    - Emulate / Emulate at a location
    - Enemy Attributes
    - Experience & Spells
    - Enemy XP values and graphics
    - Import Rom
    - Misc Hacks (still need to add several common misc hacks)
    - Overworld editor
    - Palette Editor
    - Palace graphics banks and palettes (added to overworld connections)
    - Python script execution
    - Sideview editor
      - TODO: copy/clone policy for maps which point to the same address
    - Start Values
    - Text Table
    - Apply default fixes (move maps and code out of keepout regions)
    - Zelda 2 config updates & alternate configs
    - Remember the location of the vanilla rom
    - Periodic automatic save
    - Remember project association with exported ROMs
    - CHR viewer / importer.

- Not Started
    - Drops (drop probabilities, hidden drops, pbag values)
    - Item Effects
    - Tile transforms
    - Metatile Editor (aka "Object table" in Z2Edit 1).
    - Multimap viewer
    - Help documentation

- Other Features
    - Automate recovery from autosave and export-save files.
    - IPS Patch creator

- Cleanups
    - Refactor some of the GUIs into common code
    - Add common methods to the `Edit` struct
    - Maybe add menubars to individual editor GUIs
    - Relative-ize paths to location of project file (e.g. ImportChrBank)
    - Check on TileCache recreation after commits in Overworld & Sideview editors.
      - Create `Edit` instances as tentative commits.

### Cross Compile for Windows

```
$ windows/release.sh
```

#### Cross Compile Setup and Configuration

Started by following the [ArchLinux instructions](https://wiki.archlinux.org/index.php/Rust):

- Installed [mingw-w64-gcc-base](https://aur.archlinux.org/packages/mingw-w64-gcc-base/)
  (I'm not sure why I had to do this, just followed the instructions).
- Installed [mingw-w64-gcc](https://www.archlinux.org/packages/?name=mingw-w64-gcc).
- Installed wine.
- Installed rust toolchain and added target:
  - `rustup toolchain install stable-x86_64-pc-windows-gnu`
  - `rustup target add x86_64-pc-windows-gnu`
  - Edited my `.cargo/config`:

    ```
    [target.x86_64-pc-windows-gnu]
    linker = "/usr/bin/x86_64-w64-mingw32-gcc"
    ar = "/usr/x86_64-w64-mingw32/bin/ar"

    ```
- Deal with Python:
  - Download Python-3.8.6 embeddable ZIP and saved in `windows/bindist`.
  - Really should get a windows install of python includes, but I think the
    standard includes are the same between linux and windows.
  - Download Python-3.8.6 installer, extracted with cabextract.  Then searched
    the extracted bits for "python.lib", extracted that using `7z`, copied to
    `windows/python`.  Created the `libpython3.8.a` file using
    `windows/mk-python-archive.py`.
  - Set env variables when building:
    ```
    PYO3_CROSS_INCLUDE_DIR=/usr/include/python3.8/
    PYO3_CROSS_LIB_DIR=/home/cfrantz/src/z2e2/windows/python/`
    ```
- Deal with NFD:
  - NFD includes `ShObjIdl.h`, but the x86_64-w64-mingw32 include files are
    stored as lower-case and the linux filesystem is case-sensitive.
    Symlinked `ShObjIdl.h` to `shobjidl.h` in my `/usr/x86_64-w64-mingw32/include/`.
- Deal with SDL:
  - Downloaded the SDL2-devel mingw archive from https://www.libsdl.org/download-2.0.php.
  - Unpacked in /tmp
  - Copied libs to my local rust environment:
    ```
    cp -r /tmp/SDL2-2.0.12/x86_64-w64-mingw32/lib/* ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/
    ```
  - Zipped up `SDL2.dll` and saved in `windows/bindist`.
