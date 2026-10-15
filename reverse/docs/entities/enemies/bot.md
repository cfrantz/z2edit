# Bot

A blob that moves like a [Myu](myu.md) and now and then leaps at Link.

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $04 | 3 | 2 | 0 | 1 |
| [East Hyrule](../east-hyrule.md) | $04 | 4 | 2 | 1 | 1 |
| [Towns](../towns.md) | $04 | 0 | 0 | 0 | - |
| [Palaces 1, 2, 5](../palaces.md) | $04 | 4 | 2 | 1 | 1 |
| [Palaces 3, 4, 6](../palaces.md) | $04 | 4 | 2 | 1 | 1 |
| [Great Palace](../great-palace.md) | $04 | 48 | 10 | 3 | 1 |

Outdoors and in the palaces it comes back after dying (flags bit 6); all
groups use background probe set 6.  It has no spawn routine
([spawn defaults](../../systems/enemies.md#spawning)).  The SPELL spell
turns other enemies into Bots outside towns.

## Behavior

`Bot_Ai`, after `Enemy_SkipAiIfStunned`:

1. **Leap.**  If last frame's probe found a floor under it (`objCollision`
   bit 2), in either phase, and bits 6-0 of its random byte (`rng+1,x`)
   are all 0, which happens on about 1 frame in 128, it leaps: Y
   velocity -$1B (1.69 pixels per frame up) and X velocity 1 pixel per
   frame toward Link (`BotJumpSpeeds`, by `Enemy_FaceLink`).
2. Then it runs `Myu_Ai`: steps of 2 pixels, phases of moving and
   resting, falling and landing, exactly as the [Myu](myu.md#behavior).

Since the leap's X velocity is only replaced at the Bot's next step, it
crosses the air at 1 pixel per frame.  A leap rises 13 pixels in 14
frames, under `Enemy_MoveYAndFall`'s gravity, and lands 27 frames after
it began, about 27 pixels further on.  A wall ahead stops it moving
sideways, as for a Myu.  The leap takes no account of the phase: a
resting Bot leaps too.  While it is in the air its step counter doesn't
advance and its phase can't change.

## Drawing

`Bot_Draw` (also used by the Bit): a symmetric pair of sprites, the
right one the left one mirrored; tile $B2 (`SpritePairTiles` entry $26)
when `objState` is 0, tile $B4 (entry $28) otherwise, with `objState`
set as for the [Myu](myu.md#drawing).  A resting Bot quivers: on
alternate 4-frame periods (`frameCounter` bit 2 clear) its two sprites
are drawn a pixel closer together.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Bot_Ai` | 7:$DA0C | the leap, then `Myu_Ai` |
| `BotJumpSpeeds` | 7:$D9FE | leap X velocity: $10 right, $F0 left |
| `Bot_Draw` | 7:$EF91 | drawing (Bot and Bit) |

## Verification

No-glitch movie, frames 42804-44334, two Bots on a floor at Y $B0
(`Bot_Ai` and an event breakpoint on the leap, `$DA1B`): 45 leaps over
the first 100000 frames.  One leap from X $F0, Y $B0 rose to Y $A3 and
landed 27 frames later at X $10B, 1 pixel per frame throughout; a
Bot leapt while resting.  Phases and steps as for the
[Myu](myu.md#verification).
