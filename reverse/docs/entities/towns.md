# Towns: Townspeople

Bank 3's enemy group (`EnemyHp_Bank3` ... at $9400) holds the people of
the towns.  For townspeople the definition tables are used differently:
`sramEnemyAttr` gives the first sprite tile of the person
(`Townsperson_Draw`) and `sramEnemyDamage` the palette (bits 1-0) and,
in bit 7, a person drawn in one pose, without a walking animation; hit
points,
experience and damage do not apply, and none of them can be hurt (flags
bit 7 and 4).  What each person says is the business of the town code
(dialogs); this page covers their movement.

| Type | Person (z2edit's name) | AI |
|------|------------------------|----|
| $00 | Fairy | `Fairy_Ai` |
| $01 | Red Jar | `Item_Fall` |
| $02 | Door | `LockedDoor_Ai` |
| $03 | Purple Keese | `TownBat_Ai` |
| $04 | Purple Bot | `Bot_Ai` |
| $05 | Bit | `Bit_Ai` |
| $06 | Purple Moa | `Moa_Ai` |
| $07 | Purple Acheman(?) | `Ache_Ai` |
| $08 | Gold Girl (crash?) | `Townsperson_WalkAi` |
| $09 | Townsperson #? | `Townsperson_WalkAi` |
| $0A | Ache (talking) #0 | `Townsperson_StandAi` |
| $0B | Bit (talking) #1 | `TalkingBit_Ai` |
| $0C | Invisible Dialog (sign) #2 | `Sign_Ai` |
| $0D | Immobile Blue Woman / River Man #3 | `Townsperson_StandAi` |
| $0E | Blue Lumberjack #4 | `Townsperson_StandAi` |
| $0F | Wizard #5 | `Wizard_Ai` |
| $10 | Little Kid #6 | `Townsperson_StandAi` |
| $11 | Immobile Blue Lady W/ Bowl #7 | `Townsperson_StandAi` |
| $12 | Immobile Red Granny #8 | `Townsperson_StandAi` |
| $13 | Idle Red Lumberjack #9 | `HouseLady_Ai` |
| $14 | Idle Blue Lady W/ Bowl #10 | `HouseLady_Ai` |
| $15 | Idle Blue Granny #11 | `HouseLady_Ai` |
| $16 | Idle Red Lady #12 | `HouseLady_Ai` |
| $17 | Life Lady #13 | `HouseLady_Ai` |
| $18 | Magic Lady #14 | `HouseLady_Ai` |
| $19 | Random Blue Lumberjack #15 | `Townsperson_WalkAi` |
| $1A | Random Red Lady W/ Bowl #16 | `Townsperson_WalkAi` |
| $1B | Random Red Granny #17 | `Townsperson_WalkAi` |
| $1C | Random Blue Lady #18 | `Townsperson_WalkAi` |
| $1D | Walking Blue Lumberjack #19 | `Townsperson_WalkAi` |
| $1E | Walking Red Guy #20 | `Townsperson_WalkAi` |
| $1F | Walking Red Guy #21 | `Townsperson_WalkAi` |
| $20 | Walking Blue Lady #22 | `Townsperson_WalkAi` |
| $21 | Mirror #23 | `Sign_Ai` |
| $22 | Sign #24 | `Sign_Ai` |
| $23 | Busy Townsperson Generator | `TownspersonGenerator_Ai` |

## Townspeople

All townspeople are set up by `Townsperson_Init` (with variants for
those that start idle or walking) and can be talked to: when Link touches
one and presses B (`Townsperson_CheckTalk`), the game opens a
conversation (`dialogType` = `DIALOG_TALK`, `Townsperson_StartTalk`) and
the person turns to face him while it lasts.

- **Standing people** ($0A, $0D-$12; `Townsperson_StandAi`): stand in
  place, turning toward Link every 64 frames.
- **Talking Bit** ($0B, `TalkingBit_Ai`) and **talking Ache** ($0A):
  creatures that talk; the Ache can be talked to even while Link is in
  the air.
- **Wizard** ($0F, `Wizard_Ai`): starts talking by himself when Link
  comes within 32 pixels on the ground (he teaches the town's spell).
- **Walking people** ($19-$20; `Townsperson_WalkAi`): walk along the
  street, turning around at the edges of their range.  Some enter and
  leave houses: they walk to a door, which opens (`Door_QueueOpen`),
  go in, and the door closes behind them (`Door_QueueClosed`).
- **Idle people and the life and magic ladies** ($13-$18,
  `HouseLady_Ai`): wait near their house; the ladies ($17 life, $18
  magic) lead Link in: when he follows one through her door, the
  conversation that restores his life or magic begins (`dialogType` 4).
- **Signs and mirror** ($0C, $21, $22; `Sign_Ai`): invisible objects
  that can be talked to (read).
- **Townsperson generator** ($23, `TownspersonGenerator_Ai`): every 256
  frames, if a slot is free, sends a walking person (types $19-$1C at
  random, `TownspersonGeneratorTypes`) in from the left or right edge.

The town bank's other enemies ($03-$07) are the [common enemies](common.md)
in town colors (the town bats, `TownBat_Ai`, vanish when they leave the
screen); types $08 and $09 are unused people.  Not all of this has been
checked against the movies; see the routine headers in `src/bank3.s`.
