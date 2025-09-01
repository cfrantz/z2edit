######################################################################
# Midi note to frequency
######################################################################

# The frequency of concert A is 440 Hz
_A4 = 440
# We calculate the frequency of C-1 (midi note 0) by computing the
# frequency of C above A4 and then dividing down 6 octaves.
_Cminus1 = (_A4 * 2 ** (3 / 12)) / (2**6)


def frequency(note, bend=0):
    """Computes the frequency of a midi note number."""
    note = note * 100 + bend
    return _Cminus1 * 2 ** (note / 1200)
