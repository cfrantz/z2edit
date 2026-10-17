# Fokkeru

A fire-breathing bird of the Great Palace.  It hops about at random,
paying Link no attention, and every 64 frames spits a fireball that
arcs over to land and burn on the floor.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Great Palace](../great-palace.md) | $1D | Fokkeru | 144 | 200 | 4 | - |

It is placed in rooms $02, $04, $08, $0B (two), $0E, $16 (two), $1D, $1F
and $2E.  Link's projectiles bounce off it and SPELL doesn't affect it.
It has no spawn routine.

## Behavior

`Fokkeru_Ai`:

1. **Moving.**  It moves by its X velocity unless at a wall, or by its
   knockback after a hit (`Enemy_KnockBackOrWalk_Bank5`).  Its knockback
   shrinks even in the air, unlike the Fokkas'.  It falls
   (`Enemy_MoveYAndFall`), and turns toward Link only to aim.
2. **Hopping.**  Each time it lands (`Enemy_Land`) it hops again.  It
   rises 1 or 2 pixels per frame (`rng+1,x` bit 0) and moves 1/4 or 1/2
   pixel per frame left or right (bits 2-1, `FokkeruSpeeds`), whatever
   Link does.
3. **Fireballs.**  Every 64 frames (`enemyAiState`) it spits a fireball
   (projectile type 7, `Enemy_FireProjectile`), 2 pixels per frame up
   and 1 toward Link (`FokkeruShotSpeeds`, doubled).  If all six
   projectile slots are busy, nothing happens.  Contact hurts Link
   (`Enemy_HurtLinkOnContact`).

**The fireball** (`Projectile_RollingFlame_Bank5`):

- It falls with gravity, 1/16 pixel per frame per frame, so it rises
  about 32 pixels and comes down 64 pixels away at its own height.
- When it meets the background it makes a sound, sits on the tile row,
  stops, and hops a little (1 pixel per frame up).
- Then 3 times in 4 it burns in place for 48 frames.  The 4th time it
  rolls toward Link (by its screen X against his) at 1 pixel per frame
  for 255 frames, slowing by 1/16 every 8 frames.
- It falls off ledges, and goes on unseen off the side of the screen.
- Link's shield stops it only with the reflect spell (flags $82); its
  damage class is 2.

## Drawing

`Fokkeru_Draw`: two pairs of sprites, its four frames (`SpritePairTiles`
entries $20, $1C, $28, $24) cycling every 8 frames by `enemyAiState`.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Fokkeru_Ai` | 5:$9E6B | everything above |
| `Enemy_KnockBackOrWalk_Bank5` | 5:$9E42 | moving, or knockback |
| `Projectile_RollingFlame_Bank5` | 5:$9BE1 | the fireball |
| `Fokkeru_Draw` | 5:$A05B | drawing |

## Verification

In the Great Palace's entrance (see [Bubble](bubble.md#great-palace) for
`$GP` and `$GF`), a Fokkeru placed at ($70, $80), Link standing at $B1:

- It fell to the floor, then hopped about between X $6A and $8C.  Its X
  velocity was $04, $F8, $FC or $08, picked anew at each landing.
- It spat fireballs on frames 63, 127, 191 and 255 (event at 5:$9E7E).
- The first rose from Y $9F to $80 and came down 64 frames later at X
  $B8, still at Y $9F.  It met the floor at Y $C0, X $C6, right of Link,
  on frame 141.  It hopped, then rolled left at 1 pixel per frame,
  slowing by 1/16 every 8 frames.
- Another landed at X $BB on frame 209 and burned in place, its timer
  counting down from 48.
- Screenshot: a red bird with a long neck.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --after 200 \
    --clear $GF --approach B0 --spawn 3:1D:70:80 --projectiles --event Fire=5:9E7E --frames 300
```
