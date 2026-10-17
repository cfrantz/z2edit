# Dark Link

The last enemy: Link's shadow, which steps out of him in the Great
Palace's last room and fights with his sword and shield.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Great Palace](../great-palace.md) | $23 | Dark Link | 8 (248 in the table) | 150 | 4 | - |

Its trigger is placed alone on page 1 of room $36, at X $1D0, Y $50.
Link's projectiles bounce off it, and SPELL and Thunder don't affect it.

**Spawning** (`DarkLink_Init`):

- The trigger spawns in slot 5, then moves itself to enemy slot 0,
  overwriting whatever is there.  It becomes type $23 with 8 hit points,
  at the trigger's place, and frees slot 5.
- It is drawn and moved as object 1, the slot after Link's.
- Link's drawing and sword code take it as a second Link (`swordOwner`
  1): its pose is `objState+1`, its stance `darkLinkStanding`, and its
  facing `darkLinkWalkDir`.

## Behavior

`DarkLink_Ai` first queues the glowing color of the Triforce, sprite
palette 3's color 2 (`Triforce_FlashPalette`).  Then it runs its phase, kept in `objFacing+4`:

**0. The old man** (`DarkLink_WaitForLink`):

- Until Link arrives, type $23's drawing routine (`OldMan_Draw`) shows an
  old man sitting on the pillar at the trigger's place, with the Triforce
  glowing beside him.
- When Link's screen reaches page 1 of the room, the scroll freezes and
  Link stops.
- Once he is on the floor, he is set at Y $A0, facing right, and the game
  freezes.

**1. The old man vanishes** (`DarkLink_Appear`), over 255 frames
(`timerFast+3`):

- 63 frames as before, 112 flickering, 80 gone.
- Then the screen flashes (`spellFlash` $C0) with sound 4, the backdrop
  turns $14, and Dark Link is put on Link.

**2. The shadow steps out** (`DarkLink_Separate`):

- When the flash ends, the room's colors go dark (PPU string $0D), with
  the music stopped and sound 4.
- For 127 frames it stands on Link, in pose $0A.  Then it jumps out to
  the right, 4 pixels per frame up and about 1.2 across.
- When it lands at Y $A0, the fight phase starts.

**3. The fight** (`DarkLink_Fight`):

- For 112 frames it crouches.  Then the fight music ($40) starts and the
  game unfreezes.
- Its hit points show in the boss bar, a unit each.

Each frame it reacts to Link by its distance from him across, (|dx| +
3) / 16 (`DarkLink_ReactToLink`):

- **Close** (0-1, within about 28 pixels), on the floor:
  - When Link presses B it takes his stance (`darkLinkStanding` =
    `linkStanding`), so its shield is at the height of his stroke.  1
    time in 8 it then jumps away instead.
  - Otherwise, 1 frame in 4, it backs off for 16 frames at 1.25 pixels
    per frame (`DarkLink_MaybeTurn`, `DarkLinkSpeeds`).
  - Otherwise, unless it is already in the first two steps of an attack
    or is above Link, it attacks at once.
- **Middle** (2-5, up to about 92 pixels): in 3 of every 4 64-frame
  periods it walks toward him at 1.25 pixels per frame
  (`DarkLink_TurnSometimes`).
- **Far** (6 or more): it stands and waits.

**An attack** (`DarkLink_UpdateAttack`, steps in `enemyAiDir`):

- It aims where Link's shield isn't: its stance is the opposite of his,
  low if he stands and high if he crouches.
- It stops, with a sound ($20 on channel 1).
- Step 1 lasts 16 frames, then step 2 (the thrust) 8, then step 3 for 8.
  Standing its poses are 4, 5, 4; crouching 6, 7, 6
  (`DarkLinkAttackPoses`).
- A new attack can begin in step 3, so in a close fight it attacks again
  and again.

**Jumps** (`DarkLink_GetSide`): 4 pixels per frame up.  Across it goes
1.5 pixels per frame toward the middle of the screen, or 1 time in 4
straight up.  At the edge of the screen it jumps back toward the middle,
and it never walks off (`DarkLink_Move`).  In the air it takes Link's
stance.  Gravity is $3C/256 pixel per frame per frame, up to 4 pixels per
frame.

**Moving** (`DarkLink_UpdateXVelocity`, `DarkLink_UpdateJump`,
`DarkLink_Animate`):

- Its sprites always face Link.
- It walks forward or backward (`DarkLinkWalkPoses`, a frame every 4
  frames).
- Standing still it is in pose 3, or 6 crouching.

**Swords and shields** (`DarkLink_CheckSword`): no tests while either of
them is in the air, or while it is invulnerable.  Otherwise, in order:

1. Link's sword against its shield, a 6-by-9 box in front of it at its
   stance's height: a clink.  Both are knocked back (`Enemy_KnockBack`),
   it at Link's X velocity reversed.
2. Link's sword against its body (29 pixels high): a hit.
   - It loses 1 hit point, whatever Link's attack level, so it always
     takes 8 hits.
   - Its attack is cut off, with a sound (`dmcRequest` 1).
   - It is knocked back for 24 frames (`timerFast+TIMER_BOSS`), hopping
     3 pixels per frame up and 0.625 across away from Link.
   - It is invulnerable for 56 frames (`timerFast+8`).
