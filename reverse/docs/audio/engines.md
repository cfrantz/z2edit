# Sound

Bank 6 holds two sound engines and all their data.  Each frame the NMI
maps bank 6 and runs one of them (`Sound_Update` / `Sound_UpdateTitle` in
bank 7):

- the **gameplay engine**, `Sound_Engine` ($9000), during the game;
- the **title engine**, `Sound_TitleEngine` ($8000), on the title and
  file screens (`Bank5_Nmi`).

Both use the RAM at $E0-$EF and $07DA-$07FF, each in its own way: the
gameplay engine's names are the primary ones in `src/ram.inc`, and the
title engine's are overlays at the end of it (`title*`).

Bank 6 has about 7.5K free (bytes of $FF): $878C-$8FFF (2164 bytes),
$9DA8-$9FFF (600) and $AC09-$BF6F (4967), so the engines and songs have
room to grow.

This page combines our reading of the code with the bindingforce wiki's
[Music Engine Description](https://wiki.bindingforce.net/index.php/Music_Engine_Description)
(by bgt and dontbagume; the site was down in October 2026, the
[May 2022 archive](http://web.archive.org/web/20220523083700/https://wiki.bindingforce.net/index.php/Music_Engine_Description)
has it).  Everything here was checked against the code, the ROM's tables
or APU logs; where the wiki is wrong, see [the last section](#corrections-to-the-wiki).

## The Frame

`Sound_Engine`, once a frame:

1. If `soundMute` is set (the overworld pause), turn every channel off
   (`SND_CHN` = 0) and do nothing else.
2. Set the frame counter (4-step, no IRQ).
3. Step the effects: `Sfx_UpdateDmc`, `Sfx_UpdateShared`,
   `Sfx_UpdateSq1`, `Sfx_UpdateSq2`, `Sfx_UpdateNoise`.
4. Play the music (`Music_Update`).
5. Clear the request bytes (`sfxRequest`-`sfxRequest+3`, `musicRequest`,
   `dmcRequest`).

`Sound_TitleEngine` is the same shape: the frame counter, all four tone
channels on, `TitleSfx_Update`, `TitleMusic_Update`, then clear its two
request bytes.

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
| $40 | - | Great Palace barrier | flame crackle | - | low life beep |
| $80 | - | boss dying | sword strike (lower) | - | - |

`sfxRequest` also takes two values of its own, checked first: $C0, a
fairy appearing (and boarding the raft), and $60, a town event in bank 3
that draws tiles one at a time.

The noise effect $40 is a single short burst (`Sfx_Crumble40`) with no
timer, requested every frame that a fire spell flame
(`LinkProjectile_PlaySound`) or a fireball (`Projectile_PlaySound_Bank1`,
`Projectile_Fireball_Bank2`, ...) flies.

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
spot request it to stop the music before their own.  A side view area's
song waits in `areaMusic` ($075F) and is sent to `musicRequest` when Link
appears.

### Who Wins

While an effect plays it holds its channel (`sq1Sfx`, `sq2Sfx`,
`noiseSfx`, `sharedSfxOnSq1`, `sharedSfxOnNoise`) and the music on that
channel is not played.  A shared effect beats the channel's own effects:
`Sfx_UpdateSq1` and `Sfx_UpdateNoise` give way while it has their
channel.

Within one channel, the handler tests the bits from $01 up, for each bit
either "requested" or "playing" first, and takes the first that is set.
So a lower bit wins: a request for a higher bit is dropped while a lower
effect plays, and a request for a lower bit cuts off a higher one.  When
"requested" is tested first, asking again restarts the effect; when
"playing" is first, it plays on:

| Channel | Restarts when asked again | Plays on |
|---------|---------------------------|----------|
| pulse 1 (`Sfx_UpdateSq1`) | $04, $08 | $02, $10, $20, $40 |
| pulse 2 (`Sfx_UpdateSq2`, `Sfx_UpdateSq2High`) | all ($01-$20) | |
| noise (`Sfx_UpdateNoise`) | $02, $04, $10, $20 | $80 |

The noise $40 is tested after "$20 playing" and before "$80 playing", and
does not set `noiseSfx`, so it sounds over the $80 strike for a frame.

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

Effects can use all of `NotePeriods`, including the pitches above $3E
that the music cannot reach (below): the sword chime (`Sfx_KillOrChime`)
alternates $62 (C7) and $54 (F6) every 4 frames.

### Noise Effects

The noise effects other than the bridge play *samples*: one byte per
step, the volume in bits 7-4 and the period in bits 3-0
(`Noise_PlaySample`), from a table chosen by the effect:

| Effect | Table | Frames | Step |
|--------|-------|--------|------|
| $04 block crumbling | `CrumbleNoise` ($912C) | 16 | every 2 frames |
| $10 enemy damaged | `HurtNoise` ($9123) | 16 | every 2 frames |
| $20, $80 sword strike | `SwordNoise` ($90E8) | 15 | every frame |

The $20 strike plays each period 2 lower (higher pitched) than $80.
`Sfx_NoiseStep` counts the frames down (`noiseSfxTimer`) and silences the
noise at the end.  The bridge ($02, `Sfx_CrumbleBridge`) is a 6-frame
noise burst plus a triangle note (pitch $0A), and the music's bass is
silent meanwhile.

### Checking Effects

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

### Songs

`Music_Update` starts the requested song (`Music_Start`), or plays a
frame of the current one (`Music_PlayNotes`), or does nothing.  Songs are
bits of `musicPlaying`; `Music_Start` turns the bit into an index
(`musicSong`, the bit number + 1) and starts at the first phrase
(`musicPhrase` = 0).  Song 0 stops the music (`Music_Stop`: triangle off,
pulses and noise silent).

Each world has its own table, chosen by `currentWorld`:
`OverworldMusic` (0), `TownMusic` (1-2), `PalaceMusic` (3-4),
`GreatPalaceMusic` (5 and up).  `Music_LoadPhrase` is four copies of the
same code, one per table.

At the end of a song (`Music_NextSong`) song $01 goes on to $02; $02,
$04 and $08 repeat (and $40 in the Great Palace); the fanfare $10
returns to the song it interrupted; others stop.  Starting the fanfare
over a song $01-$08 saves that song in `musicResume` ($07DB).

### Song Tables

A world's table starts with 8 bytes, the offset of each song's phrase
list; a phrase list is a series of offsets of phrase records, ended by 0.
All offsets are from the start of the table.  A phrase record is 6 bytes:

| Byte | |
|------|-|
| 0 | tempo: row offset in `NoteDurations` (`musicTempo`), a multiple of 8 |
| 1-2 | note data pointer (`musicNotesPtr`) |
| 3 | bass (triangle) offset (`bassOffset`) |
| 4 | harmony (pulse 1) offset (`harmonyOffset`) |
| 5 | drum (noise) offset (`drumOffset`, also `drumLoopOffset`); 0 = no drums |

`Music_StartPhrase` starts the melody (pulse 2) at offset 0 and sets all
four channels' timers to 1, so each reads a note on the first frame.

### Notes

All four channels read the same note data at their own offsets from
`musicNotesPtr`.  A note is one byte:

```
bit   7  6  5  4  3  2  1  0
      D1 D0 P4 P3 P2 P1 P0 D2
```

The pitch is the byte AND $3E, an even index into `NotePeriods`.  The
duration index is D2 D1 D0 (bit 0, bit 7, bit 6, from high to low;
`Music_NoteDuration`), a column in the tempo's row of `NoteDurations`.

- A melody byte of 0 ends the phrase (`Music_LoadPhrase` loads the next).
- A drum byte of 0 jumps back to `drumLoopOffset`, so the drums loop
  through the phrase.
- The bass and harmony have no end mark: they read on until the melody
  ends the phrase.  If their data is too short, they play whatever bytes
  follow.

Pitch $02 is a rest on the melodic channels (its period is 0 and
`Sound_PlayNote` writes nothing).  Pitch $00 is C3, but a melody byte
with pitch 0 and duration 0 is the end mark.  For the drums any nonzero
pitch plays the same burst (period $0A, volume 7, `Noise_Play`), and
pitch 0 is a rest (volume 0).

### Durations

`NoteDurations` ($914D) has six rows of eight durations in frames.  The
columns are musical lengths: the eighth note is column 2, and the others
follow from it, except column 0 (half an eighth, rounded down) and 6-7
(triplets); row $08's dotted eighth is 15 frames, not 13.5:

| Row | 0: 1/16 | 1: dotted 1/8 | 2: 1/8 | 3: 1/4 | 4: dotted 1/4 | 5: 1/2 | 6 | 7 | Used by |
|-----|---------|---------------|--------|--------|---------------|--------|---|---|---------|
| $00 | 4 | 12 | 8 | 16 | 24 | 32 | 5 | 6 | (nothing) |
| $08 | 4 | 15 | 9 | 18 | 27 | 36 | 6 | 6 | overworld battle ($04, $08) |
| $10 | 5 | 15 | 10 | 20 | 30 | 40 | 7 | 6 | palace boss, the ending, Dark Link |
| $18 | 6 | 18 | 12 | 24 | 36 | 48 | 8 | 16 | most songs |
| $20 | 7 | 21 | 14 | 28 | 42 | 56 | 19 | 18 | the towns' songs |
| $28 | 7 | 21 | 14 | 28 | 42 | 56 | (9) | (10) | the credits |

Columns 6-7 make triplet eighths (5+5+6 = 16 = a quarter in row $00)
except in row $18, where 8 is a triplet eighth and 16 a triplet quarter,
and row $20, where 19+19+18 = 56 is a triplet quarter.  Row $28 is row
$20 with triplet eighths; its last two entries are the first two bytes of
`DmcSamples` ($09, $0A), which happen to fit (9+9+10 = 28).  The credits'
melody uses neither.

### Pitches

`NotePeriods` ($918F) holds 2-byte timer periods (high byte first).
Its first 32 entries are the pitches the note data can reach; the rest
($40-$7A) only the sound effects use.  The names are for the pulse
channels; the triangle sounds an octave lower.

| Index | +$00 | +$02 | +$04 | +$06 | +$08 | +$0A | +$0C | +$0E |
|-------|------|------|------|------|------|------|------|------|
| $00 | C3 | rest | E3 | G3 | G#3 | A3 | A#3 | B3 |
| $10 | C4 | C#4 | D4 | D#4 | E4 | F4 | F#4 | G4 |
| $20 | G#4 | A4 | A#4 | B4 | C5 | C#5 | D5 | D#5 |
| $30 | E5 | F5 | F#5 | G5 | A5 | A#5 | B5 | C#3 |
| $40 | D3 | D#3 | F3 | F#3 | G#5 | C6 | C#6 | D6 |
| $50 | D#6 | E6 | F6 | F#6 | G6 | G#6 | A6 | A#6 |
| $60 | B6 | C7 | C#7 | D7 | D#7 | E7 | F7 | F#7 |
| $70 | G7 | G#7 | A7 | A#7 | B7 | C8 | | |

So the music has C3 to B5 but not D3, D#3, F3, F#3 or G#5.  Up to $48
the periods are within 15 cents of the note; above, up to 42 cents flat.

### Playing

`Music_PlayNotes` plays the melody (pulse 2), harmony (pulse 1), bass
(triangle) and drums (noise) in that order.  For each: count the timer
down; at 0 read the next byte, set the timer from its duration, and play
its pitch unless a sound effect holds the channel.

The pulses' volume follows `PulseEnvelope` ($9135), 24 values from $90
to $98 (duty 50%, constant volume 0-8).  A note sets the channel's step
(`sq1EnvelopeStep`, `sq2EnvelopeStep`) to $2F, or $18 for song $04; each
frame the step counts down and `PulseEnvelope[step / 2]` is written, so a
note starts at volume 8 (4 in song $04) and fades to 0 over 47 frames.
A rest does not restart the step.  While an effect holds a pulse, its
step is zeroed, so the music comes back silent until its next note.

The pulses' pitch wavers by 2 every 4 frames (`Sq1_Vibrato`,
`Sq2_Vibrato`, `Sound_Vibrato`): the period's low byte is the note's
(`sq1TimerLo`, `sq2TimerLo`) + 2 or - 2 by bit 2 of the frames left.  It
never writes the timer's high byte, which would restart the waveform and
click; no period's low byte is close enough to $00 or $FF for that to
matter.

The bass sets the triangle's linear counter before each note: $1F (about
8 frames, so the bass is staccato), or $60 (24 frames) in the fanfare.
It gives way only to the bridge's noise effect ($02).

## RAM

The gameplay engine's variables (the wiki's descriptions agree except
where noted in the last section):

