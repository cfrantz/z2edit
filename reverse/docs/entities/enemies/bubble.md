# Bubble

A flaming skull that flies diagonally around the room, bouncing off
everything, and burns away Link's magic while it touches him.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 1, 2, 5](../palaces.md) | $06 | Slow Bubble | 250 | 50 | 0 | - |
| [Palaces 1, 2, 5](../palaces.md) | $0E | Fast Bubble | 250 | 50 | 0 | - |
| [Palaces 3, 4, 6](../palaces.md) | $06 | Slow Bubble | 250 | 50 | 0 | - |
| [Palaces 3, 4, 6](../palaces.md) | $0E | Fast Bubble | 250 | 50 | 0 | - |

They are common in both palace groups' rooms.  In palaces 1, 2 and 5
they come back after dying (flags bit 6).  Their 250 hit points make
them all but indestructible.  The spawn routines (`SlowBubble_Init`,
`FastBubble_Init`, and `SlowBubble_Init_Bank4B`, `FastBubble_Init_Bank4B`) set both velocities to $F0
(1 pixel per frame) or $E0 (2), replacing the spawn default.  So every
Bubble starts out flying up and to the left, whatever side Link is on.

## Behavior

`Bubble_Ai`, after `Enemy_SkipAiIfStunned`:

1. It moves by both velocities, then `Enemy_UpdateWithBg` (probes,
   drawing, hit tests).
2. **Magic.**  While it touches Link (`objCollision` bit 4, tested every
   other frame), it takes a point of his magic (`statMeter`, down to 0),
   besides the contact damage (`Enemy_HurtLinkOnContact`).  It asks for
   the meters to be redrawn every frame (`hudRedraw` bit 7).
3. **Bouncing across.**  It turns around (`Enemy_TurnAround`) and steps
   back when either of its sprite columns is off the side of the screen,
   or there is a wall ahead.
4. **Bouncing up and down.**  It reverses its Y velocity and steps back
   when it is off the top or bottom of the screen, or meets a floor or a
   ceiling.

## Drawing

`Bubble_Draw`: a symmetric pair of sprites (`SpritePairTiles` entry $5A,
the right one mirrored), its palette changing every frame (`frameCounter`
bits 1-0).

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `SlowBubble_Init`, `FastBubble_Init` | 4:$98AA, 4:$98AE | spawn: $F0 or $E0 both ways |
| `Bubble_Ai` | 4:$99D1 | everything above |
| `Bubble_Draw` | 4:$B7B0 | drawing |

## Verification

Palace scenarios start a game in palace 1's first room (location $34)
with its steps cleared to a floor at row $B0 (`$PFLAT`):

```
PAT=areaPrgBank=4,currentRegion=0,currentWorld=3,townCode=4,palaceCode=0,areaLocation=0x34,areaId=0,areaStartPage=0,areaEntrySide=0,prevRegion=0
PFLAT="--fill 60:26F:80:70 --fill 60:26F:90:70 --fill 60:26F:A0:70"
```

A slow Bubble at (X $C0, Y $80) and a fast one at ($E0, $60), with their
velocities set as the spawn routines would:

- The slow one flew up-left at 1 pixel per frame.  With no ceiling over
  it, it went off the top of the screen and turned down at Y $F0.
- The fast one flew 2 pixels per frame.  It bounced off a ceiling at Y
  $48 and the floor at $B8, and turned at the screen's left edge (X $08).

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT --until currentWorld=3 --after 200 \
    --clear $PFLAT --spawn 0:06:0C0:80 --spawn 1:0E:0E0:60 --poke objXVelocity+1=0xF0 \
    --poke objYVelocity+1=0xF0 --poke objXVelocity+2=0xE0 --poke objYVelocity+2=0xE0 --frames 300
```
