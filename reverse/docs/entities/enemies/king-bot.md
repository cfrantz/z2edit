# King Bot

A giant Bot hidden at the top of a Great Palace room.  It drops on Link
when he comes near, hops after him, and bursts into five Bots when it is
worn down.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Great Palace](../great-palace.md) | $1E | King Bot | 56 | 0 | 3 | - |

It is placed alone, at the top of the screen, in rooms $25 and $30.
Link's projectiles bounce off it, and it is immune to SPELL and Thunder,
so only the sword hurts it.  It has no spawn routine.

## Behavior

`KingBot_Ai`:

1. **Asleep** (`objState` 0).  It does nothing, not even draw itself or
   take part in hit tests, until it is on screen and Link is near: his X
   + 8 within 48 pixels of the King Bot's X, either side
   (`Enemy_FaceLink`'s difference).  Then it wakes (`objState` 1).
2. **Falling.**  Awake, it falls (`Enemy_MoveYAndFall`), drifting across
   at the spawn default speed, and lands on the floor.
3. **Hopping.**  On the floor it stands still (`Enemy_Land` each frame).
   Each frame, 1 time in 8 (`rng+1,x` bits 2-0 zero), it turns toward Link
   and hops: 1 pixel per frame across (`KingBotSpeeds`) and 1.5 up, about
   10 pixels high and 24 across.  Contact hurts Link
   (`Enemy_HurtLinkOnContact`).
4. **Bursting.**  When its hit points fall below 48 (9 points of damage),
   it writes a Bot (type $04) into each of enemy slots 5 down to 1, over
   whatever was there:
   - placed around it, by slot (`KingBotPartXOffsets`,
     `KingBotPartXHiOffsets`, `KingBotPartYOffsets`): slots 1 and 2 24
     pixels left, 3 and 4 8 left, 5 8 right (each plus the carry), 0-8
     pixels below its Y.  Slot 5 is written first, so a King Bot in slot
     5 moves itself before placing the rest: they end up about 9 pixels
     further right and 4 lower;
   - each with 208 hit points, stunned for a frame, leaping 3 pixels per
     frame up, and drifting up to a pixel per frame either way at random
     (`rng+2`).

   The King Bot normally spawns in slot 5 (enemies take the highest free
   slot, and it is alone in its room), so it becomes one of the five
   Bots.  It gives no experience; each Bot gives 10.

**Slot 0.**  The burst never writes slot 0 (the loop stops at 1), so a
King Bot in slot 0 stays a King Bot with fewer than 48 hit points.  It
bursts again every frame, putting the five Bots back in place, so they
hang still.  It no longer takes part in hit tests, so it can't be killed
(see [Quirks](../../quirks.md)).  This can't happen in the game's own
rooms.

## Drawing

`KingBot_Draw`: its body from four pairs of `SpritePairTiles` entry $3C,
the right half mirrored.  Four more sprites (tiles $F0 and $F6) sit at
its left and right edges, moving in and out a pixel every 16 frames,
and are hidden when their side is off screen.  It is grey and about 40
pixels across.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `KingBot_Ai` | 5:$A180 | everything above |
| `KingBot_Draw` | 5:$96A5 | drawing |

## Verification

In the Great Palace's entrance (see [Bubble](bubble.md#great-palace) for
`$GP` and `$GF`):

- **Waking.**  A King Bot placed at ($60, $30), Link at $B1, stayed
  asleep and invisible.  Walking left, Link reached $86 on frame 55
  (Link's X + 8 is $2E from it), and it woke and fell.

  ```
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --after 200 \
      --clear $GF --approach B0 --spawn 5:1E:60:30 --event Wake=5:A1DD --press 10:LEFT:40 --frames 80
  ```

- **Hopping.**  Placed at ($70, $70), Link moved to $90, it woke at once
  and landed on frame 33.  It hopped on frames 42 and 78 (events at
  5:$A1FE), each time 24 pixels toward Link and 10 up, and hurt him
  while it touched him.

  ```
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --after 200 \
      --clear $GF --approach B0 --link 90:B0 --spawn 5:1E:70:70 --event Hop=5:A1FE --frames 120
  ```

- **Bursting.**  In slot 5 at ($70, $70), its hit points set to 47, it
  became five Bots in slots 1-5 at once, at X $62, $62, $72, $72, $79
  (from slot 5's new X, $79).
  They leapt and fell apart over the next frames.  In slot 0 the five Bots
  were rewritten every frame and never moved, while the King Bot stayed.

  ```
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --after 200 \
      --clear $GF --approach B0 --spawn 5:1E:70:70 --poke enemyHp+5=0x2F --log 0,1,2,3,4,5 --frames 30
  ```
