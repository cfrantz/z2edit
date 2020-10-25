# Z2Edit 2.0

Z2Edit 2.0 is a re-write of Z2Edit in Rust.

This is a very experimental work-in-progress and is not in any way useful yet.

## Introduction

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
