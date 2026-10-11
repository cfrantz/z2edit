# Menus and Dialogs

The pause menu, the level up window, the flute and town conversations are
run by `Game_UpdateMenus` from the game NMI, on frames when the current
mode asked for them by setting `menusPending` (both `Mode_SideView` and
`Mode_Overworld` do).  They are state machines on `pauseState`; while
`pauseState` is nonzero the game is paused (timers and the RNG stop).

`Game_UpdateMenus` chooses:

- in side view (`GAME_MODE_SIDE_VIEW`), by `dialogType`:
  - 0: the pause menu (`Pause_Update`);
  - 1 (`DIALOG_LEVEL_UP`): the level up window (`LevelUp_Update`);
  - 2-4: a town conversation, run by the town bank
    (`Menu_UpdateTownDialog`);
- otherwise (`Menu_UpdateOther`): the flute while `fluteActive` is set
  (`Flute_Update`), or else the overworld pause.

Before that, in one area of Saria, `Game_UpdateSideViewMenus` runs the
bridge building event (`Saria_BuildBridge`).

## The Window

The pause menu and the level up window are drawn into the nametable over
the playfield, a row at a time, and erased the same way:

1. `Menu_CopyTextTemplate` copies `MenuTextTemplate` (280 bytes) into
   SRAM (`sramMenuText`).
2. The text is filled in: `Pause_FillText` (keys, crystals, lives, spell
   costs) or `LevelUp_FillText` (next level costs, prompts, level bars).
3. `Menu_ComputeAttrs` builds the attribute bytes under the window
   (`menuAttrs`) from the palette bits of the area tiles there.
4. The window opens a row per frame (`Pause_OpenRow`, `LevelUp_OpenRow`):
   each row's tiles come from row pointer tables (`PauseMenuRowPtrs`,
   `LevelUpRowPtrs` and their second-row tables) pointing into
   `MenuRowTemplates` and `sramMenuText`, and are queued with their
   attribute bytes by `Menu_DrawRowAttrs` in `ppuBuffer1`.
5. When the menu closes, `Menu_CloseRow` redraws each row from the area
   tile map, and `Menu_Close` clears the menu state.

While a menu is open, the screen is split under the status bar by the
menu code itself (`Hud_SplitScreen`, or the timed loop at the start of
`LevelUp_Update`).

## Pause Menu

`pauseState` in side view (`Pause_RunState`):

| State | Step |
|-------|------|
| 0 | not paused: Start pauses (`Pause_Update`) |
| 1 | `Menu_CopyTextTemplate` |
| 2 | `Pause_FillText` |
| 3 | `Menu_ComputeAttrs` |
| 4 | `Pause_OpenRow`, until 10 rows are drawn |
| 5 | `Pause_MenuOpen`: Up/Down choose a spell (`spellCursor`); Start closes; Up+A on controller 2 goes to the continue screen (`Game_ShowContinueScreen`) |
| 6 | `Menu_ComputeAttrs` |
| 7 | `Menu_CloseRow` |
| 8 | `Menu_Close` |

On the overworld there is no window: Start pauses and mutes the sound
(`Pause_OverworldMute`, `soundMute`), Start again resumes
(`Pause_OverworldWait`, `Pause_OverworldUnmute`), and Up+A on controller
2 goes to the continue screen.  Neither test movie pauses on the
overworld; this was checked with `tools/play.py`.

## Level Up Window

Opened by `Hud_Update` when `experience` reaches `experienceNext`.
`pauseState` (`LevelUp_Update`):

| State | Step |
|-------|------|
| 0 | `Menu_Skip` |
| 1 | `Menu_CopyTextTemplate` |
| 2 | `LevelUp_FillText`: for each stat (attack, magic, life), its next cost and, where experience is enough, the prompt |
| 3 | `Menu_ComputeAttrs` |
| 4 | `LevelUp_OpenRow` |
| 5 | `LevelUp_ChooseStat`: Up/Down move `levelUpCursor` over the stats that can be raised and Cancel; Start raises the stat (at level 8, adds a life instead) and takes its cost from `experience` |
| 6 | `LevelUp_ApplyChoice` |
| 7 | `LevelUp_BlinkChoice`: blink the choice while the fanfare plays |
| 8 | `Menu_ComputeAttrs` |
| 9 | `Menu_CloseRow` |
| 10 | `Menu_Close` |

Level costs are in `LevelUpCostHi`/`LevelUpCostLo`; the window shows them
from the digit tables `LevelUpCostThousands`, `LevelUpCostHundreds` and
`LevelUpCostTens`.

## Flute

Playing the flute on the overworld (B, with the flute) sets `fluteActive`
and `flutePlayed`.  `Flute_Update` then steps through `pauseState`:
`Menu_Skip`, `Flute_Start` (`menuTimer` = $88 frames), `Flute_Wait`, and
`Menu_Close`; Link cannot move meanwhile.  Afterwards
`Overworld_HandleInput` applies the flute's effect at his position
(`Overworld_UseFlute`).

## Continue Screen

See the death states in [main loop and NMI](../architecture/main-loop-and-nmi.md).
`Death_ContinueMenu`: Select moves `menuCursor` between Continue and
Save; Start on Continue restarts (at the Great Palace if Link is there),
on Save blinks the choice (`Death_BlinkSaveChoice`), saves and returns to
the title screen.  Checked with `tools/play.py`.
