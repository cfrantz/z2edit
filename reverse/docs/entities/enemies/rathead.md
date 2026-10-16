# Rathead

The [Tinsuit](tinsuit.md) of palaces 3, 4 and 6: a small creature that
hops along the floor, sent in by an invisible generator.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 3, 4, 6](../palaces.md) | $0B | Rathead Generator | 2 | 0 | 0 | - |
| [Palaces 3, 4, 6](../palaces.md) | $0C | Rathead | 2 | 0 | 0 | - |

The generator is placed in rooms $02 and $15 of palaces 3, 4 and 6;
Ratheads themselves in rooms $08, $11, $18, $1E, $25 and $3C.  Touching
one drains experience.  They come back.

## Behavior

As the Tinsuit's: `RatheadGenerator_Ai`, `RatheadSpeeds` and
`Rathead_Init` are copies of `TinsuitGenerator_Ai`, `TinsuitSpeeds` and
`Tinsuit_Init`.  `Rathead_Ai` differs from `Tinsuit_Ai` in two ways:

- It has no `Enemy_SkipAiIfStunned`, so it keeps hopping while stunned.
- It lands by rounding its Y down to 8 pixels, not with `Enemy_Land`.

## Drawing

`Rathead_Draw`: two pairs of sprites, 1 pixel low, from `sramEnemyTiles`
entry $B8, or $BC after odd hops.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `RatheadGenerator_Ai` | 4:$B343 | the generator |
| `Rathead_Init` | 4:$B33C | a list's Rathead |
| `RatheadSpeeds` | 4:$B33E (+ facing, + 0 or 2) | $0C, $F4, $14, $EC |
| `Rathead_Ai` | 4:$B362 | hopping |
| `Rathead_Draw` | 4:$B38A | drawing |

## Verification

Scenario at the entrance of palace 3, which works from the file select
movie like palace 1's:

```
P3=areaPrgBank=4,currentRegion=0,currentWorld=4,townCode=4,palaceCode=2,areaLocation=0x36,areaId=0,areaStartPage=0,areaEntrySide=0,prevRegion=0
```

The steps and upper rows were cleared (`$F3`, see
[Flame](flame.md#verification)), and a generator placed with Link at X
$80:

- Ratheads were launched on frames 109 and 237, the first from the left
  edge at $0C (3/4 pixel per frame), the second from the right.
- They hopped every 16 frames along the floor.
- Screenshot: a small red figure, 32 pixels tall.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $P3,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=4 --after 200 --clear $F3 --approach 80 --spawn 0:0B:100:B0 --event Rat=4:B350 --frames 300
```
