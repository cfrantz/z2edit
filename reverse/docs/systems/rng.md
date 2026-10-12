# Random Numbers

`rng` is a 9-byte (72-bit) shift register.  The game NMI advances it once
per frame while the game runs: not while paused, nor during a dialog
except its pause states 3-6: all nine
bytes are shifted right one bit, from `rng` to `rng+8`, and the bit
shifted into the top of `rng` is bit 1 of `rng` XOR bit 1 of `rng+1`.

- Enemies read `rng+1,x` (and `rng+2,x`, `rng+3,x`) with their slot in
  `x`, so each of the six enemies sees a different byte of the same
  register.
- `SideView_Start` sets `rng` to $A5 each time an area starts; the other
  bytes carry on.  Power on clears the register (`Ram_ClearAll`), so the
  first reseed is what starts it.
- Dark Link's movement (`DarkLink_Move`) writes `rng+2` itself.

Uses include encounter strength (`Overworld_CheckEncounter`), enemy
behavior (when to jump, attack or turn), drops, and the chance of a
townsperson turning into a bat (`Dialog_MaybeTurnToBat`).