| Address | Name | |
|---------|------|-|
| $E0-$E1 | `musicNotesPtr` | note data of the phrase |
| $E2 | `musicSong` | song number in the world's table (bit number + 1) |
| $E3 | `musicPhrase` | next entry of the phrase list |
| $E4 | `drumLoopOffset` | drums' loop point |
| $E5 | `musicTempo` | row offset in `NoteDurations` |
| $E6, $E7 | `sq1TimerLo`, `sq2TimerLo` | the notes' period low bytes, for the vibrato |
| $E8 | `titleMusicPart` | scratch for `Sound_Vibrato` |
| $E9-$EF | | requests (above) |
| $0707 | `currentWorld` | chooses the song table |
| $075F | `areaMusic` | song for the area being entered |
| $07DA | `laughSample` | Ganon's laugh sample playing |
| $07DB | `musicResume` | song to resume after the fanfare |
| $07DC-$07DE | `sweepUp`, `sharedSfxLockout`, `sweepStep` | effects' pitch sweeps and lockout |
| $07DF, $07E0 | `sharedSfxOnSq1`, `sharedSfxOnNoise` | a shared effect holds pulse 1 / the noise |
| $07E1 | `sharedSfx` | shared effect playing |
| $07E2, $07E3 | `sq1EnvelopeStep`, `sq2EnvelopeStep` | pulse 1's and pulse 2's envelope steps |
| $07E4-$07E7 | `drumTimer`, `bassTimer`, `harmonyTimer`, `melodyTimer` | frames left of each channel's note |
| $07E8-$07EB | `drumOffset`, `bassOffset`, `harmonyOffset`, `melodyOffset` | each channel's next note, an offset from `musicNotesPtr` |
| $07EC | `laughSampleTimer` | frames left of the laugh's sample |
| $07ED, $07EE | `dmcEnableTimer`, `dmcTimer` | DMC effect timers |
| $07EF-$07F3 | `sfxPitchLo` ... `sharedSfxTimer` | shared effects' pitch, sweep and step |
| $07F5 | `noiseSfxTimer` | frames left of the noise effect |
| $07F6, $07F7 | `sq2SfxPitch`, `sq2SfxTimer` | pulse 2 effect |
| $07F9 | `sq1SfxTimer` | frames left of the pulse 1 effect |
| $07FA | `dmcSfx` | DMC effect playing |
| $07FB | `musicPlaying` | song playing (one bit) |
| $07FC | `sharedSfxRecent` | a shared effect started recently |
| $07FD, $07FE, $07FF | `noiseSfx`, `sq2Sfx`, `sq1Sfx` | effect playing on the noise / pulse 2 / pulse 1 |

