# z2edit

`z2edit` is a ROM editor for *Zelda II: The Adventure of Link*.  Documentation
is available at [https://github.com/cfrantz/z2doc/wiki/Z2Edit](https://github.com/cfrantz/z2doc/wiki/Z2Edit).

## Building

Z2Edit is built with [bazel](http://bazel.io) on Linux.  The bazel workspace
should download all required dependencies.

#### Known build issues

On Ubuntu 16, the MIR libraries link with an older version of the protobuf
libraries, which causes a conflict when starting the program.  A workaround
is to not link with the `nfd` (native file dialog) library.  To do this,
comment out the references to `//external:nfd` in the `BUILD` and
`imwidget/BUILD` files.

This will, unfortunately, remove the file load/save dialogs.  You can still
load and save with the `load` and `save` commands in the Debug Console.

### Build for Linux

```
$ bazel build :main

$ ./bazel-bin/z2edit <user-supplied-zelda2.nes>
```

### Build and Package for Windows (on Linux)

Prepare the build environment (e.g. download the cross compiler).  You only
need to do this once:

```
$ ./tools/downloader.py --nowin32 compiler SDL2
```

Build and package:

```
$ ./tools/release_windows.sh
```

Take z2edit.zip to a windows machine.
