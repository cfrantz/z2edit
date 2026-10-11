# Title and File Screens

The title screen, the story and the file screens are run by bank 5 from
its own NMI handler, `Bank5_Nmi`, which `Game_PowerOnInit` installs at
power on (`nmiMode` = `NMI_MODE_BANK5`).  `Bank5_Nmi` does the work of the
game NMI (sprite DMA, PPU strings from `Bank5PpuStrings`, scroll, title
music, controllers) and then runs `Bank5_RunState`, which uses `gameState`
for its own states:

| gameState | Routine | Screen |
|-----------|---------|--------|
| 0 | `Title_Run` | title screen and story |
| 1 | `FileScreen_Run` | file select, register, elimination |
| 2 | `Bank5_StartGame` | switch back to the game NMI, `gameState` = 0 |

These screens run before any game object exists, so they keep their
variables in the object slot arrays (the overlays at the end of
`src/ram.inc`: `title*`, `file*`, `letter*`, `name*`).

## Title Screen

`Title_Run` steps through `gameMode`:

| Mode | Routine | |
|------|---------|-|
| 0 | `Title_Init` | clear RAM, start the title song, set up the scroll (`Title_InitScroll`) |
| 1 | `Title_DrawScreen` | clear the nametables; queue `TitlePalette`, then `TitleScreenText` |
| 2 | `Title_Show` | load `TitleSprites`; rendering on |
| 3 | `Title_Idle` | the title screen |
| 4 | `Story_Run` | the story scroll |

In modes 3 and 4, `Title_AnimateStars` cycles the stars' palettes and
drops a shooting star every 256 frames, and `Title_AnimateSparkles`
cycles the sprites below the logo.

The screen is split with sprite 0.  Above the split the scroll is
`ppuScrollY`; below it, `Title_Idle` and `Story_Run` busy-wait for the
sprite 0 hit and write `titleSplitAddr` to PPUADDR, which sets the scroll
of the lower part to that nametable row.

The title is timed by the title song.  The music player in bank 6 keeps
the part being played in `titleMusicPart`: 1 (the intro), then 2, 4, 8,
$10 and back to 2.  `Title_Idle` scrolls the screen up a line every 4
frames during part 2 until `ppuScrollY` is $60, then waits, and starts the
story (mode 4) when part 8 begins.  In the idle title movie that is about
2050 frames after power on; the story then takes about 2400 frames and
returns to mode 3, and the cycle repeats every 3584 frames, with the song.

START (`Title_CheckStart`, checked every frame of modes 3 and 4) goes to
the file screens.

## Story

`Story_Run` scrolls the screen up one line every 4 frames (after
`titleDelay`, which `Story_TickDelay` counts down every 4 frames), and
every 8 lines runs one step of the scene, by `sceneStep`.  Each step
queues one row coming into view:

| Step | Routine | |
|------|---------|-|
| 0 | `Story_ClearTextRows` | two blank text rows (on a 16-line boundary; else go to step 3) |
| 1 | `Story_DrawTextLine` | one line of `StoryText` (15 lines of 29 tiles) |
| 2 | `Story_DrawTextAttr` | after every second line, a row of `StoryAttrRows` |
| 3 | `Title_DrawBgRow` | a background row (back to step 0, except after rows 16 and 30) |
| 4 | `Title_DrawBgAttr` | background attributes; moves the split down |
| 5 | `Story_End` | scroll the title back into view |

The text rows go to the nametables at $2800 (`titleTextAddr`), the
background rows to $2000 (`titleBgAddr`); each wraps to the other
nametable at its end (horizontal mirroring, set by `Title_Init`).

The background rows are not stored separately: `TitleBgRowPtrsLo`/`Hi`
point at the tiles of the row strings inside `TitleScreenText`, and
`TitleLogoPtrs` at its logo strings, which `Story_RedrawLogoRow` queues
again at the end of the story as their rows come into view.

## File Screens

`FileScreen_Run` (gameState 1) runs, by `gameMode`, `FileSelect_Run`,
`Register_Run` or `Eliminate_Run`, each a sequence of `fileStep`s:

| fileStep | File select | Register your name / Elimination mode |
|----------|-------------|---------------------------------------|
| 0 | `FileSelect_LoadFiles`: copy the save files to their backups | `FileScreen_Blank` |
| 1 | `FileScreen_CopyText` | `FileScreen_CopyText` |
| 2 | `FileSelect_Menu` | `Register_Menu` / `Eliminate_Menu` |
| 3 | | `FileScreen_SaveFiles`: copy the backups back to the files |
| 4 | | `FileScreen_ReturnToSelect` |

All three screens work on the backup copies (`sramBackupStats`; see
[Save files](../memory/save-files.md)).  A file is empty when its name is
all blank ($F4); `FileSelect_FindEmpty` sets a bit per empty file in
`fileEmptyMask`.

**File select.**  `FileScreen_CopyText` copies `FileScreenText` into SRAM
(`sramFileScreenText`), where `FileSelect_DrawLevels` writes each file's
attack, magic and life levels and `FileSelect_DrawContinues` its
`continueCount` (three digits, `Text_NumberToDigits`);
`FileSelect_BlankEmptyFiles` blanks those of empty files.  SELECT moves
the fairy (`FileScreen_DrawFairy`) to the next choice that applies: an
existing file, REGISTER YOUR NAME (if a file is empty), ELIMINATION MODE
(if a file exists).  START loads the chosen file (`Save_Load`) and starts
the game (gameState 2).

**Second quest.**  When the game is won, the ending's last step
(`Ending_SetSecondQuest`, after START at the end of the credits) sets
`secondQuest` to 1 before the game is saved.  When such a file is next
loaded, `FileSelect_Menu` resets its stats from the containers on (items,
keys, crystals, ...) and its world state to the defaults, keeps the
levels, the spells and the techniques bits $14, and sets `secondQuest` to
2.  The file select screen marks second quest files with a sprite (tile
$D2; `FileSelect_FindSecondQuest`).

**Register your name.**  SELECT moves the fairy over the empty files and
END.  The direction pad moves the letter cursor over `NameLetters`, a grid
of 11 x 4 letters with a gap in row 2, repeating after the pad is held for
31 frames; A writes the letter at the name cursor and moves it on, B
moves it on.  START on END saves the files.

**Elimination mode.**  START on a file resets its backup copy to the
defaults (`SaveDefaultStats`, `SaveDefaultWorld`), which blanks its name.
START on END goes to the register screen (so steps 3 and 4 of
`Eliminate_Run` are never used); the eliminations are saved when END is
chosen there.
