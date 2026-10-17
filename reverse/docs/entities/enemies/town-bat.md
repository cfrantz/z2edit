# Town Bat

A bat of Saria and Darunia: a townsperson who flies off as one after
a conversation.

## Where It Appears

| Group | Type | Enemy | AI | Drawing |
|-------|------|-------|----|---------|
| [Towns](../towns.md) | $03 | town bat | `TownBat_Ai` | `Ache_Draw` |

No town room places one.  In Saria and Darunia, when a conversation with
a walker ($19-$1C) ends, it becomes one 103 times in 256
(`Dialog_MaybeTurnToBat`, `rng+2,x` below $67).  It turns into type $03
with `objState+1` 3 and `enemyAiState` $80, standing still.  Unlike the
people it can be hurt: it has 0 hit points, so one hit kills it, and it
hurts Link with damage class 2.

## Behavior

`TownBat_Ai`: while on screen it is an [Ache](ache.md) (`Ache_Ai`).
Once it is off screen, it is removed.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `TownBat_Ai` | 3:$96A8 | everything above |
| `Dialog_MaybeTurnToBat` | 3:$B7CC | a walker turning into one |

## Verification

Town scenario (see [Townsperson](townsperson.md#verification) for `$TA`;
here `townCode=2`, Saria), area 0 with its people cleared, bats placed as
`Dialog_MaybeTurnToBat` leaves them:

- One on screen at ($80, $60) rose and dived like an Ache: Y $42 on
  frame 30, $A9 on frame 90.
- One off screen at X $180 was removed on the first frame.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at ${TA/townCode=0/townCode=2},areaId=0 \
    --until currentWorld=1 --after 200 --clear --spawn 2:03:80:60 --spawn 3:03:180:60 \
    --poke enemyAiState+2=0x80 --poke objState+3=3 --log 2,3 --frames 120
```

The turning: in Saria (`townCode=2`) area 7, Link walked to X $80 and
pressed B every 10 frames for 1500 frames, talking to whoever came by.
A type $1A walker (frame 393) and a type $1B (frame 713) turned into
bats as their conversations closed; the life lady, talked to over and
over, never did.

```
BS=""; for k in $(seq 4 10 1500); do BS="$BS --press $k:B:2"; done
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at ${TA/townCode=0/townCode=2},areaId=7 \
    --until currentWorld=1 --after 200 --approach 80 $BS --event MaybeBat=3:B7CC --event Bat=3:B7E0 \
    --log 9 --frames 1500 --dialog
```