3. Its sword (`swordX+1`, `swordY+1`) against Link's shield: a clink as
   in 1.  Otherwise against Link: he is hurt (`Link_HurtByEnemy`).

Body contact also hurts Link (`Enemy_HurtLinkOnContact`, damage class 4).

**Defeat.**  At its last hit the knockback timer is $FF instead:

- It hops and then sinks through the floor, with a sound every 16
  frames.
- At Y $F0 the screen flashes, the backdrop goes black, the music stops,
  and phase 4 starts.

**4. Defeated** (`DarkLink_Defeated`): when the flash ends the room's
colors come back (PPU string $0E).  When Link is on the floor and 64
frames have passed, the game freezes.

**5. To the middle** (`DarkLink_WalkLinkToCenter`): after 128 frames
Link walks a pixel per frame to X $160, facing right.  Object 1 moves to
($17C, $B0), where the old man will stand.

**6. The Triforce** (`DarkLink_ShowTriforce`), by `timerSlow+4` from
$0D (a step every 21 frames):

- From 8, the old man appears standing beside Link, with the Triforce
  above him (`OldMan_Draw`), flickering until 5.
- At 2, the screen flashes, music $20 plays (`DarkLink_StartTriforce`),
  and phase 7 starts.

**7. The ending** (`DarkLink_StartEnding`):

- Link faces left and holds the Triforce over his head (state $0B), the
  old man beside him, while `timerSlow+4` runs down from $15, about 440
  frames.
- Then the ending starts (`gameState` 3), with rendering off.

## Drawing

- `DarkLink_Draw`: Link's own drawing code (`Link_DrawBody` with
  `swordOwner` 1) draws its body and sword at `oamBuffer` offset $50.
  In the darkened room it is a black silhouette.
- `OldMan_Draw`: the old man and the Triforce, in phases 0-1 and 6-7.
  - The old man is `SpritePairTiles` entry $36, or $38 for 8 frames of
    every 32.
  - The Triforce is entry $3A in palette 3, whose color 2
    `Triforce_FlashPalette` cycles every 4 frames.
  - The Triforce is placed by `TriforceXOffsets` and `TriforceYOffsets`:
    beside him on the pillar, above him, or in Link's hands.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `DarkLink_Init` | 5:$9796 | spawn: move to slot 0 |
| `DarkLink_Ai` | 5:$97C6 | the phases |
| `DarkLink_WaitForLink`, `DarkLink_Appear`, `DarkLink_Separate` | 5:$97DE, 5:$980D, 5:$983E | phases 0-2 |
| `DarkLink_Fight` | 5:$987B | phase 3 |
| `DarkLink_ReactToLink`, `DarkLink_MaybeTurn`, `DarkLink_TurnSometimes`, `DarkLink_GetSide` | 5:$98EB, 5:$994F, 5:$9989, 5:$992C | decisions |
| `DarkLink_UpdateXVelocity`, `DarkLink_UpdateJump`, `DarkLink_Animate`, `DarkLink_Move` | 5:$99A4, 5:$99B8, 5:$99E2, 5:$9A0F | moving |
| `DarkLink_UpdateAttack`, `DarkLink_CheckSword` | 5:$9A50, 5:$9A8D | fighting |
| `DarkLink_Defeated`, `DarkLink_WalkLinkToCenter`, `DarkLink_ShowTriforce`, `DarkLink_StartEnding` | 5:$9B3A, 5:$9B53, 5:$9B91, 5:$9BA8 | phases 4-7 |
| `DarkLink_Draw`, `OldMan_Draw`, `Triforce_FlashPalette` | 5:$9A77, 5:$A472, 5:$A4C4 | drawing |

## Verification

The no-glitch movie (`../private/Zelda2_NoGlitch_2014_Final.fm2:0`)
fights Dark Link.  The events were hooked as on the
[Thunderbird](thunderbird.md) page:

- Phase 0 ended on frame 163856, with Link at ($171, $A0).  Phase 1
  ended 255 frames later.  Screenshots: the old man on the pillar, then
  the dark room with the black shadow.
- The fight: 8 hits (event at 5:$9AD3), on frames 164417, 164473, 164529,
  164585, 164641, 164697, 164753 and 164811.  Each came 56 frames after
  the last, as soon as the invulnerability ran out.  Most landed with
  Link standing and the shadow crouched (pose 6), its shield low.
- It attacked 12 times (5:$9978), backed off 37 times and jumped once.
  Link was never hurt.
- After the last hit it sank to Y $F1 by frame 164862, starting phase 4.
  Phase 5 started 64 frames later, on 164926.

The movie ends there.  Running on past its end without input:

- Phase 6 started on frame 165101, with Link at X $160 and object 1 at
  ($17C, $B0).
- Phase 7 started on 165325.  Link then held up the Triforce (state
  $0B).
- `gameState` became 3 on frame 165766.
- Screenshots: the old man beside Link with the Triforce above him, then
  Link holding it up.
