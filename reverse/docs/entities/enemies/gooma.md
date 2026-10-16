# Gooma

The boss of palace 5: a giant who whirls a ball and chain and hurls it at
Link.  He is type $21, the same as [Helmethead](helmethead.md), with his
own code chosen by the region.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 1, 2, 5](../palaces.md) | $21 | Gooma (in palace 5) | 144 | 500 | 5 | - |

He is placed in room $29 of palaces 1, 2 and 5.  `Helmethead_Init` gives
type $21 his values when `currentRegion` isn't 0: 144 hit points,
attributes $CD (500 experience), damage $35 (class 5), hitbox entry $11.
The fight is described with the [palace bosses](../palaces.md#bosses).

## Behavior

`Gooma_Ai` (entered from `Helmethead_Ai`):

1. Guard 6, `Boss_UpdateIntro`, `Boss_UpdateCommon`.
2. **The ball** (his fifth sprite) is tested against Link's body only:
   no shield blocks it, and a hit knocks Link up and back.
3. 18 hit points per bar block.
4. **Pacing**, like Horsehead's (`Boss_Pace`): he turns toward Link every
   32 frames, walks 1/2 pixel per frame by `enemyAiState`, toward or away
   (bit 7).  Unlike Horsehead he walks whichever way he faces.  96 pixels
   or more away he comes toward Link; from 64 to 95, every 64 frames, 3
   times in 4.
5. **The swing**, under 64 pixels, 1 time in 8 each frame while idle
   (`Gooma_StartAttack`, `Gooma_Attack`):
   - pose 1, 47 frames: he whirls the ball round his head;
   - pose 2, 15 frames: he throws it out, his arm reaching forward, the
     ball swinging out to 56 pixels and back;
   - 32 frames of rest.

## Drawing

`Gooma_Draw`: three rows of three sprites from `GoomaPartTiles` (frames:
standing, two walking, whirling, throwing), mirrored facing right,
flashing while stunned.  The ball, OAM $40, circles his head
(`GoomaWhirlYOffsets`, `GoomaWhirlXOffsets`) or flies out with a chain
link between (`GoomaBallXOffsets`, `GoomaChainXOffsets`).

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Gooma_Ai` | 4:$B4AD | everything above |
| `Gooma_ChooseAttack`, `Gooma_ResetState` | 4:$B4FF, 4:$B4F6 | pacing |
| `Gooma_Attack`, `Gooma_StartAttack` | 4:$B53B, 4:$B557 | the swing |
| `Gooma_Draw`, `Gooma_DrawPart` | 4:$B5AE, 4:$B72D | drawing |

## Verification

Scenario in palace 1's first room (see [Bubble](bubble.md#verification),
with Link's life level raised), type $21 placed at X $1D0 with
`currentRegion` poked to 2 and his hit points and SRAM values as
`Helmethead_Init` sets them.  His tiles are palace 1's, so he is drawn
garbled.

- He swung on frames 17, 123, 337, 475 and 588: pose 1 for 47 frames,
  pose 2 for 15, then 32 frames of rest.
- The ball knocked Link back from X $1B2 to $190 by frame 50, and on to
  $130.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=3 --after 200 --clear $PFLAT --approach 1A0 --spawn 0:21:1D0:B0 \
    --poke currentRegion=2 --poke enemyHp=0x90 --poke sramEnemyHitbox+33=0x11 \
    --poke sramEnemyAttr+33=0xCD --poke sramEnemyDamage+33=0x35 --event Start=4:B557 --frames 600
```
