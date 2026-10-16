# Walking Moblin

A Moblin that walks straight across the screen, spear held out, and
never attacks.  An invisible generator sends one in from the edge of the
screen every 128 frames.  z2edit calls it the "Dumb Moblin".

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $21 | Walking Moblin Generator | 0 | 0 | 0 | - |
| [West Hyrule](../west-hyrule.md) | $22 | Walking Moblin | 2 | 0 | 0 | 1 |

Only bank 1's group has them: the generator is placed in West Hyrule's
forest encounter area $27, with red [Deelers](deeler.md).  Touching a
walking Moblin drains experience (attribute bit 4).  It uses background
probe set 0.  The generator has the generators' spawn routine
(`Generator_Init_Bank1`).

## Behavior

### The Generator

`DumbMoblinGenerator_Ai` does nothing on frames when the screen is held
at an end of the area (`scrollHeld` set by `Scroll_Update`).  Otherwise
it runs the [Moby](moby.md#the-generator) generator's spawn: every 128
frames, if an enemy slot is free, a new enemy facing Link at the left or
right edge of the screen (`scrollX` + $01 or + $F9).  It then turns it
into a walking Moblin:

- type $22, 2 hit points;
- at Y $A0;
- walking 1 pixel per frame the way it faces (`DumbMoblinSpeeds`);
- keeping the Moby's Y velocity of 2 pixels per frame down until it
  lands.

### The Moblin

`DumbMoblin_Ai`:

1. It sets its timer (`timerFast+TIMER_ENEMY`) to 8 every frame, before
   `Enemy_SkipAiIfStunned`.  What reads it is unknown.
2. `Enemy_UpdateWithBg` (probes, drawing, hit tests).
3. Its spear is tested against Link's shield and body like a projectile
   (`Projectile_CheckLink`), then `Enemy_HurtLinkOnContact`.
4. It walks on (`Enemy_MoveXUnlessWall_Bank1`), stopping while there is a
   wall ahead.  It falls (`Enemy_MoveYAndFall`) and stands on floors
   (`Enemy_Land`).

It never turns: it walks past Link and on until it leaves the screen
and is despawned.  At a wall it stays.

## Drawing

`DumbMoblin_Draw` sets `objState+1` to Moblin pose 3 or 4, alternating
every 16 frames, and draws it with `Moblin_Draw`.  Then it redraws the
lower pair 16 pixels below its Y, alternating every 8 frames between
`sramEnemyTiles` entries $38 and $4A: its legs, walking.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `DumbMoblinGenerator_Ai` | 1:$992F | the generator |
| `MobyGenerator_Ai` | 1:$9B31 | its spawn |
| `DumbMoblinSpeeds` | 1:$992C | by facing, from the `rts` before it: $10 right, $F0 left |
| `DumbMoblin_Ai` | 1:$994F | the Moblin |
| `DumbMoblin_Draw` | 1:$9C43 | drawing |

## Verification

Neither test movie meets one.  Scenario (West Hyrule cave, area $2D; see
the [Myu](myu.md#verification) page for `$AT`), a generator placed with
Link at X $2A0:

- New Moblins appeared at the right edge of the screen at Y $A0 at
  `frameCounter` $80 and $00, facing Link.  In the cave Y $A0 is inside
  the floor, and they stood stuck.
- With map rows $80 and $90 cleared (`--fill 200:39F:80:70 --fill
  200:39F:90:70`) one walked left at 1 pixel per frame, from X $331
  through Link and on past him, its pose alternating every 16 frames.
- With Link at X $380, where the screen stays at the right end of the
  area, the generator never spawned in 400 frames, while a Moby
  generator beside it spawned every 128.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --fill 200:39F:80:70 --fill 200:39F:90:70 --spawn 5:21:200:30 \
    --log 0,1,2,3,4 --event Spawn=1:993D --frames 400
tools/enemy_scenario.py ... --approach 380 --spawn 5:21:200:30 --spawn 4:1D:200:30 \
    --event Spawn=1:993D --event MobySpawn=1:9B48 --event GenCheck=1:9934 --frames 400
```
