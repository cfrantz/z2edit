# The Side View Frame

During side view play (`gameMode` = `GAME_MODE_SIDE_VIEW`), the game NMI
runs `Mode_SideView` (bank 7) once per frame, which does, in order:

1. If `linkDeathPending` is set, start the death sequence
   (`GAME_STATE_DEATH`) and stop.
2. `Spell_Update` (bank 0): cast spells and run the effects of active
   ones; see [Link](link.md).
3. Refill the magic or life meter from `statMeterPending` (alternating
   frames, 2 per step), and add `experiencePending` to `experience` (10 per
   frame), with sound.
4. Status bar split: unless a conversation (`DIALOG_TALK`) is open, wait
   for the sprite 0 hit and switch to the playfield's nametable and scroll
   (`playfieldNametable`).  The status bar above is not scrolled.
5. `Hud_Update`: check for a level up (`DIALOG_LEVEL_UP`) and redraw the
   status bar.  Beep while life is low.
6. `Link_Run`: Link's movement; see [Link](link.md).
7. `Scroll_Update`: scroll to follow Link.
8. `Enemies_Update` (with the area bank mapped): the six enemies and their
   projectiles.  Sprite slots are rotated each frame to spread flicker.
9. `NorthCastle_DrawSprites`, `LinkProjectile_Update` (the sword beam and
   fire) and `BreakingBlocks_Update`.
10. `Link_Draw`, skipped on alternate frames while Link blinks after being
    hit; `Sword_CheckBlocks`.

Enemies are updated through per-type routine tables copied into SRAM from
the area bank (`sramEnemyAiPtrs`, `sramEnemyDrawPtrs`,
`sramProjectilePtrs`), so the same engine code runs each world's enemies.
Collision is covered in [object slots](../memory/objects.md#collision).
