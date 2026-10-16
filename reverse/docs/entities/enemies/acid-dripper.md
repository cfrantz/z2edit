# Acid Dripper

A spout in the ceiling that drips acid every 64 frames.  One drop in
eight becomes a Bot where it lands.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 1, 2, 5](../palaces.md) | $0D | Acid Dripper | 250 | 0 | 0 | - |
| [Palaces 3, 4, 6](../palaces.md) | $0D | Acid Dripper | 2 | 0 | 0 | - |

It is placed in rooms $10, $11, $13, $14, $1B, $27, $2D, $2F, $39 and
$3B of palaces 1, 2 and 5, and $33 of palaces 3, 4 and 6.  It blocks
Link's projectiles, is immune to SPELL and Thunder, and the sword passes
through it.

## Behavior

`AcidDripper_Ai` draws it, and every 64 frames (`enemyAiState` counts)
drips a drop of acid (projectile type 8) from X + 4, Y + 18, falling from
rest.  Nothing else: it isn't tested for hits.

**A drop** (`Projectile_AcidDrop_Bank4`) falls under gravity of 1/16.  Its
`projectileTimer` holds `rng+1,x` & 7, which marks it:

- **0 (1 in 8):** drawn in palette 3.  Where it hits the background it
  becomes a [Bot](bot.md), if an enemy slot is free
  (`Enemy_SpawnFlyer_Bank4`): 4 hit points, at the drop's X and its Y
  rounded down to the tile.  The Bot hops up and moves 1/2 pixel per
  frame toward Link.
- **Otherwise:** drawn in palette 2, and turns into a spark where it
  lands.

## Drawing

`AcidDripper_Draw`: a pair of sprites from `sramEnemyTiles` entry $18,
14 pixels below its Y.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `AcidDripper_Ai` | 4:$B9BF | dripping |
| `Projectile_AcidDrop_Bank4` | 4:$98F8 | the drop |
| `AcidDripper_Draw` | 4:$98EC | drawing |

## Verification

Scenarios in palace 1's first room (see [Bubble](bubble.md#verification),
with Link's life level raised), a dripper placed at X $1D0, Y $30, Link
standing at $1A0:

- It dripped every 64 frames, 23 times in 1500 frames, from X $1D4.
- Over three runs, with the dripper in slots 1, 2 and 3: 8 of 69 drops
  became Bots and the rest sparks.
- A new Bot hopped to Y $AF and walked toward Link at $F8.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=3 --after 200 --clear $PFLAT --approach 1A0 --spawn 3:0D:1D0:30 --projectiles \
    --event Drip=4:B9CF --event Bot=4:9916 --event Spark=4:993C --frames 1500
```
