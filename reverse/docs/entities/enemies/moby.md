# Moby

A bird that dives from the top of the screen to Link's height and then
flies straight across at him.  An invisible generator sends one every 128
frames.  No area of the game places the generator.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $1D | Moby Generator | 3 | 0 | 0 | - |
| [West Hyrule](../west-hyrule.md) | $1E | Moby | 3 | 2 | 0 | - |

Only bank 1's group has the types, and none of bank 1's enemy lists
(West Hyrule and Death Mountain) has a type $1D entry, so in the game as
shipped Mobys never appear.  The generator's spawn code also serves the
[walking Moblin](walking-moblin.md) generator, which is used.  Touching
a Moby drains experience (attribute bit 4).  The generator has the
generators' spawn routine (`Generator_Init_Bank1`), which removes any
other generator of the same type.

## Behavior

### The Generator

`MobyGenerator_Ai`: every 128 frames (`frameCounter` bits 6-0 all 0), if
one of the six enemy slots is free, it puts a Moby (type $1E) in it:

- 3 hit points, no list entry (`enemySramIndex` $FF);
- at Y $20, near the left or right side of the screen at random (X =
  `scrollX` + $30 or + $D0, `MobySpawnX`);
- diving at 2 pixels per frame (Y velocity $20) with no X velocity,
  facing Link (`Enemy_FaceLink`);
- with `enemyAiState`, `enemyAiVar` and the stun timer cleared.

It returns with the carry clear and the new slot in `temp0` when it
spawned one.  Called for the walking Moblin generator (type $21), it uses
the outer pair of positions, the screen's very edges.

### The Moby

`Moby_Ai`, after `Enemy_SkipAiIfStunned`:

1. While its Y is less than Link's (`objY`), it dives straight down.
2. Once level with him or lower, it takes 2 pixels per frame the way it
   faces (`MobySpeeds`), that is toward the side Link was on when it
   spawned, and stops falling (`Enemy_StopY`).  From then on it flies
   straight across, past Link and off the screen, where it is despawned
   like other enemies.
3. `Enemy_UpdateCommon` (no background) and `Enemy_HurtLinkOnContact`.

## Drawing

`Moby_Draw`: a pair of sprites facing its `objFacing`, flapping between
`sramEnemyTiles` entries $44 and $46 (tiles $D0, $D2 and $D0, $D4) every
8 frames.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `MobyGenerator_Ai` | 1:$9B31 | the generator (and the walking Moblins' spawn) |
| `MobySpawnX` | 1:$9B2D | $30, $D0 (Moby); $00, $F8 (walking Moblin) |
| `Moby_Ai` | 1:$9B94 | the bird |
| `MobySpeeds` | 1:$9B2A | by facing: $20 right, $E0 left (the bytes after a `jmp`) |
| `Moby_Draw` | 1:$9DF6 | drawing |

## Verification

Scenario (West Hyrule cave, area $2D; see the [Myu](myu.md#verification)
page for `$AT`), a Moby generator placed with Link at Y $80:

- Mobys were spawned at `frameCounter` $80 and $00 while slots were
  free.
- One dived from Y $20 at 2 pixels per frame and reached Y $80 after 50
  frames.  It then flew right at 2 pixels per frame, through Link and
  on.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --spawn 4:1D:200:30 --log 0,1,2,3 --event Spawn=1:9B48 \
    --event Level=1:9B9D --frames 600
```
