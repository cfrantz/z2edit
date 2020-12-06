#!/bin/bash

set -e

RELEASE=${1:-release}

if [[ ${RELEASE} == "release" ]]; then
    CARGO_RELEASE="--release"
else
    CARGO_RELEASE=""
fi

PYO3_CROSS_INCLUDE_DIR=/usr/include/python3.8/
PYO3_CROSS_LIB_DIR=windows/python/
cargo build --target "x86_64-pc-windows-gnu" $CARGO_RELEASE

git_version=$(git describe)
# We skip SDL2.dll and python38.dll because they exist in
# the `combine_zips` zipfiles.
python windows/zip4win.py "target/x86_64-pc-windows-gnu/$RELEASE/z2e2.exe" \
    --out z2edit-${git_version}.zip \
    --combine_zips windows/bindist/python-3.8.6-embed-amd64.zip,windows/bindist/SDL2-2.0.12.zip \
    --skip_dlls SDL2.dll,python38.dll
