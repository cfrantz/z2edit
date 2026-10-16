# Deeler

A spider that hangs from the trees in forests, follows Link along under
the branches and drops on its thread when he is near.  The blue Deeler
lets go low down and hops after him on the ground.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $0D | Red Deeler | 3 | 2 | 0 | 1 |
| [West Hyrule](../west-hyrule.md) | $0E | Blue Deeler | 4 | 3 | 0 | 1 |
| [East Hyrule](../east-hyrule.md) | $0D | Red Deeler | 12 | 5 | 2 | 1 |
| [East Hyrule](../east-hyrule.md) | $0E | Blue Deeler | 12 | 10 | 2 | 1 |
| [Great Palace](../great-palace.md) | $0D | Red Deeler | 4 | 3 | 0 | 1 |
| [Great Palace](../great-palace.md) | $0E | Blue Deeler | 6 | 5 | 0 | 1 |

They are placed in the forest encounter areas ($27-$2A) of both Hyrules,
at Y $50, under the branches, so that they drop to Y $B0.  The outdoor spawn routine
(`Deeler_Init_Bank1`, `Deeler_Init_Bank2`) sets the thread: its top
(`deelerThreadTop`) is the spawn Y, its bottom (`deelerThreadBottom`) $60
lower.  These are single variables, shared by all Deelers.  In the Great
Palace the types have no spawn routine and no area places them.

## Behavior

`Deeler_Ai`, after `Enemy_SkipAiIfStunned` and `Enemy_HurtLinkOnContact`:

1. **Hanging** (`enemyAiState` 0).  Every frame it turns toward Link
   (`Enemy_FaceLink`) and moves 1/2 pixel toward him (`DeelerXSpeeds`).
   It doesn't stop above him, so it keeps passing him and turning back.
   While Link's X + 8 is within 64 pixels of its X, either side, and it
   is on screen, it drops instead on 1 frame in 32 at random (`rng+1,x`
   bits 4-0 all 0).
2. **Dropping** (`enemyAiState` 1).  It goes straight down at 2 pixels
   per frame (`DeelerYSpeeds`) until its Y equals `deelerThreadBottom`.
3. **Climbing** (`enemyAiState` 2).  It goes back up at 1 pixel per frame
   until its Y equals `deelerThreadTop`, and hangs again.

A drop of 96 pixels takes 48 frames and the climb 96.  On the thread it
doesn't probe the background, so it hangs and drops through branches
and floors.

**The blue Deeler** lets go when it reaches Y $8E on its thread, going
down or up: `ror enemyAiState` sets bit 7.  (A thread whose bottom is
above $8E never reaches it.)  It then:

- falls, starting at 2 pixels per frame, under gravity of 1/16 pixel per
  frame per frame, and stops on the floor (`Enemy_UpdateWithBg`,
  `Enemy_Land`);
- 32 frames after letting go (`timerFast+TIMER_ENEMY`), hops toward Link,
  at 2 pixels per frame across (4 times `DeelerXSpeeds`) and with a Y
  velocity of -$11 to -$18 at random, and again every 64-127 frames
  (`rng+1,x` bits 5-0 ORed with $40);
- stops moving sideways when it lands.

A hop rises about 16 pixels and lasts about 45 frames, so it carries the
Deeler some 90 pixels, usually past Link.

**One thread for all.**  Since the thread's ends are shared, a Deeler
spawned at another height than the last one drops to the other's bottom
and climbs back to the other's top, where it then hangs.  The ends are
compared for equality, but every spawn height is a multiple of 16, so a
Deeler always reaches them ([quirks](../../quirks.md)).

## Drawing

`Deeler_Draw`: a pair of sprites, tiles $BA, $BC (`SpritePairTiles`
entry $10), mirrored on alternate 8-frame periods so that it wriggles.

- It is upside down while hanging.
- Once a blue Deeler has let go, it is drawn 2 pixels lower.
- On the thread, when on screen and below the top, three more sprites
  draw the thread: tile $BE, palette 1, at its X.  They are spaced a
  third of the thread's length apart from `deelerThreadTop`, and shifted
  down half a space on even frames, so that over two frames they cover
  the thread.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Deeler_Init_Bank1`, `Deeler_Init_Bank2` | 1:$9625, 2:$961B | spawn routine: the thread's ends |
| `Deeler_Ai` | 7:$D6DF | everything above |
| `DeelerYSpeeds` | 7:$D6DB | $20 down, $F0 up |
| `DeelerXSpeeds` | 7:$D6DD | $08 right, $F8 left |
| `Deeler_Draw` | 7:$F027 | drawing, with the thread |

## Verification

Neither test movie meets a Deeler.  Scenario (West Hyrule cave, area
$2D; see the [Myu](myu.md#verification) page for `$AT`), Link at X $2A0,
with the thread set as the spawn routine would for Y $30 (`--poke`):

- A red Deeler 24 pixels from Link dropped at frame 16 and reached Y $90
  after 48 frames.  It was back at $30 after another 96, and dropped
  again 80 frames later.
- A red Deeler 136 pixels away moved toward Link at 1/2 pixel per frame
  and didn't drop until within 64 pixels.
- A blue Deeler let go at Y $8E and landed on the floor at once (Y $90).
  It hopped 32 frames later, and again after 98, 110, 74, 88, 118 and 68
  frames.  Its first hop started with Y velocity -$15 and landed 45
  frames and 92 pixels later, past Link.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --spawn 0:0D:2C0:30 --spawn 1:0E:280:30 --spawn 2:0D:330:30 \
    --poke deelerThreadTop=0x30 --poke deelerThreadBottom=0x90 \
    --event Step=7:D70F --event LetGo=7:D734 --event Hop=7:D769 --event Land=7:D785 --frames 700
```

The shared thread: red Deelers at Y $30 and $50 with the thread set for
$50 (top $50, bottom $B0).  The one from $30 dropped 128 pixels to $B0,
climbed to $50, and from then on hung and dropped between $50 and $B0
with the other.

```
tools/enemy_scenario.py ... --spawn 0:0D:2B0:30 --spawn 1:0D:2A0:50 \
    --poke deelerThreadTop=0x50 --poke deelerThreadBottom=0xB0 --event Step=7:D70F --frames 500
```
