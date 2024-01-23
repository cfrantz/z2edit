import argparse
import sys
import IPython

from ajson.python import relax

def main(args):
    IPython.embed()
    return 0

if __name__ == '__main__':
    sys.exit(main(sys.argv))
