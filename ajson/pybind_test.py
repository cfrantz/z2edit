import argparse
import sys
import IPython
import termios
from threading import Thread

from ajson import python_test_lib

def main(args):
    IPython.embed()
    return 0

if __name__ == '__main__':
    sys.exit(main(sys.argv))