## Routines

The wiki's routine names, by address, and ours:

| Address | Wiki | Here |
|---------|------|------|
| $9000 | Main Music Loop | `Sound_Engine` |
| $9031, $9038 | Configure Pulse1 / Pulse2 Channel | `Sq1_SetEnvelope`, `Sq2_SetEnvelope` |
| $9042, $9044, $9067, $906B | Play Note Pulse1 / Play Note / Pulse2 / Triangle | `Sq1_PlayNote`, `Sound_PlayNote`, `Sq2_PlayNote`, `Tri_PlayNote` |
| $906F | Get Note Duration | `Music_NoteDuration` |
| $9089, $9097, $909E | Vibrato Pulse1 / Pulse2 / Vibrato | `Sq1_Vibrato`, `Sq2_Vibrato`, `Sound_Vibrato` |
| $920B | Play E9 SFX | `Sfx_UpdateDmc` |
| $92F4 | Play EF SFX | `Sfx_UpdateSq1` |
| $9408 | Play EE SFX | `Sfx_UpdateSq2` |
| $956C, $9587 | Crumble Bridge SFX / SFX Noise Decay | `Sfx_CrumbleBridge`, `Sfx_NoiseStep` |
| $959D | Play Noise | `Noise_Play` |
| $95A7 | Play ED SFX | `Sfx_UpdateNoise` |
| $95E6, $95EE | Crumble Block / Enemy Hurt SFX (Decay) | `Sfx_CrumbleOrHurt`, `Sfx_CrumbleOrHurtStep` |
| $9604 | Sword Strike SFX | `Sfx_SwordStrike` |
| $9612 | Play Noise Samples | `Noise_PlaySample` |
| $990B | Play EC SFX | `Sfx_UpdateShared` |
| $9B18 | Play Music | `Music_Update` |
| $9B24 | Next Song | `Music_NextSong` |
| $9B3B | Mute All | `Music_Stop` |
| $9B61 | Load Song | `Music_Start` |
| $9B80 | Load Song Data | `Music_LoadPhrase` |
| $9C87 | Play Notes | `Music_PlayNotes` |

