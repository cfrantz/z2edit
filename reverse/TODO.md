# TODO

Remaining work, roughly in the suggested order.  The enemies of every
area bank are documented; what is left is mostly routines still marked
`Status: provisional`.

## Done

- 1. Area drawing (banks 1-5): every object routine confirmed against the
  tile maps the game builds (`tools/area_objects.py`, `tools/area_dump.py`);
  see docs/world/areas.md, towns.md and palaces.md.
- 2. Projectiles and boss leftovers (banks 1, 2, 4): confirmed with
  per-frame sprite readings and boss scenarios in their own palaces.
- 3. Town conversations (bank 3): every handler confirmed by scenarios
  (`enemy_scenario.py --dialog`) and by both movies replayed with the
  handlers hooked; see docs/world/towns.md.
- 4. Title, story, ending and saves (bank 5): checked against the idle
  title movie, the file select movie, and the no-glitch movie played past
  its end with START pressed (scenes 13-14, the second quest mark); see
  docs/systems/title-and-files.md.
- 5. Sound (bank 6): the 8 routines confirmed from APU write logs (the
  title and file select movies, and effects started in scenarios with
  `enemy_scenario.py --apu`); docs/audio/engines.md has a Sound Effects
  section.
- 6. Loose ends: the RAM bytes named; the fairy's refill timed; the red
  jars' area flags (a quirk in nine palace rooms); extraSpriteAttrs;
  Helmethead's helmets; the unhorsed Rebonack's facing.
- 7b. The area banks' shared code: docs/architecture/area-banks.md
  (`tools/bank_compare.py`).

## 7. Larger documentation

- A reimplementation guide built from `docs/quirks.md`.

## 8. Line-by-line commentary

Every routine has a header, but most bodies have no comments.  This
phase brings all the code up to the standard of the enemy pass
(`Moa_Ai`, `Deeler_Ai`): a trailing intent comment on nearly every
instruction (obvious ones like `pha` and `clc` stay bare), a short block
comment per step, every `@loc_` label named, and `BUG:`/`QUIRK:` notes
collected into docs/quirks.md.  Claims are checked in the emulator, with
scripted scenarios for paths the movies don't reach.  Where a routine
exists in several banks, comment one copy and carry the comments to the
identical ones (`tools/bank_compare.py`).

Baseline (2026-10-18): about 23,800 instructions, 11% with a trailing
comment; the enemy routines that are done measure 45-65%.  `@loc_`
labels: bank 0 260, bank 7 246, bank 6 155, bank 5 153, bank 3 59,
bank 4 48, banks 1-2 23.

- 8.0. Done: `make status` has a Commentary table, and
  `tools/status.py --routines BANK` gives it per routine.

Banks 7 and 0, by subsystem (instruction counts; under 3% commented
unless noted):

- 8a. Done: the core frame and the game state machine (bank 7), checked
  over both movies; docs/architecture/main-loop-and-nmi.md has the game
  NMI's frame step by step.  The DMC samples at $F300 are labelled.
- 8b. Done: sprite management (bank 7): the slot rotation, the off-screen
  tests, the `Sprite_*` helpers, the dying puff, and Link's and Dark
  Link's drawing, with Link's poses (`LINK_POSE_*`) checked over the 100%
  movie; docs/systems/link.md (Poses and Drawing) and docs/hardware/ppu.md
  (Sprites).  `Fairy_Draw` (18%) is left for 8p.
- 8c. Done: game modes and transitions (bank 7): location entry, world
  loading, area transitions, doors, death, game over and the continue
  screen (scripted with play.py; docs/systems/death.md).
- 8d. Done: area loading and scrolling (banks 7 and 0); `scrollPage` and
  `drawPage` renamed `scrollDir` and `columnDrawDir`; the scroll limits
  checked over the 100% movie and the row fill with area_dump.py;
  docs/world/areas.md (Scrolling).
- 8e. Done: the side view frame (`Mode_SideView`) and the object framework
  (movement, spawning, death, experience, drops, despawning, projectiles);
  `Enemy_Spawn` has a header; the despawn distance ($160) and the boss
  death steps (with Link's double) checked over the movies.
- 8f. Done: background collision; the probe tables and their Y encoding
  (dy - $20) documented, and the enemy probe docs corrected;
  `TIMER_LINK_RECOIL`.
- 8g. Done: combat (hitboxes, shield, knockback, damage, kills and drops,
  item pickup); docs/memory/objects.md has the boxes; experience bags are
  kinds $0A-$0D (docs/systems/items.md corrected).
- 8h. Done: Link (bank 0): entering, control, the fairy, walking,
  jumping, attacks, the sword beam and fire, spells and their effects.
  `linkJumpState` 2 means "jumped" (not "going up"); the attack's frame 4
  holds the sword back for 32 frames (B strikes again at once); the beam
  flies 32 frames, then sparks.  docs/systems/link.md has the attack
  table, spell effects and the fairy.
- 8i. Done: HUD, pause menu, menu window, level up (bank 0).
  `Math_Digit1000` is now `Math_ToDigits` (four digits); spell costs show
  halved; Cancel moves `experienceNext` to the next cost up (the window
  reopens); level 8 buys a life; docs/systems/menus.md has the window's
  geometry and attribute masks.  Bank 3's `Menu_*` copies remain (8l).
- 8j. Done: the overworld (banks 0, 7, 1, 2): setup, movement, the map
  decoders and PPU queues, encounters, the raft, the hammer and flute.
  New: terrain names; `overworldWalkFrames` (objYHi+13) starts encounters
  without the timer; the right-edge sprite mask; QUIRK: bank 1's palace
  stones overwrite Death Mountain's map (seen in the 100% movie).
- 8k. Done: bank 0's set pieces (North Castle's torches, side view
  start, the big door, falling, breaking blocks, Saria's bridge, the
  game's start).  $8000 is the game over screen (PPU string 9), not
  unused; bank 0 has no `@loc_` labels left.
- 8l. Done: towns (bank 3): the `Town_*` object drawers, the dialog
  window and text printing (letters every 6 frames, not 5; checked with
  a scripted conversation), the handlers, and the `Menu_*_Bank3` copies
  (comments and label names carried over from bank 0 by script, only the
  equivalent calls differing).  Bank 3 has no `@loc_` labels left.
- 8m. Done: the area banks' shared code: the object drawers
  (`Area_*`, `Palace_*`), `Area_IsSolidTile`, `Generator_Init` and the
  projectile movers and draws.  One copy was commented and the others
  filled by script where the code matches, instruction for
  instruction; variants by hand.  Bank 4's and 5's boss draws are left
  for 8p.
- 8n. Done: bank 5's screens: `Bank5_Nmi`, the title and story scroll,
  the file select, register and elimination screens, saving, and the
  ending (scenes, curtain, credits).  `Text_NumberToDigits` got bank 0's
  `Math_ToDigits` comments.  Bank 5's remaining `@loc_` labels are in its
  enemies (8p).
- 8o. Done: sound (bank 6): both engines' players, request handlers and
  effects; no `@loc_` labels left.  QUIRK: Ganon's laugh never plays its
  second sample (DmcLaugh2; seen in a scripted game over).
- 8p. The enemies' top-up: the enemy routines of banks 1-5 measure
  15-40%, below the standard set by the later ones; Gooma (18 `@loc_`),
  Barba, Rebonack, Fokka and Thunderbird have unnamed labels.
