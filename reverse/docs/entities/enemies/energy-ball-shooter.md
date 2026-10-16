# Energy Ball Shooter

A statue in a palace wall that fires energy balls down at Link.

## Where It Appears

| Group | Type | Enemy |
|-------|------|-------|
| [Palaces 1, 2, 5](../palaces.md) | $16, $17 | Energy Ball Shooter (facing right, facing left) |
| [Palaces 3, 4, 6](../palaces.md) | $16, $17 | Energy Ball Shooter (facing right, facing left) |

They are placed in rooms $1A, $1D and $24 of palaces 1, 2 and 5, and $29
of palaces 3, 4 and 6.  The spawn routine (`EnergyBallShooter_Init`) sets
the facing by type: $16 right, $17 left.  The statue is background; the
shooter itself isn't drawn and can't be hit.

## Behavior

`EnergyBallShooter_Ai`: every 32 frames (`frameCounter`), while it is on
screen, `Enemy_FireAimedBall`:

1. It takes a projectile slot (`Enemy_FireProjectile`).
2. If Link isn't on the side it faces, it gives the slot back and fires
   nothing.
3. It takes d = (|dx| - 20) / 16 from Link's distance.  From d 8 on, about
   148 pixels or more, it fires nothing.
4. Otherwise it fires projectile type 1 from 4 right of and 5 below its
   position.  The ball goes across at `AimedBallXSpeeds[d]` ($06-$13) and
   down at `AimedBallYSpeeds[d]` ($16-$0B), in a straight line down at
   Link, flatter the further he is.

The ball (`Projectile_Straight_Bank4`) flies straight and bursts at Y $C5,
near the floor.  Its flags ($97) let it pass through Link's shield.
Wolf heads fire the same projectile type, but straight ahead.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `EnergyBallShooter_Init` | 4:$B8FC | spawn: facing by type |
| `EnergyBallShooter_Ai` | 4:$9BDD | every 32 frames |
| `Enemy_FireAimedBall` | 4:$9BE5 | the aimed ball |
| `Projectile_Straight_Bank4` | 4:$974E | the ball |

## Verification

Scenario in palace 1's first room (see [Bubble](bubble.md#verification)),
a shooter of type $16 placed at (X $168, Y $60), facing right, Link at X
$1A0-$1D0 to its right:

- It fired at `frameCounter` $E0, $00, $20, $40, $60 and $80, every 32
  frames.
- With Link 74 pixels away the ball flew at ($0F, $11), at 90 pixels
  ($11, $0F), at 104 pixels ($12, $0E): flatter as he moved off.
- A shooter at X $140 fired once.  Then the screen scrolled with Link and
  left it just off screen, and it stopped.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT --until currentWorld=3 --after 200 \
    --clear $PFLAT --approach 1A0 --spawn 4:16:168:60 --poke objFacing+5=1 --projectiles \
    --event Fired=4:9C39 --event Cancel=4:9C3F --frames 200
```
