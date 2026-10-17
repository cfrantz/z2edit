# TODO

Remaining work, roughly in the suggested order.  The enemies of every
area bank are documented; what is left is mostly routines still marked
`Status: provisional`.

## 1. Area drawing (banks 1-5)

The routines that draw an area's background objects, about 15 per bank,
sharing a pattern; verify with screenshots and `enemy_scenario.py --map`.

- Banks 1 and 2: `Area_DrawObjectRow_*`, `Area_DrawObjectTwoRows_*`,
  `Area_DrawObjectColumn_*`, `Area_DrawObjectPillar_*`, cactus, lava,
  tree stump, crosses, headstone, stonehenge, pit, sleeping Zelda, cloud
  (bank 2 also `Area_DrawPillarSlope_Bank2`).
- Bank 3: `Town_DrawLargeObject`, `Town_DrawSmallObject`, the roof, wall
  and slant routines, `Town_SetFloorTiles`, `Area_PlaceElevator_Bank3`,
  `Area_PlaceLockedDoor_Bank3`.
- Banks 4 and 5: `Palace_SetFloorTiles_*`, `Palace_DrawObject*`,
  `Palace_DrawLava_*`, `Palace_DrawPit_Bank4`, `Palace_DrawCloud_*`,
  `Palace_DrawCrystalStatue`, `Palace_DrawElevatorShaft`,
  `Palace_DrawSleepingZelda`.

## 2. Projectiles and boss leftovers (banks 1, 2, 4)

- The projectile drawing routines of banks 1, 2 and 4 (Flash, Flicker,
  Spin, Bob, Flip, Large, Fire, Flame), as done for bank 5.
- Bank 2's and bank 4's projectile movement (`Projectile_Straight_Bank2`,
  `Projectile_Rock_*`, `Projectile_Fireball_*`, `Projectile_MoveAndCheck_*`
  and the rest).
- Bank 4: `Barba_Breathe`, `Barba_Sink`, `RebonackUnhorsed_Ai`,
  `Barba_Draw`, `Rebonack_Draw`, `Carock_Draw`,
  `Sprite_DrawEnemyPair3_Bank4`.
- `Generator_Init_Bank2` (same code as the confirmed bank 3 and 5 copies)
  and the `Enemy_InitNone_*` stubs.

## 3. Town conversations (bank 3)

- `Dialog_TechniqueMan`, `Dialog_Type0E`, `Dialog_SetTalked`,
  `Dialog_CheckTownFlag`, `Dialog_Mirror`, `Dialog_Type1D` (the town flag
  that is never set), `Dialog_SleepyCreature`, `Dialog_TextDone`,
  `Dialog_Reward`, `Dialog_WaitForB`.
- Untested on the townspeople pages: talking to the Ache from mid-air,
  the four conversations that wake the Ache and Bit, a walker turning
  into a bat (`Dialog_MaybeTurnToBat`).

## 4. Title, story, ending and saves (bank 5)

`Title_Init`, `Title_AnimateStars`, `Title_AnimateSparkles`, the
`Story_*` routines, `Title_DrawBgRow`, `Title_DrawBgAttr`,
`Ending_Flash`, `Curtain_DrawEdge`, `Ending_SetSecondQuest`,
`FileScreen_DrawLinkIcons`, `FileSelect_FindSecondQuest`,
`Save_RestoreFromBackup`, `Save_CopyToBackup`.  The no-glitch movie plays
the ending (run on past its end; see the Dark Link page).

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
