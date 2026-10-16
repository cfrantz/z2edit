# Locked Door

A locked door that bars Link's way until he opens it with a key or the
magic key.  It stays open from then on.

## Where It Appears

Type $02 in every enemy group (`LockedDoor_Ai`, `LockedDoor_Draw`).  It
can't be hit (`Enemy_CheckHit` skips type 2), blocks Link's projectiles,
and is immune to SPELL and Thunder.

It isn't placed by enemy lists but by the area: small object $05
(`Area_PlaceLockedDoor_BankN`) records its column in `lockedDoorPos`.
`Area_PlaceElevatorAndDoor` then checks its page's area flag
(`Area_TestFlag`): if the door hasn't been opened, it goes in enemy slot
1 at that column, Y $A0, with no list entry and 255 hit points.  Doors
are in the palaces (and towns' area data has the object too).

## Behavior

`LockedDoor_Ai`, after `Enemy_UpdateCommon`:

1. **Shut** (`enemyAiState` 0).  When Link touches it:
   - he is pushed back (`Enemy_KnockBack`), unless he is a fairy
     (`linkFairy`);
   - he can't move toward it this frame (`blockedDirections`);
   - with the magic key, or a key (`keyCount`, used up), it opens.
2. **Opening.**  Its page's area flag is cleared (`Area_ClearFlag`), so it
   stays open.  A sound plays ($80), the game freezes (`gameFrozen`), and
   Link stops.  `enemyAiState` counts the frames.
3. **Open.**  After 17 frames it leaves a spark at its place (projectile
   slot 0, `objState` $F0, Y $AC) with a sound, unfreezes the game and is
   removed.

## Drawing

`LockedDoor_Draw`: three sprites in a column at its X, palette 1:

- the keyhole panel, tile $76, at Y + 15;
- the bottom bar, tile $74, at Y + 31, rising by `enemyAiState` while it
  opens;
- the top bar, tile $74, at Y - 1, dropping by `enemyAiState` while it
  opens.

So the bars close in on the keyhole and the door vanishes.  It isn't
drawn when its column is off screen.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Area_PlaceLockedDoor_Bank1` | 1:$82C1 | area object: the column |
| `Area_PlaceElevatorAndDoor` | 0:$90ED | placing it, unless opened |
| `LockedDoor_Ai` | 7:$D991 | everything above |
| `LockedDoor_Draw` | 7:$F13A | drawing |

## Verification

Scenario in the West Hyrule cave (area $2D; see the
[Myu](myu.md#verification) page for `$AT`), with the candle, a door placed
in slot 1 at X $2C8 and Link holding one key, walking into it:

- On frame 11 Link was pushed back and the door opened.  The spark came
  on frame 28, 17 frames later, and the door was gone.
- Screenshots on frames 10, 18, 26 and 34: the door whole, the bars
  closing in on the keyhole, the keyhole alone, nothing.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT,itemOwned=1 --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --spawn 1:02:2C8:60 --poke enemyHp+1=0xFF --poke keyCount=1 \
    --event Open=7:D9E7 --event Spark=7:D99E --event Push=7:D9CD --press 5:RIGHT:40 --frames 60
```
