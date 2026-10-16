# Megmet

A small creature of the caves that hops toward Link without a pause,
high and fast.

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $1F | 4 | 5 | 1 | 1 |

Only bank 1's group has it: caves of West Hyrule (areas $0C, $15-$17,
$23-$25, $2C, $2E) and Death Mountain (areas $01, $10, $17 of the second
group).  It uses background probe set 6 and has no spawn routine
([spawn defaults](../../systems/enemies.md#spawning)).

## Behavior

`Megmet_Ai`, after `Enemy_SkipAiIfStunned`, using last frame's
background probes:

1. **Ceiling** (`objCollision` bit 3): it stops rising (`Enemy_StopY`) and
   moves down a pixel.
2. **Wall ahead** (bits 1-0): it turns around (`Enemy_TurnAround`), in the
   air as well as on the ground.
3. It moves by its X velocity.
4. **On a floor** (bit 2): it hops at once, with no pause and without
   being put down on the floor first.  The Y velocity is -4 or -5 *whole*
   pixels per frame at random (`MegmetYSpeeds`), and the X velocity 1/2
   pixel per frame toward Link (`MegmetXSpeeds`, by `Enemy_FaceLink`).
5. **Falling:** `Enemy_MoveYWithGravity` with gravity $30/256 pixel per
   frame per frame.  Its Y velocity is in whole pixels with an 8-bit
   fraction, so it changes by 1 every 5 or 6 frames.  The terminal
   velocity of $30 is never reached.
6. `Enemy_UpdateWithBg` and `Enemy_HurtLinkOnContact`.

A hop at -4 rises 51 pixels and lasts 47 frames; one at -5 rises about
75 and lasts about 58.  It covers some 24-29 pixels toward Link.  It
hops from wherever its fall crossed into the floor's row, a few pixels
deep, so its height on the ground varies.  In a pit it bounces between
the walls.

## Drawing

`Megmet_Draw`: a pair of sprites facing its `objFacing`, `sramEnemyTiles`
entry $40 (tiles $D6, $D8) while rising or falling slowly, $42 ($DA,
$DC) once falling at 2 pixels per frame or more.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Megmet_Ai` | 1:$987E | everything above |
| `MegmetXSpeeds` | 1:$987A | $08 right, $F8 left |
| `MegmetYSpeeds` | 1:$987C | $FB, $FC: -5, -4 pixels per frame |
| `Megmet_Draw` | 1:$9DFB | drawing |

## Verification

Scenario (West Hyrule cave, area $2D; see the [Myu](myu.md#verification)
page for `$AT`), Megmets placed on the floor at X $370 and $2C8:

- Both hopped on their first frame.  A hop at -4 rose from Y $90 to $5D
  in 21 frames and came down 47 frames after it began, 24 pixels toward
  Link.
- They hopped again at once on landing, at Y $90-$93.  In the pit at
  $340 they hopped from Y $A0-$A3 and turned at its walls in mid-air.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --spawn 0:1F:370:90 --spawn 1:1F:2C8:90 \
    --event Hop=1:98A4 --event Ceiling=1:9887 --event Wall=1:9892 --frames 400
```
