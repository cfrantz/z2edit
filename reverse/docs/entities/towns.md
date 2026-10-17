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
| $00 | [Fairy](enemies/fairy.md) | `Fairy_Ai` |
| $01 | [Red Jar](enemies/item.md) | `Item_Fall` |
| $02 | [Door](enemies/locked-door.md) | `LockedDoor_Ai` |
| $03 | [Town bat](enemies/town-bat.md) | `TownBat_Ai` |
| $04 | [Purple Bot](enemies/bot.md) | `Bot_Ai` |
| $05 | [Bit](enemies/bit.md) | `Bit_Ai` |
| $06 | [Purple Moa](enemies/moa.md) | `Moa_Ai` |
| $07 | [Purple Acheman(?)](enemies/ache.md) | `Ache_Ai` |
| $08 | [(unused walker)](enemies/townsperson.md) | `Townsperson_WalkAi` |
| $09 | [(unused walker)](enemies/townsperson.md) | `Townsperson_WalkAi` |
| $0A | [Ache (talking) #0](enemies/talking-creatures.md) | `Townsperson_StandAi` |
| $0B | [Bit (talking) #1](enemies/talking-creatures.md) | `TalkingBit_Ai` |
| $0C | [Invisible Dialog (sign) #2](enemies/townsperson.md) | `Sign_Ai` |
| $0D | [Immobile Blue Woman / River Man #3](enemies/townsperson.md) | `Townsperson_StandAi` |
| $0E | [Blue Lumberjack #4](enemies/townsperson.md) | `Townsperson_StandAi` |
| $0F | [Wizard #5](enemies/town-wizard.md) | `Wizard_Ai` |
| $10 | [Little Kid #6](enemies/townsperson.md) | `Townsperson_StandAi` |
| $11 | [Immobile Blue Lady W/ Bowl #7](enemies/townsperson.md) | `Townsperson_StandAi` |
| $12 | [Immobile Red Granny #8](enemies/townsperson.md) | `Townsperson_StandAi` |
| $13 | [Idle Red Lumberjack #9](enemies/house-people.md) | `HouseLady_Ai` |
| $14 | [Idle Blue Lady W/ Bowl #10](enemies/house-people.md) | `HouseLady_Ai` |
| $15 | [Idle Blue Granny #11](enemies/house-people.md) | `HouseLady_Ai` |
| $16 | [Idle Red Lady #12](enemies/house-people.md) | `HouseLady_Ai` |
| $17 | [Life Lady #13](enemies/house-people.md) | `HouseLady_Ai` |
| $18 | [Magic Lady #14](enemies/house-people.md) | `HouseLady_Ai` |
| $19 | [Random Blue Lumberjack #15](enemies/townsperson.md) | `Townsperson_WalkAi` |
| $1A | [Random Red Lady W/ Bowl #16](enemies/townsperson.md) | `Townsperson_WalkAi` |
| $1B | [Random Red Granny #17](enemies/townsperson.md) | `Townsperson_WalkAi` |
| $1C | [Random Blue Lady #18](enemies/townsperson.md) | `Townsperson_WalkAi` |
| $1D | [Walking Blue Lumberjack #19](enemies/townsperson.md) | `Townsperson_WalkAi` |
| $1E | [Walking Red Guy #20](enemies/townsperson.md) | `Townsperson_WalkAi` |
| $1F | [Walking Red Guy #21](enemies/townsperson.md) | `Townsperson_WalkAi` |
| $20 | [Walking Blue Lady #22](enemies/townsperson.md) | `Townsperson_WalkAi` |
| $21 | [Mirror #23](enemies/townsperson.md) | `Sign_Ai` |
| $22 | [Sign #24](enemies/townsperson.md) | `Sign_Ai` |
| $23 | [Busy Townsperson Generator](enemies/townsperson.md) | `TownspersonGenerator_Ai` |

## Townspeople

- **[Townspeople](enemies/townsperson.md)**: standing people ($0D, $0E,
  $10-$12) turn toward Link every 64 frames; walkers ($1D-$20, and $19-$1C
  sent onto the street every 256 frames by the generator, $23) walk until
  they leave; signs and the mirror ($0C, $21, $22) are invisible and can
  be read.  Link talks to anyone by touching them and pressing B.
- **[House people](enemies/house-people.md)** ($13-$18): pace in front of
  their house and go in and out through its door; the life ($17) and
  magic ($18) ladies lead Link in to restore his life or magic.
- **[Town wizard](enemies/town-wizard.md)** ($0F): speaks first when Link
  comes near, and gives the town's spell.
- **[Talking creatures](enemies/talking-creatures.md)** ($0A Ache, $0B
  Bit): asleep until asked four times.
- **[Town bat](enemies/town-bat.md)** ($03): what a walker of Saria or
  Darunia may turn into after talking.

The town bank's other enemies ($04-$07) are the [common enemies](common.md)
in town colors.  Types $08 and $09 are unused walkers.
