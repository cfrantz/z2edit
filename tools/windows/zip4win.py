#!/usr/bin/env python2
#############################################################################
#
# zip4win.py - Create ZIP archive with binary, runfiles and DLLs
#
#############################################################################
import argparse
import errno
import logging
import os
import os.path
import subprocess
import struct
import sys
import zipfile

flags = argparse.ArgumentParser(description='Windows Cross Compiler Downloader')
flags.add_argument('files', metavar='FILE', type=str, nargs='*',
                   help='Files')
flags.add_argument('--ignore_missing_dlls', default=False, type=bool,
                   help='Ignore missing DLLs')
flags.add_argument('--objdump_bin', default=None,
                   help='objdump binary')
flags.add_argument('--out', default=None,
                   help='Output ZIP file.')
flags.add_argument('--mxe', default=None,
                   help='Location of MXE install.')
flags.add_argument('--skip_dlls', default=None,
                   help='DLL dependencies to ignore.')
flags.add_argument('--target', default=None,
                   help='Target platform (win32 or win64).')

TARGET = {
    'win32': 'i686-w64-mingw32.shared',
    'win64': 'x86_64-w64-mingw32.shared',
}

PREFIX = {
    'pei-i386': 'i686-w64-mingw32.shared',
    'pei-x86-64': 'x86_64-w64-mingw32.shared',
}

SKIP_DLLS = [
    'ADVAPI32.DLL',
    'GDI32.DLL',
    'IMM32.DLL',
    'KERNEL32.DLL',
    'MSVCRT.DLL',
    'OLE32.DLL',
    'OLEAUT32.DLL',
    'OPENGL32.DLL',
    'SHELL32.DLL',
    'USER32.DLL',
    'VERSION.DLL',
    'WINMM.DLL',
]

REQUIRED_DLLS = {
    'pei-i386': [
        'libgcc_s_sjlj-1.dll',
    ],
    'pei-x86-64': [
        'libgcc_s_seh-1.dll',
    ],
}

ARCH_TO_TARGET = {
    0x8664: 'win64',
    0x014c: 'win32',
}

missing_dlls = 0

def guess_exe(filename):
    with file(filename, 'r') as f:
        hdr = f.read(4096)
        if hdr[:2] == 'MZ' and 'This program cannot be run in DOS mode' in hdr:
            logging.info('Detected legacy DOS header in %r', filename)
            (pehdr,) = struct.unpack('<L', hdr[0x3c:0x40])
            (signature, arch) = struct.unpack('<LH', hdr[pehdr:pehdr+6])
            logging.info('Found PE header at %#x: sig=%08x arch=%04x',
                         pehdr, signature, arch)
            return ARCH_TO_TARGET.get(arch, 'unknown')
    return None

def guess_dlls(args, f):
    global missing_dlls
    peformat = None
    dlls = []
    logging.info('Inspecting %r', f)
    cmd = [args.objdump_bin, '-p', f]
    data = subprocess.check_output(cmd)
    for line in data.splitlines():
        if 'file format ' in line:
            peformat = line.split()[-1]
        elif 'DLL Name: ' in line:
            dll = line.split()[-1]
            if dll.upper() in SKIP_DLLS:
                continue
            dllpath = os.path.join(args.mxe, 'usr', PREFIX[peformat],
                                   'bin', dll)
            if os.path.isfile(dllpath):
                dlls.append(dllpath)
                dlls.extend(guess_dlls(args, dllpath))
            else:
                logging.warn('The file %r does not exist.  Add it to skip_dlls '
                             'if it is a standard windows DLL.', dll)
                missing_dlls = missing_dlls + 1

    for dll in REQUIRED_DLLS[peformat]:
        dll = os.path.join(args.mxe, 'usr', PREFIX[peformat], 'bin', dll)
        dlls.append(dll)

    return set(dlls)

def set_objdump(args):
    args.objdump_bin = os.path.join(args.mxe, 'usr', 'bin',
                                    TARGET[args.target] + '-objdump')

def pack_zip(args):
    global missing_dlls
    dlls = set()
    files = []
    for f in args.files:
        target = guess_exe(f)
        if target:
            if args.target == None:
                args.target = target
            elif target != args.target:
                logging.warn('Unexpected target for %r: %s (expected %s)',
                             f, target, args.target)

            set_objdump(args)
            dlls.update(guess_dlls(args, f))
            files.append((f, True))
        else:
            files.append((f, False))

    if missing_dlls and not args.ignore_missing_dlls:
        sys.exit(1)

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
    #logging.basicConfig(level=logging.INFO)
    args = flags.parse_args(sys.argv[1:])
    if args.out == None:
        logging.error('No output file specified (use --out)')
        sys.exit(2)

    if args.skip_dlls:
        SKIP_DLLS += args.skip_dlls.split(',')
    SKIP_DLLS = map(str.upper, SKIP_DLLS)

    pack_zip(args)
