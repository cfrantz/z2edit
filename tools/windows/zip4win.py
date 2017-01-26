#!/usr/bin/env python
#############################################################################
#
# zip4win.py - Create ZIP archive with binary, runfiles and DLLs
#
#############################################################################
import argparse
import errno
import os
import os.path
import subprocess
import sys
import zipfile

flags = argparse.ArgumentParser(description='Windows Cross Compiler Downloader')
flags.add_argument('files', metavar='FILE', type=str, nargs='*',
                   help='Files')
flags.add_argument('--out', default=None,
                   help='Output ZIP file.')
flags.add_argument('--mxe', default=None,
                   help='Location of MXE install.')
flags.add_argument('--target', default='win64',
                   help='Target platform (win32 or win64).')
flags.add_argument('--objdump_bin', default=None,
                   help='objdump binary')

TARGET = {
    'win32': 'i686-w64-mingw32.shared',
    'win64': 'x86_64-w64-mingw32.shared',
}

PREFIX = {
    'pei-i386': 'i686-w64-mingw32.shared',
    'pei-x86-64': 'x86_64-w64-mingw32.shared',
}

SKIP_DLLS = [
    'msvcrt.dll',
    'KERNEL32.dll',
    'USER32.dll',
    'IMM32.dll',
    'OPENGL32.dll',
]

REQUIRED_DLLS = {
    'win32': [
        'libgcc_s_sjlj-1.dll',
    ],
    'win64': [
        'libgcc_s_seh-1.dll',
    ],
}

def guess_exe(filename):
    with file(filename, 'r') as f:
        hdr = f.read(256)
        if hdr[:2] == 'MZ' and 'This program cannot be run in DOS mode' in hdr:
            return True
    return False

def guess_dlls(args, f):
    peformat = None
    dlls = []
    cmd = [args.objdump_bin, '-p', f]
    data = subprocess.check_output(cmd)
    for line in data.splitlines():
        if 'file format ' in line:
            peformat = line.split()[-1]
        elif 'DLL Name: ' in line:
            dll = line.split()[-1]
            if dll in SKIP_DLLS:
                continue
            dll = os.path.join(args.mxe, 'usr', PREFIX[peformat], 'bin', dll)
            dlls.append(dll)

    for dll in REQUIRED_DLLS[args.target]:
        dll = os.path.join(args.mxe, 'usr', PREFIX[peformat], 'bin', dll)
        dlls.append(dll)

    return set(dlls)

def pack_zip(args):
    dlls = set()
    files = []
    for f in args.files:
        if guess_exe(f):
            dlls.update(guess_dlls(args, f))
            files.append((f, True))
        else:
            files.append((f, False))

    with zipfile.ZipFile(args.out, 'w', zipfile.ZIP_DEFLATED) as z:
        for f, exe in files:
            if exe:
                bname = os.path.basename(f)
                z.write(f, bname + '.exe')
            else:
                z.write(f)

        for d in dlls:
            z.write(d, os.path.basename(d))


if __name__ == '__main__':
    args = flags.parse_args(sys.argv[1:])
    if args.objdump_bin == None:
        args.objdump_bin = os.path.join(args.mxe, 'usr', 'bin',
                                    TARGET[args.target] + '-objdump')
    pack_zip(args)
