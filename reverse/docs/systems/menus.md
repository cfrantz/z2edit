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
3. `Menu_ComputeAttrs_Bank0` builds the attribute bytes under the window
   (`menuAttrs`) from the palette bits of the area tiles there.
4. The window opens a row per frame (`Pause_OpenRow`, `LevelUp_OpenRow`):
   each row's tiles come from row pointer tables (`PauseMenuRowPtrs`,
   `LevelUpRowPtrs` and their second-row tables) pointing into
   `MenuRowTemplates` and `sramMenuText`, and are queued with their
   attribute bytes by `Menu_DrawRowAttrs_Bank0` in `ppuBuffer1`.
5. When the menu closes, `Menu_CloseRow_Bank0` redraws each row from the area
   tile map, and `Menu_Close` clears the menu state.

While a menu is open, the screen is split under the status bar by the
menu code itself (`Hud_SplitScreen`, or the timed loop at the start of
`LevelUp_Update`).

The window is 7 metatiles (112 pixels) wide, from the metatile at
`scrollX` + $78, and 10 rows of 16 pixels from Y $30 (map rows 1-10).
Each row is two tile rows of 14 tiles (`menuRowTiles`, `menuRowTiles2`),
queued as one string per nametable when the window crosses into the
other (`Menu_QueueRowTiles_Bank0`).  Its attribute bytes are the area's
(`menuAttrs`, 4 columns by 6 rows) masked to palette 0 in the window's
quadrants only: the window covers half of its first or last attribute
column (`Menu_CheckAttrAlignment_Bank0`), and each row is the top or
bottom half of its attribute byte.  Rows open from the top and close from
the bottom; closing restores the closed row's quadrants and keeps palette
0 in the half still covered.

Menu cursors (`Menu_MoveCursor`) move when Up or Down is first pressed,
then every 7 frames once held for 16 (checked with `tools/play.py`,
holding Down in the pause menu: frames 0, 16, 23, 30, ...).

## Pause Menu

`pauseState` in side view (`Pause_RunState`):

| State | Step |
|-------|------|
| 0 | not paused: Start pauses (`Pause_Update`) |
| 1 | `Menu_CopyTextTemplate` |
| 2 | `Pause_FillText` |
| 3 | `Menu_ComputeAttrs_Bank0` |
| 4 | `Pause_OpenRow`, until 10 rows are drawn |
| 5 | `Pause_MenuOpen`: Up/Down choose a spell (`spellCursor`); Start closes; Up+A on controller 2 goes to the continue screen (`Game_ShowContinueScreen`) |
| 6 | `Menu_ComputeAttrs_Bank0` |
| 7 | `Menu_CloseRow_Bank0` |
| 8 | `Menu_Close` |

The menu lists the lives in reserve (`lives` - 1), the keys, the
crystals still to place, and each owned spell with its cost at the
current magic level, halved (the menu counts 2 meter units as 1: Shield
shows 032 at level 1).  Spells not owned are blank rows; the arrow ($FA)
skips them, wrapping around.  Opening the menu also lets the last spell
be cast again (`spellLastCast`).

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
| 3 | `Menu_ComputeAttrs_Bank0` |
| 4 | `LevelUp_OpenRow` |
| 5 | `LevelUp_ChooseStat`: Up/Down move `levelUpCursor` over the stats that can be raised and Cancel; Start raises the stat (at level 8, adds a life instead) and takes its cost from `experience` |
| 6 | `LevelUp_ApplyChoice` |
| 7 | `LevelUp_BlinkChoice`: blink the choice while the fanfare plays |
| 8 | `Menu_ComputeAttrs_Bank0` |
| 9 | `Menu_CloseRow_Bank0` |
| 10 | `Menu_Close` |

Level costs are in `LevelUpCostHi`/`LevelUpCostLo`; the window shows them
from the digit tables `LevelUpCostThousands`, `LevelUpCostHundreds` and
`LevelUpCostTens`.  The cursor starts on the first stat (attack, magic,
life) that can be raised and skips those that can't.  Choosing a stat
plays the item fanfare, refills its meter (magic or life), raises it and
draws its level bar one tile longer, blinking it (8 frames on, 8 off)
while the fanfare plays; `experienceNext` becomes the cheapest next
level.  At level 8 the 9000 experience buys a life (up to 127).

Cancel sets `experienceNext` to the next cost above the current one
(`Exp_UpdateAfterLevelUp`), so with enough experience the window opens
again at once.  Checked with `tools/play.py` from North Castle with 250
experience: Cancel moved `experienceNext` from 50 to 100 and then 200,
reopening the window each time; with attack at level 8 and 9000
experience, choosing attack added a life and left 0 experience.  When no
cost is above `experienceNext`, Cancel is skipped (from the code).

## Status Bar

`Hud_Update` (each side view frame) opens the level up window when
`experience` reaches `experienceNext`, then redraws the halves of the
status bar flagged in `hudRedraw` (bit 7 the left half, bit 6 the right),
one half per frame, alternating when both are (`hudHalf`), and only when
`ppuBuffer0` has room:

- left (`HudLeftTemplate`): the magic level, the attack level and the
  magic meter;
- right (`HudRightTemplate`): the life level, the life meter, the
  experience and `experienceNext` (`Math_ToDigits`); the next level's
  ones digit is not drawn (the name table's '0'; every cost is a
  multiple of 50).

A meter has one block per container: empty, or full per 32 units.  The
remainder is shown by a sprite behind the background at the fill's end
(`Hud_DrawMeterSprites`, sprites 62 and 63).

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
