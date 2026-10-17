# Hidden Jar

An invisible object in a palace room.  A stroke of Link's sword turns it
into a red jar, or half the time into a red Iron Knuckle (in palaces 3,
4 and 6) or a red Fokka (in the Great Palace).

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 1, 2, 5](../palaces.md) | $05 | Hidden jar | 255 | 0 | 0 | - |
| [Palaces 3, 4, 6](../palaces.md) | $05 | Hidden jar or Iron Knuckle | 255 | 0 | 0 | - |
| [Great Palace](../great-palace.md) | $05 | Hidden jar or Fokka | 255 | 10 | 3 | 1 |

It is placed in rooms $02, $0E, $0F, $14, $1F, $2A, $2E, $32, $3A, $3B
and $3D of palaces 1, 2 and 5, and $00, $07, $09, $0F, $16, $19, $20,
$24, $28, $2B, $3B and $3E of palaces 3, 4 and 6, and $0A, $10, $11, $17,
$21, $22, $23, $2C and $34 (two) of the Great Palace.  It has no drawing
routine and no spawn routine.

## Behavior

`HiddenJar_Ai` (palaces 1, 2, 5), `HiddenJarOrIronknuckle_Ai` (3, 4,
6) and `HiddenJar_Ai_Bank5` (the Great Palace).  While it is on screen it is tested only against the sword
(`Enemy_CheckSwordHit`).  When a stroke meets it (`objCollision` bit 5),
it changes in place, 3 pixels to the right:

- **Red jar:** type $01 with kind $11 (`enemyAiState`), an
  [item](item.md).  It falls to the floor, and another stroke takes it
  (a magic jar can't be taken by touch).  In palaces 3, 4 and 6 this
  happens half the time.
- **Red Iron Knuckle** (palaces 3, 4, 6, the other half, `rng+1,x` bit
  7): type $19 with 24 hit points, stunned for 8 frames, then an
  ordinary [Iron Knuckle](iron-knuckle.md).
- **Red Fokka** (the Great Palace, the other half): type $19 with 48 hit
  points, stunned for 8 frames.  It keeps the jar's kind, $11, in
  `enemyAiState`.  In the Great Palace the jar is also stunned 8
  frames, and is given 1 hit point rather than 0.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `HiddenJar_Ai` | 4:$B83E | palaces 1, 2, 5 |
| `HiddenJarOrIronknuckle_Ai` | 4:$AF61 | palaces 3, 4, 6 |
| `HiddenJar_Ai_Bank5` | 5:$A521 | the Great Palace |

## Verification

Scenario in palace 1's first room (see [Bubble](bubble.md#verification),
with Link's life level raised), a hidden jar placed at X $1C4, Y $B0,
Link standing at $1A0 facing it:

- A standing stroke on frame 20 met it on frame 28: it became a red jar.
- The jar fell to Y $BD.  A crouching stroke took it on frame 61
  (`Item_Collect`).
- Placed at Y $A0 instead, too high, the strokes missed it.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=3 --after 200 --clear $PFLAT --approach 1A0 --spawn 0:05:1C4:B0 \
    --event Jar=4:B84B --event Collect=7:E771 --press 20:B:3 --press 60:DOWN+B:3 --frames 130
```

In palace 3's entrance (see [Flame](flame.md#verification) for `$P3` and
`$F3`), a hidden object placed at X $A8, Y $C0, in each enemy slot in
turn, Link at $92 facing it, struck by a standing stroke on frame 28:

- in slots 0 and 5 it became a red jar and fell;
- in slots 1-4 it became a red Iron Knuckle, knocked back to $B9, which
  started an attack within 16 frames (its stun ending, `enemyAiDir`).

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $P3,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=4 --after 200 --clear $F3 --approach 80 --spawn 1:05:A8:C0 \
    --event Jar=4:AF6E --press 20:B:3 --frames 45
```

In the Great Palace's entrance (see [Bubble](bubble.md#great-palace)
for `$GP` and `$GF`), a hidden object placed at X $B4, Y $B0, in each
enemy slot in turn, Link moved to $90 facing it, struck by a standing
stroke on frame 28:

- in slots 4 and 5 it became a red jar, which fell to Y $BD;
- in slots 0-3 it became a red Fokka at X $B8, with `enemyAiState` $11,
  which turned to face Link once its stun ended.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --after 200 \
    --clear $GF --approach B0 --link 90:B0 --spawn 0:05:B4:B0 --event Jar=5:A52E --press 20:B:3 --frames 45
```

