# Bit

A small blob that moves like a [Myu](myu.md), at double speed for part
of every 256 frames.

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $05 | 3 | 2 | 0 | 1 |
| [East Hyrule](../east-hyrule.md) | $05 | 4 | 2 | 0 | 1 |
| [Towns](../towns.md) | $05 | 0 | 0 | 0 | - |

Outdoors it comes back after dying (flags bit 6); all groups use
background probe set 6.  It has no spawn routine
([spawn defaults](../../systems/enemies.md#spawning)).  The talking Bit
of the towns (type $0B, `TalkingBit_Ai`) is a different enemy drawn the
same way.

## Behavior

`Bit_Ai`, after `Enemy_SkipAiIfStunned`, computes `frameCounter` OR
(enemy index * 32).  Below $C0 it runs `Myu_Ai`, moving exactly as a
[Myu](myu.md#behavior) with 2-pixel steps.  At $C0 or more it sets the
step speeds (`myuStepSpeeds`) to 4 pixels right and left and jumps to
`Myu_Move`: the same movement with 4-pixel steps, 1/2 pixel per frame on
average while moving.

The result is $C0 or more only when bits 7 and 6 are both set.  Since
the enemy index (0-5) times 32 is ORed into the frame counter, not added,
its bit 6 or 7 can stand in for the frame counter's:

| Enemy index | Index * 32 | Fast for frames (`frameCounter` mod 256) | Share |
|---|---|---|---|
| 0, 1 | $00, $20 | $C0-$FF | 1/4 |
| 2, 3 | $40, $60 | $80-$FF | 1/2 |
| 4, 5 | $80, $A0 | $40-$7F and $C0-$FF | 1/2 |

An add would have staggered the fast quarters evenly by slot
([quirks](../../quirks.md)).  In the fast periods its X velocity after a
step is 4 pixels per frame, so a Bit walking off a ledge then flies off
it at that speed.

## Drawing

`Bot_Draw`, as for the [Bot](bot.md#drawing): tiles $B2 and $B4, the
same quiver while resting.  Its palette (`sramEnemyAttr` bits 7-6) is 2
where the Bot's is 3.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Bit_Ai` | 7:$DA2B | picks the step size, then `Myu_Ai` or `Myu_Move` |
| `Bot_Draw` | 7:$EF91 | drawing |

## Verification

Scenario (West Hyrule cave, area $2D, with the commands of the
[Myu](myu.md#verification) page): Bits placed in enemy indices 0, 2 and
4 on the floor at Y $90 for 1500 frames.  Every 2-pixel and 4-pixel step
was tallied by the frame counter's quarter:

| Enemy index | $00-$3F | $40-$7F | $80-$BF | $C0-$FF |
|---|---|---|---|---|
| 0 | 2 px | 2 px | 2 px | 4 px |
| 2 | 2 px | 2 px | 4 px | 4 px |
| 4 | 2 px | 4 px | 2 px | 4 px |

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --spawn 0:05:240:90 --spawn 2:05:250:90 --spawn 4:05:260:90 --frames 1500
```
