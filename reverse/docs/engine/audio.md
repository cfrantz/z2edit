# Sound

Bank 6 holds two sound engines.  Each frame the NMI maps bank 6 and runs
one of them (`Sound_Update` / `Sound_UpdateTitle` in bank 7):

- the **gameplay engine**, `Sound_Engine` ($9000), during the game;
- the **title engine**, `Sound_TitleEngine` ($8000), on the title and
  file screens (`Bank5_Nmi`).

Both use the RAM at $E0-$EF and $07DA-$07FF, each in its own way: the
gameplay engine's names are the primary ones in `src/ram.inc`, and the
title engine's are overlays at the end of it (`title*`).

Prior work: the bindingforce wiki's
[Music Engine Description](https://wiki.bindingforce.net/index.php/Music_Engine_Description).
Its description agrees with the code, except as noted below.

## Requests

Game code asks for sounds by setting bits; the engine clears the request
bytes after each frame.

| Byte | Gameplay engine | Title engine |
|------|-----------------|--------------|
| `musicRequest` ($EB) | song to start (one bit) | sound effect (`titleSfxRequest`) |
| `soundMute` ($EA) | nonzero: all channels off | song part to start (`titleSongRequest`) |
| `dmcRequest` ($E9) | DMC sample: 1 Link hurt, 2 Ganon's laugh | effect playing (`titleSfx`) |
| `sfxRequest` ($EC) | shared effect (pulse 1 or noise) | (note timers) |
| `sfxRequest+1` ($ED) | noise effect | |
| `sfxRequest+2` ($EE) | pulse 2 effect | |
| `sfxRequest+3` ($EF) | pulse 1 effect | |

The callers of each effect bit are listed in the headers of
`Sfx_UpdateDmc`, `Sfx_UpdateSq1`, `Sfx_UpdateSq2`, `Sfx_UpdateNoise` and
`Sfx_UpdateShared`.  While an effect plays it holds its channel
(`sq1Sfx`, `sq2Sfx`, `noiseSfx`, `sharedSfxOnSq1`, `sharedSfxOnNoise`)
and the music on that channel is not played.

## Gameplay Music

`Music_Update` starts the requested song (`Music_Start`) or plays the
current one (`Music_PlayNotes`).  Songs are bits of `musicPlaying`; each
world has its own table, chosen by `currentWorld`: `OverworldMusic`,
`TownMusic`, `PalaceMusic` (worlds 3-4), `GreatPalaceMusic` (world 5).
At the end of a song (`Music_NextSong`) song $01 goes on to $02; $02,
$04 and $08 repeat (and $40 in the Great Palace); the fanfare $10
returns to the song it interrupted (`musicResume`); others stop.

A world's table starts with 8 bytes, the offset of each song's phrase
list; a phrase list is a series of offsets of phrase records, ended by 0.
A phrase record is 6 bytes:

| Byte | |
|------|-|
| 0 | tempo: row offset in `NoteDurations` (`musicTempo`) |
| 1-2 | note data pointer (`musicNotesPtr`) |
| 3 | bass (triangle) offset |
| 4 | harmony (pulse 1) offset |
| 5 | drum (noise) offset; 0 = no drums |

The melody (pulse 2) starts at offset 0.  All four channels read the same
note data at their own offsets.  A note byte holds the pitch in bits 5-1
(an index into `NotePeriods`; 0 = rest) and the duration in bits 0, 7
and 6, from high to low (`Music_NoteDuration`), an index into the
tempo's row of `NoteDurations`.  A melody note of 0 ends the phrase; a
drum note of 0 loops the drums.  The drums play a fixed noise burst for
any nonzero pitch.

The pulses' volume follows `PulseEnvelope` (from step $2F, or $18 for
song $04) and their pitch wavers by 2 every 4 frames (`Sound_Vibrato`,
which never writes the timer's high byte).

Differences from the wiki: `$07E2` is pulse 1's envelope step and `$07E3`
pulse 2's (the wiki has them the other way round); bit 0 of a note is
the high bit of its duration index; the vibrato alternates on bit 2 of
the frames left (every 4 frames); the drum, bass, harmony and melody
"pointers" (`$07E8`-`$07EB`) are offsets from `musicNotesPtr`.

## DMC Samples

`Sfx_UpdateDmc` plays samples from `DmcSamples` (start, length pairs),
stored in bank 7 at $F300-$FB70: sample 1 is Link's cry, samples 2-7
Ganon's laugh, played one after another (`laughSample`).

## Title Music

The title song has five parts, played in turn by `TitleMusic_NextPart`:
$01 (the intro), $02, $04, $08, $10, then $02 again.  Bank 5 times the
title screen by `titleMusicPart`; see
[Title, file screens and ending](title-and-files.md).

`TitleMusic` starts with 8 offsets of phrase lists (parts 1-5; 6-8 are
empty), then 8-byte phrase records:

| Byte | |
|------|-|
| 0 | row offset in `TitleNoteLengths` |
| 1-2 | note data pointer (in `TitleNotes`) |
| 3 | triangle offset (0 = none) |
| 4 | pulse 1 offset (0 = none) |
| 5 | noise offset (0 = none), also its loop point |
| 6 | pulse 2 envelope offset in `TitleEnvelopes` |
| 7 | pulse 1 envelope offset |

Unlike the gameplay engine, lengths and pitches are separate bytes: a
byte >= $80 sets the channel's note length (low nybble, an index into
the phrase's row of `TitleNoteLengths`), and the others are pitches
(`TitlePeriods`; $02 is a rest).  In the intro, pulse 1 echoes pulse 2's
notes 8 frames later and both swell (`titleEcho`, `titleSq2Swell`).

The title engine's sound effects are the file screens' cursor sounds
(`TitleSfx_Update`).
