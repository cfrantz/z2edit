######################################################################
#
# Set up the python environment for Z2Edit.
#
######################################################################
import code
import pydoc
import sys

pydoc.pager = pydoc.plainpager
sys.stdout.orig_write = sys.stdout.write
sys.stdout.orig_write = sys.stdout.write

class PythonConsole(code.InteractiveConsole):
    """Create an Interpreter that the GUI's PythonConsole can use.

    This object captures stdout and stderr write functions so the
    data can be written into the PythonConsole GUI element.
    """
    def __init__(self, *args, **kwargs):
        code.InteractiveConsole.__init__(self, *args, **kwargs)
        self.outbuf = ''
        self.errbuf = ''
        sys.stdout.write = self.outwrite
        sys.stderr.write = self.errwrite

    def outwrite(self, data):
        self.outbuf += data

    def errwrite(self, data):
        self.errbuf += data

    def GetOut(self):
        data = self.outbuf
        self.outbuf = ''
        return data;

    def GetErr(self):
        data = self.errbuf
        self.errbuf = ''
        return data;


def CreatePythonConsole():
    return PythonConsole()
