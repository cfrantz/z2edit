#!/bin/sh

ORIG="$HOME/source/z2-music/z2.nes"
TEST="/tmp/output.nes"

bazel build music:test || exit
./bazel-bin/music/test ~/source/z2-music/z2.nes

xxd "$ORIG" > /tmp/orig
xxd "$TEST" > /tmp/test

colordiff -u /tmp/orig /tmp/test