## DMC Samples

`Sfx_UpdateDmc` plays samples from `DmcSamples` (start, length pairs),
stored in bank 7 at $F300-$FB70: sample 1 is Link's cry (`DmcLinkHurt`),
samples 2-7 Ganon's laugh (`DmcLaugh1`-`DmcLaugh6`), played one after
another, 23 frames apart (`laughSample`), except that sample 3
(`DmcLaugh2`) is skipped: the counter starts at 3 and is counted up
before each next sample (QUIRK; a scripted game over started samples
$CF, $DC, $E1, $E7 and $EA).  The fifth part (241 bytes from $F9C0) overlaps
the sixth (from $FA80).  `DmcSamplesUnused` lists two more samples,
`DmcUnused1` and `DmcUnused2`, that no sound effect plays.

## Title Music

### Parts

The title song has five parts, played in turn by `TitleMusic_NextPart`:
$01 (the intro), $02, $04 (a build-up), $08 (the main section), $10 (a
breakdown), then $02 again.  Bank 5 times the title screen by
`titleMusicPart`: the title scrolls into view during part 2, and the
story starts with part 8 (see
[Title, file screens and ending](../systems/title-and-files.md)).
A request for $80 (`Title_CheckStart`) has an empty phrase list and stops
the music.

### Song Table

