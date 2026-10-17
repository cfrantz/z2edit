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

## 5. Sound (bank 6)

The 8 provisional routines (`TitleSq1_VibratoSfx`, `TitleSq2_VibratoSfx`,
`TitleMusic_PlayNotes`, `Sq1_VibratoSfx`, `Sfx_SweepStep32`,
`Sfx_StopSq1AndShared`, `Sfx_StartSharedOnNoise`,
`Sfx_StartSharedOnSq1`), and the audio documentation phase
(`docs/audio/`).

## 6. Loose ends

- 23 RAM bytes still named `ram_XXXX` in `src/ram.inc`.
- The fairy's refill and flash duration (never observed).
- A red jar from an enemy list clears its page flag: consequences
  unverified.
- The `EnemyTiles_Bank1` header is inaccurate.
- The meaning of `extraSpriteAttrs` (Octorok) is provisional.
- Helmethead's head strike knocking off a helmet: confirmed only from the
  no-glitch movie, not by a scenario.
- The unhorsed Rebonack's facing is left at 0 when he spawns (minor, not
  marked).
- The orange Fokka (unused) would restart its attack every frame in the
  air (attack mask 0).

## 7. Larger documentation

- A reimplementation guide built from `docs/quirks.md`.
- A comparison of the five area banks' shared code (which routines are
  copies, and where they differ).
