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

## 6. Loose ends

- The meaning of `extraSpriteAttrs` (Octorok) is provisional.
- Helmethead's head strike knocking off a helmet: confirmed only from the
  no-glitch movie, not by a scenario.
- The unhorsed Rebonack's facing is left at 0 when he spawns (minor, not
  marked).

## 7. Larger documentation

- A reimplementation guide built from `docs/quirks.md`.
- A comparison of the five area banks' shared code (which routines are
  copies, and where they differ).