`TitleMusic` ($84DA) starts with 8 offsets of phrase lists (parts 1-5;
6-8 are empty), then phrase lists, then 8-byte phrase records:

| Byte | |
|------|-|
| 0 | row offset in `TitleNoteLengths` (`titleDurationBase`; 0 in every record) |
| 1-2 | note data pointer (in `TitleNotes`) |
| 3 | triangle offset (0 = none) |
| 4 | pulse 1 offset (0 = none) |
| 5 | noise offset (0 = none), also its loop point |
| 6 | pulse 2 envelope offset in `TitleEnvelopes` |
| 7 | pulse 1 envelope offset |

Pulse 2 (the melody) starts at offset 0.

### Notes

Unlike the gameplay engine, lengths and pitches are separate bytes:

- A byte >= $80 sets the channel's note length (low nybble, an index
  into the phrase's row of `TitleNoteLengths`) for the notes after it.
- A byte of 0 ends the phrase (pulse 2) or loops the noise.
- Any other byte is a pitch, an index into `TitlePeriods`; $02 is a rest.

With row 0, the lengths are:

| Byte | Frames | | Byte | Frames | |
|------|--------|-|------|--------|-|
| $80 | 8 | 1/16 | $88 | 11 | triplet 1/8 |
| $81 | 24 | dotted 1/8 | $89 | 10 | triplet 1/8 (11+11+10 = 32) |
| $82 | 16 | 1/8 | $8A | 80 | 1/2 + 1/8 |
| $83 | 32 | 1/4 | $8B-$8E | 256 | two whole notes |
| $84 | 48 | dotted 1/4 | $8F | 6 | dotted 1/32 |
| $85 | 64 | 1/2 | | | |
| $86 | 96 | dotted 1/2 | | | |
| $87 | 128 | whole | | | |

`TitleNoteLengths` has only 11 entries; $8B-$8F read on into
`TitlePeriods` ($00, $00, $00, $00, $06), and a length of 0 counts down
256 frames.

