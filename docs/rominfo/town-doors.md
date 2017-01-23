## Understanding Town Doors

When link enters a door in a town, he is tranfered to a new room.
Since doors do not represent a left, down, up or right exit from an area,
there is a separate table which encodes where the doors go.

* Load the number of the current sideview area (`$561`), multiply by
4 (shift left) and add the current screen number that link is on (`$3b`).
Transfer the result to the `Y` register.

```
cfff: ad6105        LDA $0561
d002: 0a            ASL A
d003: 0a            ASL A
d004: 653b          ADC $3b
d006: a8            TAY
```

* Load the destination map/room number from the table at `$8817`.  Mask off the
two least significant bits (the room number) and shift right twice for
the map number.  Store the result in the current sideview area (`$561`).

Note: The `AND` mask seems redundant considering the `LSR` instruction will
just discard those bits anyway.

```
d007: b91788        LDA $8817,Y
d00a: 29fc          AND #$fc
d00c: 4a            LSR A
d00d: 4a            LSR A
d00e: 8d6105        STA $0561
```

* Re-load the destination map/room number from the table at `$8817`.
Mask off the destination map, keeping the room number.  Store the value
in the Start-Map-Page variable (`$75c`).  Mask again, and store in
Start-Side-of-Screen variable (`$701`).

```
d011: b91788        LDA $8817,Y
d014: 2903          AND #$03
d016: 8d5c07        STA $075c
d019: 2901          AND #$01
d01b: 8d0107        STA $0701

```

## TL;DR

For towns, there is a table at `$8817`.  Every town sideview area has a 4-byte
entry.  Each byte describes where the door on that screen should take
the player.

## Trivia

The fireplace in the hidden town of Kasuto is just a door and its destination
is in the table.
