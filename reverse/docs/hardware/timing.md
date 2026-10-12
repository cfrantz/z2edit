# Timing and Frame Budget

The NES CPU runs 29,780 cycles per NTSC frame; vblank, when the PPU can
be written freely, is the first ~2,270 of them after the NMI.  Zelda II
does almost all its work in the NMI handler ([Main loop and
NMI](../architecture/main-loop-and-nmi.md)), so a frame's work is the
length of one game NMI.

## Order of the Game NMI

1. In vblank: disable NMI (PPUCTRL bit 7), turn rendering off, sprite DMA
   from `oamBuffer`, the attribute column (`Ppu_WriteAttrColumn`), the
   PPU strings (`Ppu_WriteStrings`), then the scroll and rendering back
   on.
2. Sound (`Sound_Update`, bank 6) and the controllers (`Joypad_Update`).
3. Unless paused: the timers, the RNG ([Random numbers](../systems/rng.md)),
   hide the sprites but sprite 0, and run the game for a frame
   (`Game_RunState`).
4. Re-enable NMI and return.

Because NMI stays disabled until the end, an NMI that runs past the next
vblank simply misses it: the game slows down (a lag frame) instead of
re-entering itself.

## The Status Bar Split

In the side view the status bar at the top of the screen uses its own
nametable and no scroll.  Sprite 0 is placed on the status bar; during
the frame the game polls PPUSTATUS for the sprite 0 hit and then sets the
playfield's nametable and scroll (in `Mode_SideView`; bank 0 has copies
in `Hud_SplitScreen` and `LevelUp_Update`).  The hit comes about 5,830
cycles after the NMI starts, some 31 scanlines into the picture, so
every side view frame spends the time until then (less the vblank work)
waiting.  Only the spells and the death check run before the split;
Link, the scroll and the enemies run after it.

## Measurements

Cycles from the NMI's start to its `rti`, by `gameMode`, over the first
30,000 frames of the 100% movie (`Zelda2_100AK_schmidtty.fm2`), with the
emulator's cycle counter.  The side view figures include the wait for
sprite 0.

| gameMode | Handler | Median | Maximum | Frames |
|---------:|---------|-------:|--------:|-------:|
| $05 | `Mode_Overworld` | 6,136 | 47,914 | 3,920 |
| $09 | `Mode_DrawArea` | 14,293 | 19,940 | 1,260 |
| $0A | `Mode_StartSideView` | 9,778 | 9,934 | 35 |
| $0B | `Mode_SideView` | 18,050 | 28,466 | 23,927 |
| $00 | `Mode_LoadWorld` | 93,824 | 96,595 | 5 |
| $03 | `Mode_IndexOverworldMap` | 527,387 | 546,801 | 9 |

- The side view, where the game spends most of its time, stays within
  the frame: its worst frame used 96% of it, and no side view frame
  lagged.
- The overworld usually takes a fifth of the frame, but some frames run
  over: 108 of its 4,027 frames lagged.
- Two modes are deliberately long: `Mode_LoadWorld` (about 3 frames) and
  `Mode_IndexOverworldMap` (about 18 frames, with the screen blank; the
  music keeps going through `Sound_UpdateSometimes`).
- Loading a side view area (`Area_Load`) runs on the main thread instead,
  with the NMI only playing sound (`NMI_MODE_SOUND_ONLY`).
