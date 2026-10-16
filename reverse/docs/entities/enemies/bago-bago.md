# Bago Bago

A fish that leaps out of the water beside bridges and spits rocks.  An
invisible **generator** launches one every 128 frames.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $0F | Bago Bago Generator | 3 | 3 | 1 | - |
| [West Hyrule](../west-hyrule.md) | $10 | Bago Bago | 3 | 3 | 1 | 1 |
| [East Hyrule](../east-hyrule.md) | $0F | Bago Bago Generator | 8 | 0 | 2 | - |
| [East Hyrule](../east-hyrule.md) | $10 | Bago Bago | 8 | 5 | 2 | 1 |
| [Palaces 1, 2, 5](../palaces.md) | $0F | Bago Bago Generator | 8 | 0 | 0 | - |
| [Palaces 1, 2, 5](../palaces.md) | $10 | Bago Bago | 8 | 5 | 2 | 1 |
| [Great Palace](../great-palace.md) | $0F | Fire Bago Bago Generator | 8 | 20 | 3 | - |
| [Great Palace](../great-palace.md) | $10 | Fire Bago Bago | 8 | 20 | 3 | 1 |

The area lists place only generators (bridges and water in both Hyrules
and Death Mountain, a palace 1/2/5 room, two Great Palace rooms).  Every
Bago Bago is launched by one, with 3 hit points whatever the table says.
Touching a Bago Bago drains experience (attribute bit 4).  The generator
is invisible, can't be hurt, and has the generators' spawn routine
(`Generator_Init_BankN`), which removes any other generator of the same
type.

## Behavior

### The Generator

`BagoBagoGenerator_Ai` runs every frame.  Every 128 frames (`frameCounter`
bits 6-0 all 0), if one of the six enemy slots is free, it puts a Bago
Bago (type $10) in it:

- below the screen, at Y $D0, at its left or right side at random (X =
  `scrollX` + $10 or + $F0, `BagoBagoSpawnX`);
- leaping up at 4 pixels per frame, and moving toward the middle of the
  screen at 2 pixels per frame (`BagoBagoSpawnSpeeds`), facing that way;
- with 3 hit points, and no list entry (`enemySramIndex` $D0).

Its other state (`enemyAiState`, timers, collision bits) is whatever the
slot's last enemy left; a nonzero `enemyAiState` costs it its first frame
of gravity.

### The Bago Bago

`BagoBago_Ai`, after `Enemy_SkipAiIfStunned`:

1. **Off screen.**  Once both of its sprite columns are off the side of
   the screen (`drawOffscreen+1` bits 3-2), it is removed.  This is how
   Bago Bagos leave, since they never turn.
2. **In the air** (`enemyAiState` 0).  Gravity of 1/16 pixel per frame
   per frame, so the leap rises 128 pixels in 64 frames (to Y $50) and
   then falls, while it keeps moving 2 pixels per frame across.  It tests
   for a floor only while coming down, so it leaps up through bridges and
   floors.
3. **Landing.**  On a floor while coming down, it lands (`Enemy_Land`),
   turns toward Link and walks his way at 1 pixel per frame
   (`BagoBagoSpeeds`), and never turns again.  If it walks off a ledge it
   falls again (`enemyAiState` back to 0).  It doesn't test for walls, so
   it walks through them.
4. **Spitting.**  Whenever its timer (`timerFast+TIMER_ENEMY`) has run out
   and it is 0 to 16 pixels below Link's Y (`objY`), it reloads the
   timer with 112 frames.  Then, half the time (`rng+1,x` bit 7 clear),
   it spits a rock if it faces Link (`Enemy_ShootIfFacingLink`, projectile
   type 3, as the [Octorok's](octorok.md#spitting)).  In the air it faces
   the way it was launched.
5. `Enemy_UpdateWithBg` (probes, drawing, hit tests) and
   `Enemy_HurtLinkOnContact`.

A Bago Bago from the left side typically lands on the bridge about 110
frames after launch, some 220 pixels across, and walks on toward Link,
past him and off the screen.

## Drawing

`BagoBago_Draw`: a pair of sprites facing its `objFacing`, alternating
every 8 frames between tiles $C0, $C2 and $C0, $C4 (`SpritePairTiles`
entries $0C and $0E).

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `BagoBagoGenerator_Ai` | 7:$D78F | the generator |
| `BagoBagoSpawnX` | 7:$D78B | $10, $F0 from the screen's left edge |
| `BagoBagoSpawnSpeeds` | 7:$D78D | $20, $E0 |
| `BagoBago_Ai` | 7:$D7E1 | the fish |
| `BagoBagoSpeeds` | 7:$D7DF | walking: $10 right, $F0 left |
| `BagoBago_Draw` | 7:$F0B9 | drawing |

## Verification

Scenario in West Hyrule's area $01 (location $13: a bridge over water
ending at a cliff, Link on the cliff at Y $60), with the area's own
generators, 900 frames:

- A Bago Bago was launched at every `frameCounter` of $00 or $80 when a
  slot was free.
- It came up to Link's height (Y $71) after 31 frames, as the gravity
  predicts (93 pixels), spat at once (the timer was 0), and next tried
  112 frames later.
- Those from the right landed on the bridge (Y $90) 109 frames after
  launch, 218 pixels across.  One from the left landed on Link's cliff.
- All walked toward Link without turning, through the cliff face, and
  were removed at X $400, off the screen.

```
AT=areaPrgBank=1,currentRegion=0,currentWorld=0,townCode=0,palaceCode=0,areaLocation=0x13,areaId=0x01,areaStartPage=0,areaEntrySide=0,prevRegion=0
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x01 --after 60 \
    --log 0,1,2,3,4,5 --event Spawn=7:D7A1 --event Land=7:D803 --event Aim=7:D825 \
    --event Shoot=7:D82F --event Remove=7:D7EC --frames 900
```
