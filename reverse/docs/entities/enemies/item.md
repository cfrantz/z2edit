# Item

An item lying in an area: an inventory item, a key, an experience bag, a
magic jar, a container, a doll or a quest item.  Its kind is in
`enemyAiState`; the kinds and what each does are listed in
[Items and Drops](../../systems/items.md).

## Where It Appears

Type $01 in every enemy group (`Item_Fall`, `Item_Draw`).  It has no hit
points, blocks Link's projectiles, and is immune to SPELL and Thunder.
Items come from three places:

- **The area's own items** (area object $0F, `Area_PlaceItem`).  Unless
  its page's area flag shows it taken (`Area_TestFlag`), it goes in the
  highest free enemy slot.  It is placed 3 pixels into its column and
  $20 below its row, with no list entry.  Since the flag is per page, a
  page holds at most one item or locked door that stays taken.
- **Red jars in enemy lists** (type $01 there): the spawn routine
  (`RedJar_Init_Bank1`, `RedJar_Init_Bank2`) makes it kind $11, a jar
  that fills the magic meter.  Bank 2's also moves it 3 pixels right.
- **Enemy drops** (`Enemy_Kill`, every 6th kill in a drop group):
  `EnemyDropItems`, with bit 7 set.

## Behavior

**Falling** (`Item_Fall`, every frame):

1. `Enemy_UpdateWithBg`: probes, drawing, hit tests.
2. It falls under gravity of 1/16 pixel per frame per frame.
3. On a floor its Y is snapped to the tile row.  An experience bag (kinds
   $0A-$0D) stops dead.  Anything else landing at 14/16 pixel per frame
   or more bounces: 9/16 pixel per frame up, or 1 from a fall of 3 or
   more.  Slower, it stops.

**Collecting.**  How an item is collected depends on its kind:

| Kinds | Collected by | Then |
|-------|--------------|------|
| $00-$07, $0E, $0F, $12-$15 (inventory items, containers, doll, quest items) | touch (`Enemy_CheckLinkContact`) | Link stops and holds it up for 112 frames (`itemHoldTimer`, `heldItem`) |
| $08-$0D, $10, $11 (key, experience bags, magic jars) | the sword (`Enemy_TakeDamage`); touching them does nothing | at once |

A sword stroke on an item of the first group does nothing: no stun, no
hit sound.  `Item_Collect` then clears the item's page flag in the area's
flags, unless the item is a drop (bit 7).  So a red jar from an enemy
list clears its page's flag too.  Then it applies the item
([Items and Drops](../../systems/items.md#kinds)) and removes it.

## Drawing

`Item_Draw`: a symmetric pair of sprites (the right one mirrored) from
`SpritePairTiles` entry $2E + 2 × kind.  Kinds $10 and up have their own
palette (`ItemSpriteAttrs`: 3, 2, 0, 1, 1, 1).  An item one sprite wide
has $F5 as its second tile; that sprite is hidden.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Area_PlaceItem` | 7:$C9A5 | an area's item |
| `RedJar_Init_Bank1`, `RedJar_Init_Bank2` | 1:$9620, 2:$9610 | a list's red jar |
| `Item_Fall` | 7:$D959 | falling and bouncing |
| `Enemy_CheckLinkContact` | 7:$E4D9 | collecting by touch |
| `Enemy_TakeDamage` | 7:$E726 | collecting by the sword |
| `Item_Collect`, `Item_CollectExpBag`, `Item_CollectSpecial` | 7:$E771, 7:$E7F4, 7:$E80C | taking it |
| `Item_Draw` | 7:$F112 | drawing |

## Verification

Scenarios in the West Hyrule cave (area $2D; see the
[Myu](myu.md#verification) page for `$AT`):

- **Falling.**  A key, a bag and a candle (kind $80) placed in the air:
  - The key landed on frame 39 at 2.4 pixels per frame.  It bounced 3
    pixels up and settled 19 frames later.
  - The bag stopped dead.
- **Touch.**  Link touched the candle on frame 26 and `Item_Collect`
  ran.  The key lay under Link for 60 frames, touching him
  (`objCollision` bit 4), and wasn't taken.
- **Sword.**  Crouching strokes (Down+B): one took an experience bag
  (kind $8A: `Item_Collect`, `Item_CollectExpBag`), another a key
  (`keyCount` + 1), on the frame the sword met them.  Standing strokes
  passed over them.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --spawn 0:01:2C6:90 --poke enemyAiState=0x08 --poke keyCount=0 \
    --event Collect=7:E771 --event KeyGet=7:E7B5 --press 15:DOWN:40 --press 20:DOWN+B:3 --frames 80
```
