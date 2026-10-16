; Game-wide constants and enumerations.

.ifndef CONSTANTS_INC
CONSTANTS_INC = 1

; nmiMode values.  Nmi dispatches on bits 7 and 6.
NMI_MODE_SOUND_ONLY     = $40   ; bit 7 clear: Nmi_SoundOnly
NMI_MODE_BANK5          = $80   ; bit 7 set, bit 6 clear: Nmi_Bank5
NMI_MODE_GAME           = $C0   ; bits 7 and 6 set: game NMI

; gameState values during the game (bank 5 uses other values on the title
; screens).
GAME_STATE_BEGIN        = 0     ; start or continue: lives = 3
GAME_STATE_PLAY         = 1
GAME_STATE_DEATH        = 2     ; Link died, or the continue screen was requested
GAME_STATE_START_ENDING = 3
GAME_STATE_ENDING       = 4     ; bank 5's ending sequence
GAME_STATE_SAVE_AFTER_ENDING = 5 ; save, then back to the title screen
GAME_STATE_RESPAWN      = 6     ; restart after losing a life

; gameMode values in GAME_STATE_PLAY.
GAME_MODE_OVERWORLD     = $05
GAME_MODE_LOAD_AREA     = $08   ; Main_Loop loads the area being entered
GAME_MODE_SIDE_VIEW     = $0B
GAME_MODE_LOAD_ROOM     = $14   ; Main_Loop loads a connected room

; dialogType values.
DIALOG_NONE             = 0
DIALOG_LEVEL_UP         = 1
DIALOG_TALK             = 2     ; (provisional)

; techniques bits.
TECH_UPWARD_THRUST      = $04
TECH_DOWNWARD_THRUST    = $10

; linkMode values.
LINK_MODE_ENTER         = 0     ; being placed in a new area
LINK_MODE_CONTROL       = 1     ; under the player's control
LINK_MODE_SINK          = 2     ; sinking to his death (e.g. in lava)

; ppuStringSelect values that select the RAM buffers.
PPU_STRING_BUFFER0      = 0
PPU_STRING_BUFFER1      = 1
PPU_STRING_BUFFER2      = 2

; The area tile map (sramAreaTiles) is 4 pages of 13 rows of 16 tiles.
AREA_PAGE_SIZE          = $D0

; Indexes into timerFast and timerSlow.
TIMER_LINK_ENTER        = 2     ; timerFast: Link walking into an area
TIMER_ENEMY             = 3     ; timerFast: 3-8, one per enemy
TIMER_LINK_ATTACK       = 9     ; timerFast: Link's attack animation frame time
TIMER_LINK_HURT         = 11    ; timerFast: Link knocked back after being hit
TIMER_BOSS              = 12    ; timerFast: bosses (the 255-frame wait at the start of a fight in palaces 3, 4 and 6; the Great Palace bosses)
TIMER_INVULNERABLE      = 10    ; timerSlow: Link invulnerable (and blinking at 1-2)

; Object slots: indexes into the obj* arrays in ram.inc.
OBJ_SLOT_LINK           = 0
OBJ_SLOT_ENEMY          = 1     ; 1-6: enemies
OBJ_SLOT_PROJECTILE     = 7     ; 7-12: enemy projectiles
OBJ_SLOT_LINK_PROJECTILE = 13   ; 13-14: Link's projectiles
OBJ_SLOT_SWORD_PROBE    = 16    ; sword tip, for breaking blocks
NUM_ENEMIES             = 6
NUM_PROJECTILES         = 6

; Special enemyType values (other types are defined per world).
ENEMY_TYPE_ITEM         = $01   ; an item lying in the area
ENEMY_TYPE_ELEVATOR     = $13

; enemyActive values.
ENEMY_STATE_EMPTY       = 0
ENEMY_STATE_ACTIVE      = 1
ENEMY_STATE_DYING       = 2
ENEMY_STATE_SHOW_EXP    = 3     ; showing the experience it was worth

; Indexes into statLevel.
STAT_ATTACK             = 0
STAT_MAGIC              = 1
STAT_LIFE               = 2

; Indexes into statMeter and statContainers.
STAT_METER_MAGIC        = 0
STAT_METER_LIFE         = 1

; Indexes into spellOwned.
SPELL_SHIELD            = 0
SPELL_JUMP              = 1
SPELL_LIFE              = 2
SPELL_FAIRY             = 3
SPELL_FIRE              = 4
SPELL_REFLECT           = 5
SPELL_SPELL             = 6
SPELL_THUNDER           = 7

; Indexes into itemOwned.
ITEM_CANDLE             = 0
ITEM_GLOVE              = 1
ITEM_RAFT               = 2
ITEM_BOOTS              = 3
ITEM_FLUTE              = 4
ITEM_CROSS              = 5
ITEM_HAMMER             = 6
ITEM_MAGIC_KEY          = 7

; Save files.  A file holds SAVE_WORLD_SIZE bytes from saveWorld and
; SAVE_STATS_SIZE bytes from statLevel.
SAVE_STATS_SIZE         = 50
SAVE_WORLD_SIZE         = 224
; Save file status.  Saving writes the backup copy, then the file, so an
; interrupted save can be completed or discarded at power-on.
SAVE_STATUS_WRITING_BACKUP = $5A  ; file intact; backup being written
SAVE_STATUS_WRITING_FILE = $69    ; backup complete; file being written
SAVE_STATUS_VALID       = $A5

.endif
