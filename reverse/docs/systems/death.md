# Death and Continuing

When Link's life runs out, or he sinks to his death (`LINK_MODE_SINK`,
e.g. in lava), the game goes to `GAME_STATE_DEATH`:

1. `Death_StartAnimation`, `Death_Animate`: Link's hurt pose with a
   color changing every frame, for 7 x 20 frames (`timerSlow`).
2. `Death_LoseLife`: the background turns red (`Death_QueueRedPalette`),
   both meters are refilled, any dialog is closed, and a life is lost.
   - With lives left: `GAME_STATE_RESPAWN` (`GameState_Respawn`) shows the
     lives screen and reloads the area.
   - With none left: the experience is reset to 0, Ganon laughs (DMC
     sample 2), and the game over screen ("GAME OVER / RETURN OF GANON"
     over Ganon's silhouette, CHR bank $10) stays $F0 frames, or until
     Start; then the continue screen (CHR bank 6).
3. The continue screen (`Death_DrawContinueScreen`,
   `Death_ContinueMenu`): Continue or Save, chosen with Select (the
   menu cursor sound) and taken with Start.
   - **Continue**: `continueCount` is counted (stopping at 255), the
     sound engine's state is cleared, the experience is reset to 0, and play restarts from North Castle
     (`GAME_STATE_BEGIN`, which resets the lives to 3), or from the Great
     Palace's first area with 3 lives if Link died there.
   - **Save** (`Death_SaveAndQuit`): blink the choice for $40 frames
     (an attribute byte switched every 4 frames), save the game
     (`Death_SaveGame`) and go back to the title screen.

Holding Up and A on controller 2 jumps straight to the continue screen
without dying (`Game_ShowContinueScreen`), the familiar "Up+A" save.

Continuing keeps the levels, spells and items; only the experience is
lost.  See [Save files](../memory/save-files.md) for what saving keeps.

The game over and continue paths are not in the test movies; they were
checked with `tools/play.py` (from `build/movies/file_select.fm2`, with
`lives` poked to 1 and `linkDeathPending` set), for both choices and in
the Great Palace.