`TitlePeriods` ($808F) is a plain chromatic scale, unlike `NotePeriods`:
2 per semitone, from $04 (C2) to $7E (C#7), with $46 = A4 (440 Hz on a
pulse).  For a MIDI note n the byte is 2 × (n − 34); $00 and $02 are
rests.  Its periods from $1C (C3) up are the same as `NotePeriods`'.

| | C | C# | D | D# | E | F | F# | G | G# | A | A# | B |
|-|---|----|---|----|---|---|----|---|----|---|----|---|
| 2 | $04 | $06 | $08 | $0A | $0C | $0E | $10 | $12 | $14 | $16 | $18 | $1A |
| 3 | $1C | $1E | $20 | $22 | $24 | $26 | $28 | $2A | $2C | $2E | $30 | $32 |
| 4 | $34 | $36 | $38 | $3A | $3C | $3E | $40 | $42 | $44 | $46 | $48 | $4A |
| 5 | $4C | $4E | $50 | $52 | $54 | $56 | $58 | $5A | $5C | $5E | $60 | $62 |
| 6 | $64 | $66 | $68 | $6A | $6C | $6E | $70 | $72 | $74 | $76 | $78 | $7A |
| 7 | $7C | $7E | | | | | | | | | | |

### Example

The main section (part 4, $08): its phrase list offset is $16, and at
$84DA + $16 the list is `3F 3F 00`, one phrase played twice.  The record
at $84DA + $3F is `00 3D 86 3A 1A 5F 00 00`: row 0, notes at $863D,
triangle at +$3A, pulse 1 at +$1A, noise at +$5F, envelopes 0.  The
melody:

```
83       quarter notes
02       rest
48       A#4
82 46    eighth: A4
83 3E 34 quarter: F4 C4
84 2E    dotted quarter: A3
83 30 34 3A 38 34 30
         quarter: A#3 C4 D#4 D4 C4 A#3
82 34    eighth: C4
83 30    quarter: A#3
85 2E    half: A3
82 02    eighth: rest
00       end
```

### Playing

The pulses' volumes come from `TitleEnvelopes` at each record's offsets,
and they waver like the gameplay engine's (`TitleSound_Vibrato`).  In the
intro, pulse 1 echoes pulse 2's notes 8 frames later and both swell
(`titleEcho`, `titleSq2Swell`, `titleEchoSwell`).  The noise plays
`TitleDrums`.

The title engine's sound effects are the file screens' cursor sounds
(`TitleSfx_Update`), played on both pulses with the same vibrato
(`TitleSq1_VibratoSfx`, `TitleSq2_VibratoSfx`); the pulses' music is
silent meanwhile.

These were checked by logging the APU writes in the idle title movie and
the file select movie.  In the intro, pulse 1 played each of pulse 2's
pitches 8 frames later ($6A, $70, $8D, $BD, ...).  The two pulses' peak
volume rose from 3 to 15 over the first 384 frames.  The name cursor's
period low byte went $9F+2, then $9D, as bit 2 of `titleSfxTimer` changed.

## Corrections to the Wiki

Where the wiki (May 2022) disagrees with the code or the ROM:

- Its RAM list has $07E2 as pulse 2's envelope step and $07E3 as pulse
  1's; it is the other way round (its Play Notes section has it right).
- The envelope restarts at $18 for song $04, not $40.
- Load Song saves the interrupted song in $07DB (`musicResume`), not
  $07FB, and only when it is $01-$08.
- Noise effect $40 is not unused: flames and fireballs request it every
  frame.
- The title phrase record is 8 bytes, not the gameplay engine's 6: bytes
  6-7 choose the pulses' envelopes.
- Title pitch $46 is A4, not $4C; the wiki's title pitch table and its
  example name every note 2 semitones too low (its $48 "Ab4" is A#4).
  The last $82 in the example is an eighth note, not a quarter.
- Its title lengths are row 0's; $8B-$8E are 256 frames because they
  read $00 from `TitlePeriods`, not because they are duplicates.
- Its "TODO check what values are used": the songs use tempo rows $08 to
  $28, never row $00, and row $28's last two entries (which the wiki
  calls garbage) are `DmcSamples`' first bytes.
- The bass, harmony and drum "pointers" are offsets from the note data
  pointer, as its RAM list ("next note index") says.
