# z2music

A minimal music encoder/decoder for Zelda II.

This library provides an API for decoding and encoding the music data stored in
Zelda II ROMs.  Currently, the only interactions with the music are
programmatic which makes it somewhat awkward to use.

## Classes

All of the data structures in are represented as C++ classes.  The following
classes exist:

### Rom

This class represents the entire ROM.  It has methods to read and write
arbitrary CHR or PRG data.  It can also read/write disk files in the standard
iNES header format.  Additionally, it has features for reading/writing specific
song data.

When writing modified song data back to the ROM, however, there is currently no
error checking to ensure that later song tables aren't overwritten.

### Song

This class represents a single song.  The songs are identified from the
`Rom::SongTitle` enum which looks up the specifically named song in the correct
table.  Each song is comprised of a list of patterns that play in order.  For
space effeciency a pattern can appear in this list of patterns multiple times.
Several times in the original game, music is stored in an ABAC or AABB pattern,
for example.

### Pattern

This class represents a single pattern element of a 'Song'.  Each pattern has
a tempo and four channels of note data.

### Note

This class represents a single note entry in a pattern's channel data.  A note
has a duration and pitch and is stored as a single byte.  These values are
encoded in a rather obtuse way, so enums are provided for convenience in
`Note::Pitch` and `Note::Duration`.

## Decoding

Here is an example of how to decode a theme and display the notes:

```
using std:
using z2music;

const Rom rom("/path/to/z2.nes");
const Song* house = rom.song(Rom::SongTitle::HouseTheme);

for (size_t i = 0; i < house->sequence_length(); ++i) {
  const Pattern* p = song.at(i);
  // Pulse2 usually contains the melody
  for (Note n : p->notes(Pattern::Channel::Pulse2)) {
    cout << n.pitch_string() << " ";
  }
  cout << endl;
}
```

Additional examples can be found in `music/main.cc` and `music/solstice.cc`.
