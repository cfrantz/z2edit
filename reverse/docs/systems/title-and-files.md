# Title, File Screens and Ending

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

In modes 3 and 4, `Title_AnimateStars` cycles the stars' palettes every
16 frames and `Title_AnimateSparkles` the sprites below the logo every 8.
When `frameCounter` wraps, one of the stars (sprite 27) becomes a
shooting star and falls to Y $37, where it is parked off screen.  Only
`Title_LoadSprites` brings it back, at the end of the story, so it falls
once per title cycle (frame 262 and frame 4614 of the idle title movie),
not every 256 frames.

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

Steps run one every 4 frames until the chain is back at step 0, so most
8-line boundaries queue a background row (step 3) and every second one
also clears two text rows and types a line.  Before the scroll starts,
`Story_Delay` runs steps 0-3 once each (from `Title_Idle`, the frame the
scroll reaches $60), so the first line is ready.  The background rows
redraw the landscape into the nametable the scroll is moving into, and
`Title_DrawBgAttr` moves the split address down 16 rows at a time ($2A00,
$2000, $2200, ...), so the landscape below the split stays put while the
text scrolls above it.  The copyright line gets the second attribute
palette (`titleAttrRow` 6).

These were checked against the idle title movie (`build/movies/title_idle.fm2`),
logging each step with the PPU strings it queued and taking screenshots.
The story starts at frame 2055 and scrolls from frame 2597.  The title is
back in mode 3 at frame 4466, with all seven logo strings redrawn
(frames 4133-4326).

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

`Save_CopyToBackup` marks a file `SAVE_STATUS_WRITING_BACKUP` ($5A) and
leaves it so, and `Save_RestoreFromBackup` marks it $69 while copying
back and $A5 (valid) after.  So a file only looked at on the file
select screen stays $5A, which `Save_CheckFiles` takes as intact at
power-on (`file_select.fm2`: all three were $5A from frame 62 until the
register screen saved them on frame 799).

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

## Ending

Defeating Dark Link (`DarkLink_StartEnding`) sets `gameState` 3;
`GameState_StartEnding` maps bank 5 and `GameState_Ending` runs, by
`gameMode`: `Ending_LoadArea` (area $3D, the room where Zelda sleeps),
`Area_Load`, `Mode_DrawArea`, `Ending_DrawCurtainTop`,
`Mode_StartSideView`, then `Ending_Run` every frame.  The ending uses the
game NMI; its variables are overlays on the object arrays (`ending*`,
`triforce*`, `curtain*`, `credits*` in `src/ram.inc`).

`Ending_Run` plays scenes by `sceneStep`.  Most scenes end with
`Ending_Wait`: 96 frames, then the next scene.  Frames are from the 100%
movie, where Dark Link falls at frame 253062:

| Scene | Routine | Frame | |
|-------|---------|-------|-|
| 0 | `Ending_HoldTriforce` | 253106 | Link holds up the Triforce before the sleeping Zelda |
| 1 | `Ending_SplitTriforce` | 253202 | it becomes three pieces |
| 2 | `Ending_Flash` | 253804 | the screen flashes |
| 3 | `Ending_WakeZelda` | 253996 | Zelda sits up; then area $3E is loaded |
| 4 | `Ending_ShowCouple` | 254092 | Link and Zelda on the stage |
| 5 | `Ending_TypeMessage` | 254231 | YOU SAVED HYRULE AND YOU ARE A REAL HERO! |
| 6 | `Curtain_Run` | 254849 | the curtain comes down part way |
| 7 | `Ending_Kiss` | 255329 | the kiss |
| 8 | `Ending_ShowTheEnd` | 255425 | THE END |
| 9 | `Curtain_Run` | 255521 | the curtain comes down to the floor |
| 10 | `Curtain_Raise` | 255681 | it rises again; the credits music starts |
| 11 | `Credits_Run` | 256224 | the credits (`CreditsText`) |
| 12 | `Ending_WaitForStart` | 260212 | the Triforce; PUSH START TO REPLAY |
| 13 | `Curtain_Run` | (+300) | after START, the curtain closes (200 frames) |
| 14 | `Ending_SetSecondQuest` | (+500) | `secondQuest` = 1, experience 0; on to `GameState_SaveAfterEnding` |

The 100% movie ends during scene 12.  Scenes 13 and 14 were run with the
no-glitch movie played to its end (Dark Link falls at frame 164926),
then no input but a START 300 frames into scene 12: scene 12 began at
frame 172916 and the START press at frame 173216.  Scene 13 ran until
frame 173417, then `Ending_SetSecondQuest` ran: `secondQuest` 1,
experience 0, `gameState` 5.  The title was back on frame 173423
(`Title_Init`, with the title song).  On the file select screen,
`FileSelect_FindSecondQuest` returned bit 0 (file 0) and the screen drew
the second quest mark, tile $D2 at Y $43, X $30, beside that file.  The
two scenes were missing from the original disassembly: the bootstrap
ended the scene table after scene 12 and left `Ending_SetSecondQuest` as
data.

`Ending_Flash` (scene 2) flashes Link's palette like a spell: `spellFlash`
counts down from $E0 for 96 frames (166508-166604 in that run), then
`Ending_Wait` gives 96 more.  The curtain is drawn a row at a time;
every 8 frames `Curtain_DrawEdge` blanks the row under it and sets the
attributes there.

The credits show 9 pages, each a title and names (`CreditsTextPtrs`; the
three PROGRAMER pages share one title string).  `Credits_FadeInText`
fades each string in through `CreditsFadePalettes`, and
`Credits_FadeOutPage` holds the page for 192 frames and fades it out.
