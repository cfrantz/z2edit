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
- 8h. Link (bank 0, ~920): `Link_Control`, `Link_UpdateJump`,
  `Link_UpdateXVelocity`, attacks, `Link_ReadInput`, `Link_EnterArea`,
  palette, `Spell_*` effects, `LinkProjectile_*`.
- 8i. HUD, pause, menus and level up (bank 0, ~1340): `Hud_*`,
  `Menu_*_Bank0` (bank 3 has copies, 8l), `Pause_*`, `LevelUp_*`,
  `Exp_*`, `Math_Digit1000`.
- 8j. The overworld (banks 0, 7, 1, 2, ~1430 plus ~230 in banks 1-2):
  `Mode_Overworld`, movement, `Overworld_CheckEncounter` and the
  encounter sprites, the raft, the map decoders and the row/column/
  attribute queues, and bank 7's `Overworld_CheckSpecialTerrain`.
- 8k. Bank 0's remaining set pieces (~470): `NorthCastle_DrawSprites`,
  `SideView_Start`, `BigDoor_*`, `Mode_FallAnimate` and `Fall_*`,
  `BreakingBlock_Update`, `Saria_BuildBridge`,
  `Game_StartAtNorthCastle`.

The other banks:

- 8l. Towns and conversations (bank 3, ~1600): `Town_*` objects,
  `Dialog_*`, `Menu_*_Bank3`, and the townspeople (`Townsperson_*`,
  `HouseLady_*`, `Door_*`, ~23% commented).
- 8m. The area banks' shared code (banks 1-5, ~1500 before
  de-duplication): `Area_*` objects, `Palace_*`, `Projectile_*` and the
  generators.
- 8n. Bank 5's screens (~2300): `Bank5_Nmi`, `Title_*`, `Story_*`,
  `FileScreen_*`, `FileSelect_*`, `Register_*`, `Eliminate_*`, `Save_*`,
  and the ending (`Ending_*`, `Curtain_*`, `Credits_*`).
- 8o. Sound (bank 6, ~1700, no comments, 155 `@loc_` labels): both
  engines' players, request handlers and the sound effects.
- 8p. The enemies' top-up: the enemy routines of banks 1-5 measure
  15-40%, below the standard set by the later ones; Gooma (18 `@loc_`),
  Barba, Rebonack, Fokka and Thunderbird have unnamed labels.
