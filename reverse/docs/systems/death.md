# Death and Continuing

When Link's life runs out, or he sinks to his death (`LINK_MODE_SINK`,
e.g. in lava), the game goes to `GAME_STATE_DEATH`:

1. `Death_StartAnimation`, `Death_Animate`: Link's death pose with a
   flashing palette.
2. `Death_LoseLife`: the background turns red (`Death_QueueRedPalette`),
   both meters are refilled, any dialog is closed, and a life is lost.
   - With lives left: `GAME_STATE_RESPAWN` (`GameState_Respawn`) shows the
     lives screen and reloads the area.
   - With none left: the experience is reset to 0 and the game over
     screen is shown, then the continue screen.
3. The continue screen (`Death_DrawContinueScreen`,
   `Death_ContinueMenu`): Continue or Save.
   - **Continue**: `continueCount` is counted (stopping at 255), the
     sound engine's state is cleared, the experience is reset to 0, and play restarts from the North Palace
     (`GAME_STATE_BEGIN`, which resets the lives to 3), or from the Great
     Palace's first area if Link died there.
   - **Save** (`Death_SaveAndQuit`): blink the choice, save the game
     (`Death_SaveGame`) and go back to the title screen.

Holding Up and A on controller 2 jumps straight to the continue screen
without dying (`Game_ShowContinueScreen`), the familiar "Up+A" save.

Continuing keeps the levels, spells and items; only the experience is
lost.  See [Save files](../memory/save-files.md) for what saving keeps.
