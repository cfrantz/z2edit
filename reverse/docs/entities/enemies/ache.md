# Ache and Acheman

A bat that hangs from the ceiling and swoops down past Link.  The
**Acheman** swoops the same way, and at the bottom of each swoop turns
into a man for a moment and throws a fireball.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $07 | Ache | 3 | 3 | 0 | 1 |
| [West Hyrule](../west-hyrule.md) | $0A | Acheman | 8 | 10 | 2 | 1 |
| [East Hyrule](../east-hyrule.md) | $07 | Ache | 8 | 10 | 2 | 1 |
| [East Hyrule](../east-hyrule.md) | $0A | Acheman | 8 | 10 | 2 | 1 |
| [Great Palace](../great-palace.md) | $07 | Ache | 4 | 3 | 0 | 1 |
| [Great Palace](../great-palace.md) | $0A | Acheman | 2 | 0 | 2 | 1 |

Outdoors both come back after dying (flags bit 6).  They live in caves:
West Hyrule's area $0B (location $0D) has two Aches and an Acheman, and
Death Mountain and East Hyrule have more.  No Great Palace area places
them, and the towns' type $07 (z2edit: "Purple Acheman") isn't placed
either.  Neither has a spawn routine
([spawn defaults](../../systems/enemies.md#spawning)).  Its probe set
doesn't matter, since it never tests the background.

## Behavior

`Ache_Ai`, after `Enemy_SkipAiIfStunned`:

1. **Perched** (`objState` 0).  It hangs at its spawn height, Y $30 in
   practice.  When Link's X + 8 comes within 32 pixels of its X, either
   side, it starts a swoop.  The direction is random (`rng+1,x` bit 0),
   not toward Link.  On the area's first page (`objXHi` 0) it doesn't
   wait for Link: it swoops right at once, and on page 3 left at once,
   toward the middle of the area.
2. **Swooping** (`objState` 1).  Each frame it moves 1/2 pixel across
   (`AcheSpeeds`) and by its Y velocity, which starts at 4 pixels per
   frame down and decreases by 1/16 every frame.  So it dives fast, slows
   to a stop at the bottom, 126 pixels down (Y $AE from Y $30) after 64
   frames, and climbs back the same way.  After 128 frames, back above Y
   $30, it perches again at Y $30, 64 pixels from where it started.  The
   swoop ignores walls and floors; where Link stands high, it dives into
   the ground.
3. On page 0 or 3 a perched bat swoops again at once, so it keeps going
   across the area.  On pages 1 and 2 it waits for Link again.  Its
   perch is wherever the swoop ended.

**The Acheman** (type $0A).  When its Y velocity reaches 0 at the bottom
of a swoop, `enemyAiState` is set to 48.  For those 48 frames it is a man
(`Ache_Draw`) that doesn't move, turns toward Link every frame, and on
the 25th frame throws a fireball (`Enemy_FireProjectile`, projectile
type 4): from its position, 2 pixels lower, at 2 pixels per frame the
way it faces.  Then it climbs back to its perch.  If all six projectile
slots are busy, it doesn't throw.  The fireball (`Projectile_Fireball_Bank1`,
`Projectile_Fireball_Bank2`) flies straight with a sound.

## Drawing

`Ache_Draw`:

- **Perched:** a pair of tile $50 (`SpritePairTiles` entry $16), the
  right one mirrored, both upside down.
- **Flying:** the same the right way up, alternating every 8 frames with
  tile $4E (entry $18), flapping.  Its facing doesn't matter: the pair is
  always drawn as if facing left, which makes it symmetric.
- **The Acheman as a man:** two pairs, tiles $7C, $7E over $80, $82
  (entries $1A and $1C), facing its `objFacing`.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Ache_Ai` | 7:$DB53 | perching, swooping, the man's throw |
| `AcheSpeeds` | 7:$DB4F | swoop X velocity: $F8, and $08 from the next table |
| `Enemy_FireProjectile` | 7:$DBCE | the throw (also the Octorok's rock) |
| `Ache_Draw` | 7:$F17E | drawing |

## Verification

Scenario (West Hyrule cave, area $2D; see the [Myu](myu.md#verification)
page for `$AT`), Link at X $2A0: Aches placed at Y $30 at X $2B0 (near
Link), $260 (64 pixels away), $340 (page 3) and an Acheman at $290.

- The near Ache and the Acheman swooped at once, one left and one right.
  The far one never moved.  The page 3 Ache swooped left at once.
- Each swoop reached Y $AE after 63 frames and was back at Y $30 after
  128, 64 pixels on.
- The page 3 Ache swooped again at once (from X $300, still on page 3),
  then perched at $2C0 on page 2 and stayed.
- The Acheman threw on frame 88, the 25th of its 48 man frames, and
  perched at frame 175 = 128 + 48 - 1.

In area $0B itself (location $0D, with the candle), screenshots showed
the bats perched and flapping, drawn the same facing either way, and the
Acheman as a man facing Link.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --spawn 0:07:2B0:30 --spawn 1:07:260:30 --spawn 2:07:340:30 \
    --spawn 3:0A:290:30 --event Swoop=7:DBA1 --event Perch=7:DBBD --event Fire=7:DB6B --frames 400
```
