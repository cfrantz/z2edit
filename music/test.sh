#!/bin/sh

ORIG="$1"
TEST="/tmp/output.nes"

bazel build music:test || exit
./bazel-bin/music/test "$ORIG"

xxd "$ORIG" > /tmp/orig
xxd "$TEST" > /tmp/test

colordiff -u /tmp/orig /tmp/test && echo "No diffs!"
