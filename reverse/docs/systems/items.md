# Items and Drops

Items are objects of enemy type 1 in an enemy slot, with their kind in
`enemyDropItem` (or placed by an area's item objects).  They fall with
gravity and bounce once when they land (`Item_Fall`), and are drawn by
`Item_Draw`.  Touching one calls `Item_Collect`.

## Kinds

| Kind | Item | Collected by |
|------|------|--------------|
| $00-$07 | the eight inventory items (candle, glove, raft, boots, flute, cross, hammer, magic key) | `Item_Collect`: set `itemOwned`, play the item fanfare |
| $08 | key | `Item_Collect`: add to `keyCount` |
| $09-$0D | experience bags | `Item_CollectExpBag`: experience of class `ExpBagClasses` |
| $0E, $0F | magic container, heart container | `Item_Collect`: add to `statContainers`, fill the meter; the 7th magic container sets `townFlags+6` bit 3 |
| $10 | blue magic jar: one block of magic | `Item_CollectSpecial` |
| $11 | red magic jar: fills the magic meter | `Item_CollectSpecial` |
| $12 | 1-up doll | `Item_CollectSpecial`: one more life |
| $13-$15 | the child, the trophy and the medicine (quest items) | `Item_CollectSpecial`: set a bit of `townFlags` |

An item that has been taken is cleared from the area's flags
(`Area_ClearFlag`), so it does not come back.

## Drops

Each enemy type belongs to one of three drop groups, or none (bits 7-6 of
its damage byte, `sramEnemyDamage`).  `Enemy_Kill` counts kills per group
in `dropKillCounts`; every 6th kill in a group makes that enemy drop an
item instead of giving experience.  The item is picked by `rng+1` (bits
2-0) from `EnemyDropItems`: entries 0-7 (mostly blue jars, one small
experience bag) for groups 1 and 3, entries 8-15 (red jars and larger
bags) for group 2.

z2edit's config calls the two halves of the table `small` and `large`,
and the 6 the `counter`.
