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
| `soundMute` ($EA) | nonzero: the engine stops and all channels are off | song part to start (`titleSongRequest`) |
| `dmcRequest` ($E9) | DMC sample: 1 Link hurt, 2 Ganon's laugh | effect playing (`titleSfx`) |
| `sfxRequest` ($EC) | shared effect (pulse 1 or noise) | (note timers) |
| `sfxRequest+1` ($ED) | noise effect | |
| `sfxRequest+2` ($EE) | pulse 2 effect | |
| `sfxRequest+3` ($EF) | pulse 1 effect | |

The effects, confirmed by ear (each bit set alone during play) and by
the code; the callers of each are in the handlers' headers:

| Bit | `dmcRequest` ($E9) | `sfxRequest` ($EC), shared | +1 ($ED), noise | +2 ($EE), pulse 2 | +3 ($EF), pulse 1 |
|-----|-------------|-----------------|-------------|-------------|-------------|
| $01 | Link's grunt | Link dying | - | falling into a pit | - (ends the shared and DMC effects) |
| $02 | Ganon's laugh | sword denied | bridge crumbling | pulled into battle | item dropped |
| $04 | - | spell cast | block crumbling | leaving battle | enemy killed |
| $08 | - | flute | - | spell menu | sword chime (beam, item pickup) |
| $10 | - | flute (2) | enemy damaged | tink (shot deflected) | menu cursor |
| $20 | - | flute (3) | sword strike | walking in a swamp | elevator |
| $40 | - | Great Palace barrier | short crumble | - | low life beep |
| $80 | - | boss dying | sword strike | - | - |

`sfxRequest` also takes two values of its own, checked first: $C0, a
fairy appearing (and boarding the raft), and $60, a town event in bank 3
that draws tiles one at a time.

The songs (`musicRequest` bits) are per world (the ending uses the Great
Palace's table, with `currentWorld` 6); a song with an empty phrase list
stops the music:

| Bit | Overworld | Towns | Palaces | Great Palace |
|-----|-----------|-------|---------|--------------|
| $01 | main theme, from its intro | same | same | same |
| $02 | main theme | town theme | palace theme | Great Palace theme |
| $04 | battle | (1 phrase) | (24 phrases) | the ending (`Ending_DrawCurtainTop`) |
| $08 | battle, other key | (1 phrase) | boss (`Boss_UpdateIntro`) | the credits (`Ending_Wait`) |
| $10 | item fanfare | same | same | same |
| $20 | silence | silence | silence | Triforce |
| $40 | silence | silence | crystal fanfare | Dark Link battle (`DarkLink_Fight`) |
| $80 | silence | silence | silence | silence |

Bank 7's `Music_RequestSilence` requests $80; bosses and the crystal
spot request it to stop the music before their own.  While an effect plays it holds its channel
(`sq1Sfx`, `sq2Sfx`, `noiseSfx`, `sharedSfxOnSq1`, `sharedSfxOnNoise`)
and the music on that channel is not played.

## Sound Effects

Each effect has a frame counter and is stepped once a frame by its
channel's handler (`Sfx_UpdateSq1`, `Sfx_UpdateSq2`, `Sfx_UpdateNoise`,
`Sfx_UpdateShared`, `Sfx_UpdateDmc`).  The shared effects take pulse 1 or
the noise channel (`Sfx_StartSharedOnSq1`, `Sfx_StartSharedOnNoise`).  A
new shared effect replaces the playing one and silences the channel it no
longer uses.  Pulse 1 effect $01 (`Sfx_StopSq1AndShared`) ends the shared
and DMC effects and silences pulse 1 and the noise; the music takes
pulse 1 back the same frame.

Effects waver like the music (`Sq1_VibratoSfx`): the period's low byte is
the note's ±2, by bit 2 of the effect's frames left.  Sweeping effects
(the Great Palace barrier, two pulse 2 effects) move their pitch by a
fixed fraction of itself each frame (`Sfx_SweepStep32`, `16`, `8`), so
the barrier's period falls by 16, then 12, then 8.

These were checked by starting effects in a town room and logging the
APU writes (`tools/enemy_scenario.py --apu`, with `--poke` and
`--poke-at` to set the requests):

```
TA=areaPrgBank=3,currentRegion=0,currentWorld=1,townCode=0,palaceCode=0xfe,areaLocation=0x32,areaStartPage=0,areaEntrySide=0,prevRegion=0
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $TA,areaId=0x1f --until currentWorld=1 \
    --after 200 --clear --poke sfxRequest=0x60 --poke-at 3:sfxRequest=0xC0 --poke-at 12:sfxRequest=0x60 \
    --apu 4000,4002,4003,400C,400E --frames 30
```

The text letter ($60, noise) started.  On frame 3 the fairy ($C0, pulse
1) replaced it and set the noise volume to $10.  On frame 12 the text
letter came back and set pulse 1's to $10.  The fairy's period low byte
alternated $78 and $74 every 4 frames.

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
stored in bank 7 at $F300-$FB70: sample 1 is Link's cry (`DmcLinkHurt`),
samples 2-7 Ganon's laugh (`DmcLaugh1`-`DmcLaugh6`), played one after
another (`laughSample`).  The fifth part (241 bytes from $F9C0) overlaps
the sixth (from $FA80).  `DmcSamplesUnused` lists two more samples,
`DmcUnused1` and `DmcUnused2`, that no sound effect plays.

## Title Music

The title song has five parts, played in turn by `TitleMusic_NextPart`:
$01 (the intro), $02, $04, $08, $10, then $02 again.  Bank 5 times the
title screen by `titleMusicPart`; see
[Title, file screens and ending](../systems/title-and-files.md).

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
(`TitleSfx_Update`), played on both pulses with the same vibrato
(`TitleSq1_VibratoSfx`, `TitleSq2_VibratoSfx`).

These were checked by logging the APU writes in the idle title movie and
the file select movie.  In the intro, pulse 1 played each of pulse 2's
pitches 8 frames later ($6A, $70, $8D, $BD, ...).  The two pulses' peak
volume rose from 3 to 15 over the first 384 frames.  The name cursor's
period low byte went $9F+2, then $9D, as bit 2 of `titleSfxTimer` changed.
