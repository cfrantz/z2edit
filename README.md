# z2edit

### Build for Linux

```
$ bazel build :main


$(bindir)/main <user-supplied-zelda2.nes>
```

### Build and Package for Windows (on Linux)

```
./tools/release_windows.sh
```

### Manual Build for Windows (on Linux)

1. Prepare the build environment

   ```
   ./tools/downloader.py --nowin32 compiler SDL2
   ```

2. Build

   ```
   bazel build --crosstool_top=//tools/windows:toolchain --cpu=win64 :z2edit
   ```

3. Package

   ```
   ./tools/windows/zip4win.py --mxe tools/mxe --out z2edit.zip bazel-bin/z2edit
   ```

   Take z2edit.zip to a windows machine.
