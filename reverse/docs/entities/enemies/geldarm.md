# Geldarm

A sand worm of the desert that stands with its neck stretched up.
Striking its neck with the sword makes it duck into the sand; it comes
back up a few seconds later.

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $20 | 8 | 5 | 1 | 1 |

Only bank 1's group has it: the desert encounter areas of West Hyrule
($1E-$20) and Death Mountain ($1D-$1E of the second group), each with
five Geldarms and two [rock generators](rock-generator.md).  It is
immune to the SPELL spell, and its hitbox byte is $85: hitbox 5, tall
(bit 7).

Its spawn routine, `Geldarm_Init`, sets it up stretched up
(`enemyAiState` 1).  Its head is at its spawn Y, and its base, kept in
`objState+1`, is $3F pixels lower ($40 with the carry set).

## Behavior

`Geldarm_Ai` doesn't call `Enemy_SkipAiIfStunned`.  It never probes the
background.

**Stretched up** (`enemyAiState` 1).  It doesn't move.  While it is on
screen it tests a box over its neck: 8 pixels wide, from 8 pixels below
its head (9 with the carry) down to its base.

- If Link's sword overlaps the box (`Sword_GetHitbox`,
  `Hitbox_Overlap`), it ducks: `enemyAiState` 0, sound $10 on channel 1,
  and its direction (`objFacing`) flips to 1, sinking.
- Otherwise, if Link himself overlaps it, he is hurt (`Link_HurtByEnemy`).

**Moving** (`enemyAiState` 0), by `objFacing`:

- **Sinking** (1): 1 pixel per frame down.  At its base (height 0) it
  turns to rising and waits 128 frames (`timerFast+TIMER_ENEMY`).
- **Rising** (0): each time its timer runs out, every 5 frames, it rises
  2 pixels: one from a step that is otherwise put back, one from the
  step proper.  When its height reaches $40 it is stretched up again.

So a sword stroke on the neck makes it duck for 64 frames, stay down for
128 and take 160 to come back up.  Then it stays up until struck again.
Otherwise it is tested like any enemy (`Enemy_UpdateCommon`), at its
hitbox about its head, so that is where it takes damage, and touching it
there hurts Link (`Enemy_HurtLinkOnContact`).  The scenarios below didn't
test damage.

## Drawing

`Geldarm_Draw`, unless its column is off screen:

- the head, tile $62, at its Y;
- unless it is fully down, five neck sprites, tile $64, spaced a fifth of
  its height apart (the remainder shared out a pixel at a time) down to
  its base, all in one 8-pixel column;
- all mirrored on alternate 16-frame periods;
- while stretched up, the head swaying 2 pixels and the first neck sprite
  1 pixel to one side, then the other, every 32 frames;
- while stunned, the palette flashing (`enemyStunTimer` bits 1-0).

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Geldarm_Init` | 1:$986C | spawn routine |
| `Geldarm_Ai` | 1:$9BB5 | everything above |
| `GeldarmYOffsets` | 1:$9BB3 | step by direction: -1 rising, +1 sinking |
| `Geldarm_Draw` | 1:$9E0C | drawing |
| `GeldarmSegmentXOffsets` | 1:$9E0A | the sway |

## Verification

Neither test movie meets one.  Scenario (West Hyrule cave, area $2D; see
the [Myu](myu.md#verification) page for `$AT`), a Geldarm placed with
its head at Y $50 and base at $90 (pokes as `Geldarm_Init` would), Link
at X $2A0:

- Placed at X $2B8, it hurt Link on the first frame and knocked him back
  out of reach.
- Placed at $2BC-$2C4, with B pressed on frame 2, the sword struck its
  neck on frame 10.  It reached its base (Y $90) on frame 74 and rose
  from frame 203, 2 pixels every 5 frames.  It was stretched up again
  (Y $50) on frame 357, and then stayed up.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --spawn 3:20:2C0:50 --poke enemyAiState+3=1 --poke objFacing+4=0 \
    --poke objState+4=0x90 --event Hit=1:9C0F --event Bottom=1:9BDE --event Top=1:9BEA \
    --press 2:B:4 --frames 500
```
