#!/usr/bin/env python3

import sys
from pathlib import Path

progname = Path(sys.argv[0]).name

if progname == "emulator":
    from z2edit.emulator import app
else:
    from z2edit import app

app.main()
