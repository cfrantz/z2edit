# Bubble

A flaming skull that flies diagonally around the room, bouncing off
everything, and burns away Link's magic while it touches him.  The Great
Palace adds a large one that splits in two.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 1, 2, 5](../palaces.md) | $06 | Slow Bubble | 250 | 50 | 0 | - |
| [Palaces 1, 2, 5](../palaces.md) | $0E | Fast Bubble | 250 | 50 | 0 | - |
| [Palaces 3, 4, 6](../palaces.md) | $06 | Slow Bubble | 250 | 50 | 0 | - |
| [Palaces 3, 4, 6](../palaces.md) | $0E | Fast Bubble | 250 | 50 | 0 | - |
| [Great Palace](../great-palace.md) | $14 | Slow Bubble | 255 | 20 | 3 | - |
| [Great Palace](../great-palace.md) | $15 | Fast Bubble | 255 | 20 | 3 | - |
| [Great Palace](../great-palace.md) | $17 | Large Bubble | 255 | 0 | 3 | - |

They are common in both palace groups' rooms.  In palaces 1, 2 and 5
they come back after dying (flags bit 6).  Their 250 hit points make
them all but indestructible.  The spawn routines (`SlowBubble_Init`,
`FastBubble_Init`, and `SlowBubble_Init_Bank4B`, `FastBubble_Init_Bank4B`) set both velocities to $F0
(1 pixel per frame) or $E0 (2), replacing the spawn default.  So every
Bubble starts out flying up and to the left, whatever side Link is on.

In the Great Palace (`GpBubble_Ai`) they have 255 hit points, are
immune to Thunder and always come back.  Their spawn routines are
`GpSlowBubble_Init` and `GpFastBubble_Init`; the large bubble uses the
fast one.  Slow ones are placed in rooms $0A, $1D and $23, fast ones in
$04, $1C, $1E, $2F and $31, and large ones in $12, $1F, $20 and $33.

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

## The Large Bubble

`GpBubble_Ai` moves and bounces the Great Palace's bubbles the same way.
It adds one step, for the large bubble ($17), after the contact test:

- **Splitting.**  When its hit points fall below 192 (64 points of
  damage: three strokes at attack level 8, 32 at level 1), it doubles both
  its velocities and becomes a fast bubble ($15).
- It then puts a second fast bubble in the highest free enemy slot (if
  there is one), 16 pixels right of it and 16 below (17 if it was flying
  up: the carry from doubling its Y velocity).  The new bubble flies
  the same way across but the other way up or down.  Its Y velocity is the
  first's inverted without adding 1 (so $1F, not $20).
- Both get 255 hit points.  The large bubble's 0 experience is lost with
  it: each fast bubble is worth 20.

## Drawing

`Bubble_Draw`: a symmetric pair of sprites (`SpritePairTiles` entry $5A,
the right one mirrored), its palette changing every frame (`frameCounter`
bits 1-0).  `GpBubble_Draw` is the same.

`LargeBubble_Draw`: a flaming skull about 32 pixels across, of mirrored
pairs from entry $40, its palette changing every 2 frames.  Two rows are
drawn at its position, and a third 24 pixels higher only while the bubble
is on screen.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `SlowBubble_Init`, `FastBubble_Init` | 4:$98AA, 4:$98AE | spawn: $F0 or $E0 both ways |
| `Bubble_Ai` | 4:$99D1 | everything above |
| `Bubble_Draw` | 4:$B7B0 | drawing |
| `GpSlowBubble_Init`, `GpFastBubble_Init` | 5:$A127, 5:$A12B | Great Palace spawn |
| `GpBubble_Ai` | 5:$A075 | Great Palace, with the split |
| `GpBubble_Draw`, `LargeBubble_Draw` | 5:$962C, 5:$A133 | Great Palace drawing |

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

### Great Palace

The Great Palace scenarios start a game in its entrance (area 0,
location $36).  They dig out the steps so that Link walks along a floor
at Y $B0 up to X $D0 (`$GF`), and he stops at $B1:

```
GP=areaPrgBank=5,currentRegion=2,currentWorld=5,townCode=4,palaceCode=2,areaLocation=0x36,areaId=0,areaStartPage=0,areaEntrySide=0,prevRegion=2,statLevel+2=8,statContainers+1=8
GF="--fill 50:CF:80:70 --fill 50:CF:90:70 --fill 50:CF:A0:70"
```

A large bubble in slot 2 with its hit points set to 191 split on the
first frame:

- It became a fast bubble flying $E0/$E0.
- A second one appeared in slot 5, 16 pixels right and 17 down, with
  velocities $E0/$1F.
- Both had 255 hit points.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --after 200 \
    --clear $GF --approach B0 --spawn 2:17:90:50 --poke objXVelocity+3=0xF0 --poke objYVelocity+3=0xF0 \
    --poke enemyHp+2=0xBF --log 2,5 --frames 40
```

A slow bubble flying down-right into Link took a point of magic on every
other frame while it touched him (event at 5:$A087): 10 points from
frame 21 to 39.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --after 200 \
    --clear $GF --approach B0 --spawn 4:14:A0:98 --poke objXVelocity+5=0x10 --poke objYVelocity+5=0x10 \
    --poke statMeter=0x40 --event Drain=5:A087 --log 4 --frames 40
```

Screenshot: the large bubble is a big flaming skull; the small ones are
white or green skulls, their colors cycling.

