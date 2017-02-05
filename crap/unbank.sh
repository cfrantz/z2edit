#!/bin/bash

SKIP=16
SIZE=16384
ROM=${1:-zelda2.nes}
OUT=$(basename $ROM)

for i in `seq 0 7`
do
    OF="${OUT}.bank${i}"
    dd if=$ROM of=$OF bs=1 skip=$SKIP count=$SIZE
    SKIP=$((SKIP + $SIZE))
done

OF="${OUT}.chr"
dd if=$ROM of=$OF bs=1 skip=$SKIP
