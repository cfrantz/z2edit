#!/usr/bin/env python3
"""Write an FM2 movie from a list of button presses.

    make_movie.py OUT.fm2 --frames N [K:BUTTONS[:LEN] ...]

The movie starts at power-on and lasts N frames.  Each press holds
BUTTONS (any of A B SELECT START UP DOWN LEFT RIGHT, joined with +; a
"2/" prefix for controller 2) from frame K for LEN frames (default 2).
For recording runs through code the test movies never reach (the title
screen left alone, the file select menus), with tools/cdl_movie.py.
"""
import argparse

from play import parse_press

# FM2 writes each controller as 8 characters, RLDUTSBA from left to right.
LETTERS = "RLDUTSBA"


def pad_text(mask):
    return "".join(c if mask & (0x80 >> i) else "." for i, c in enumerate(LETTERS))


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("out")
    p.add_argument("--frames", type=int, required=True)
    p.add_argument("presses", nargs="*")
    args = p.parse_args()

    pads = [[0] * args.frames, [0] * args.frames]
    for spec in args.presses:
        k, pad, mask, n = parse_press(spec)
        for f in range(k, min(k + n, args.frames)):
            pads[pad][f] |= mask

    with open(args.out, "w") as f:
        f.write("version 3\nemuVersion 21050\npalFlag 0\n"
                "romFilename Zelda_2_-_The_Adventure_of_Link\n"
                "fourscore 0\nport0 1\nport1 1\nport2 0\n"
                f"length {args.frames}\n")
        for a, b in zip(*pads):
            f.write(f"|0|{pad_text(a)}|{pad_text(b)}||\n")


if __name__ == "__main__":
    main()
