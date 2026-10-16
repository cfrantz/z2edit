# Falling Block

Blocks that fall from the ceiling of a palace room and pile up into
walls; an invisible generator drops one every 16 frames.

## Where It Appears

| Group | Type | Enemy | HP | Damage class |
|-------|------|-------|---:|-------------:|
| [Palaces 1, 2, 5](../palaces.md) | $08 | Falling Block Generator | 0 | 1 |
| [Palaces 1, 2, 5](../palaces.md) | $09 | Falling Block | 0 | 1 |
| [Palaces 3, 4, 6](../palaces.md) | $08 | Falling Block Generator | 0 | 1 |
| [Palaces 3, 4, 6](../palaces.md) | $09 | Falling Block | 0 | 1 |

The generator is placed in rooms $15 and $2B of palaces 1, 2 and 5 and
$03 of palaces 3, 4 and 6.  The overworld groups' types $08 and $09
point to the same addresses, which are empty space in their banks.
Neither type can be hurt (a block's 255 hit points are set when it is
dropped).

## Behavior

**The generator** (`FallingBlockGenerator_Ai`), while it is on screen,
every 16 frames:

- It looks for a free enemy slot from 5 down whose random byte
  (`rng+1,x`), rounded down to 16, is $60 or more.  A slot that fails is
  skipped, so some periods drop nothing.
- It puts a block (type $09) there at that X on page 1 ($160-$1F0, the
  page is fixed) and Y $1F.  The block is falling, with no list entry
  and 255 hit points.
- A block that starts inside a ceiling is despawned at once.

**A block** (`FallingBlock_Ai`), never stunned:

1. It falls under gravity of 2/16 pixel per frame per frame.
2. If it touches Link, it hurts him and shatters.  If the sword strikes
   it, it shatters too.  Shattering plays a sound and sinks the block a
   pixel per frame for 15 frames before it is removed.
3. When it lands it is removed with a sound and becomes part of the room.
   The area map's tile there becomes the breakable block
   (`SpecialTiles_Bank4+4`), and its 2×2 tiles ($BA, $BC over $BB, $BD)
   are queued for the nametable.  Later blocks land on top of it, so the
   blocks pile up into walls.

## Drawing

The blocks are drawn by `Enemy08_Draw` by their state: `SpritePairTiles`
entry $20 while falling, $22 or $24 while shattering.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `FallingBlockGenerator_Ai` | 4:$AB98 | the generator |
| `FallingBlock_Ai` | 4:$ABE9 | a block |
| `Enemy08_Draw` | 7:$EFD2 | drawing |

## Verification

Scenario in palace 1's first room (see [Bubble](bubble.md#verification)
for `$PAT` and `$PFLAT`), with its ceiling over page 1 cleared too,
Link at X $1A0, a generator placed at ($1A0, $40):

- Blocks were dropped every 16 frames when a slot's X qualified, at
  page-1 X $160-$1F0.
- They fell under gravity.  Those at the same X landed ever higher (Y
  $C4, then $B2, then $A0) as they stacked.
- One that fell on Link shattered (frame 124).
- Screenshot: grey stone stacks built up beside Link, blue blocks still
  falling.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT --until currentWorld=3 --after 200 \
    --clear $PFLAT --fill 100:1FF:00:70 ... --fill 100:1FF:60:70 --approach 1A0 --spawn 5:08:1A0:40 \
    --event Spawn=4:ABBB --event Shatter=4:AC19 --event Land=4:AC25 --frames 400
```
