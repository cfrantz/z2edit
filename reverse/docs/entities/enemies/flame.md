# Flame

A fire placed in a palace room.  It drops to the floor and burns there
for good; touching it hurts.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 3, 4, 6](../palaces.md) | $10 | Flame | 8 | 5 | 2 | - |

It is placed in rooms $14, $16, $19, $22 and $23 of palaces 3, 4 and 6.
It blocks Link's projectiles, is immune to SPELL, and the sword passes
through it.  It comes back.  Its spawn routine (`Flame_Init`) moves it 4
pixels right.

Not to be confused with the flames that Orange Moas drop and Wizzrobes
cast, which are projectiles (type 7).

## Behavior

`Flame_Ai` treats it as an [item](item.md) (`Item_Fall`): drawn, tested
for hits and contact, falling under gravity of 1/16, bouncing once or
twice on landing and settling on the floor.  Then
`Enemy_HurtLinkOnContact`.  It never moves again.

## Drawing

`Flame_Draw`: one sprite, tile $52 in palette 2, at its X and Y,
mirrored every other 4 frames (`frameCounter` bit 2), so it flickers.  It
isn't drawn when its column is off screen.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Flame_Init` | 4:$B47D | 4 pixels right |
| `Flame_Ai` | 4:$B485 | falling, contact |
| `Flame_Draw` | 4:$B48B | drawing |

## Verification

Scenario at the entrance of palace 3 (see [Rathead](rathead.md#verification)
for `$P3`).  The steps and upper rows were cleared (`$F3`, not drawn), so
the floor is at Link's Y $C0:

```
F3="--fill 0:1FF:20:70 --fill 0:1FF:30:70 --fill 0:1FF:40:70 --fill 0:1FF:50:70 --fill 0:1FF:60:70 --fill 0:1FF:80:70 --fill 0:1FF:90:70 --fill 0:1FF:A0:70 --fill 0:1FF:B0:70"
```

A flame placed at X $A0, Y $60, with Link at $82:

- It fell, bounced twice (to Y $C8, then $CD) and settled at Y $D0 on
  frame 114.
- Link, walking right from frame 40, touched it on frame 54 and was
  knocked back.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $P3,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=4 --after 200 --clear $F3 --approach 80 --spawn 0:10:A0:60 \
    --press 40:RIGHT:40 --frames 120
```
