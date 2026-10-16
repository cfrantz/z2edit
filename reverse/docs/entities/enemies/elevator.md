# Elevator

A lift that carries Link up and down between rooms.

## Where It Appears

Type $13 in every enemy group (`Elevator_Ai`, `Elevator_Draw`).  It is
never despawned, can't be hit, and is tested for Link's touch every frame
(`Enemy_CheckLinkContact`).

It isn't placed by enemy lists but by the area: the large extra object
$50 (`Area_PlaceElevator_BankN`) records its column in `elevatorPos`.
`Area_PlaceElevatorAndDoor` then calls `Elevator_Place`, which puts it
in enemy slot 0 at X = column × 16 + 4, Y $98, with no list entry.
When Link arrives by elevator (`enteredVertically`), he is put at X $70
on an elevator at the top of the screen (Y $10, if he came down) or the
bottom (Y $C0, if he came up).  Elevators are in the palaces and the
Great Palace.

## Behavior

`Elevator_Ai`.  While Link touches it (`objCollision` bit 4), he rides
it:

- `linkOnElevator` is set, and his jump and Y velocity are cleared.
- Up or Down (`linkUpDownHeld`) moves it 1.5 pixels per frame that way
  (`ElevatorSpeeds`), with a sound.  It doesn't move if Link is blocked
  that way: his head against a ceiling for Up (`objCollision` bit 3), his
  feet on a floor for Down (bit 2).
- Link is kept 8 pixels below its Y.

When its Y reaches $D8 or more (down past the bottom, or up past the top,
wrapping), Link goes to the connected room above or below
(`Mode_FindConnectedRoom`, gameMode $13, by `sramRoomConnections`).  He
arrives at X $70 with the scroll reset.  Then `Enemy_UpdateCommon`.

## Drawing

`Elevator_Draw`: two platforms, each three sprites (tile $AC, palette 1)
wide, one at its Y over Link's head and one 40 pixels lower, under his
feet.  The third tiles go blank ($F5) when their column is off screen.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Area_PlaceElevator_Bank1` | 1:$82BA | area object: the column |
| `Elevator_Place` | 0:$9150 | placing it |
| `Elevator_Ai` | 7:$D8C2 | riding |
| `ElevatorSpeeds` | 7:$D8BF | by Up/Down: 0, $18, $E8 |
| `Mode_FindConnectedRoom` | 7:$C644 | the next room |
| `Elevator_Draw` | 7:$EF47 | drawing |

## Verification

The no-glitch movie rides the palace elevators (`Elevator_Ai` and
`Mode_FindConnectedRoom` run there).  Scenario in the West Hyrule cave
(area $2D; see the [Myu](myu.md#verification) page for `$AT`), an
elevator placed in slot 0 at X $2A4, Y $78, under Link; Up held for 30
frames from frame 10, then Down for 30 from frame 45:

- It rose 1.5 pixels per frame, Link's Y always its Y + 8, and stopped
  when Up was let go.
- It came down at the same speed, and stopped at Y $78 when Link's feet
  met the floor, though Down was still held.
- Screenshot: one platform above Link's head, one under his feet.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT,itemOwned=1 --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --spawn 0:13:2A4:78 --press 10:UP:30 --press 45:DOWN:30 --frames 80
```
