; NES hardware and MMC1 mapper registers.
; Names follow the NESdev wiki.

.ifndef HARDWARE_INC
HARDWARE_INC = 1

; PPU
PPUCTRL         = $2000
; PPUCTRL bits
PPUCTRL_NAMETABLE_MASK  = %00000011     ; base nametable
PPUCTRL_INC_32          = %00000100     ; VRAM address increment 32 (down)
PPUCTRL_SPRITES_1000    = %00001000     ; 8x8 sprite pattern table at $1000
PPUCTRL_BG_1000         = %00010000     ; background pattern table at $1000
PPUCTRL_SPRITES_8X16    = %00100000     ; 8x16 sprites
PPUCTRL_NMI             = %10000000     ; generate NMI at vblank

PPUMASK         = $2001
; PPUMASK bits
PPUMASK_GRAYSCALE       = %00000001
PPUMASK_BG_LEFT         = %00000010     ; show background in leftmost 8 pixels
PPUMASK_SPRITES_LEFT    = %00000100     ; show sprites in leftmost 8 pixels
PPUMASK_BG              = %00001000     ; show background
PPUMASK_SPRITES         = %00010000     ; show sprites
PPUMASK_EMPHASIS_MASK   = %11100000     ; color emphasis

PPUSTATUS       = $2002
; PPUSTATUS bits
PPUSTATUS_SPRITE_OVERFLOW = %00100000
PPUSTATUS_SPRITE0_HIT   = %01000000
PPUSTATUS_VBLANK        = %10000000

OAMADDR         = $2003
OAMDATA         = $2004
PPUSCROLL       = $2005
PPUADDR         = $2006
PPUDATA         = $2007

; APU
SQ1_VOL         = $4000
SQ1_SWEEP       = $4001
SQ1_LO          = $4002
SQ1_HI          = $4003
SQ2_VOL         = $4004
SQ2_SWEEP       = $4005
SQ2_LO          = $4006
SQ2_HI          = $4007
TRI_LINEAR      = $4008
TRI_UNUSED      = $4009
TRI_LO          = $400A
TRI_HI          = $400B
NOISE_VOL       = $400C
NOISE_UNUSED    = $400D
NOISE_LO        = $400E
NOISE_HI        = $400F
DMC_FREQ        = $4010
DMC_RAW         = $4011
DMC_START       = $4012
DMC_LEN         = $4013
OAMDMA          = $4014
SND_CHN         = $4015
JOY1            = $4016
; Controller button bits, as assembled by Joypad_Read (A is read first).
JOY_RIGHT               = %00000001
JOY_LEFT                = %00000010
JOY_DOWN                = %00000100
JOY_UP                  = %00001000
JOY_START               = %00010000
JOY_SELECT              = %00100000
JOY_B                   = %01000000
JOY_A                   = %10000000
JOY2            = $4017         ; read: controller 2
APU_FRAME       = $4017         ; write: APU frame counter

; MMC1.  Registers are loaded serially, one bit per write, LSB first.
; Any address within each 8K range selects the register.
MMC1_CONTROL    = $8000
MMC1_CHR0       = $A000
MMC1_CHR1       = $C000
MMC1_PRG        = $E000

; MMC1 control register values (5 bits).
MMC1_MIRROR_ONE_SCREEN_LO = %00000
MMC1_MIRROR_ONE_SCREEN_HI = %00001
MMC1_MIRROR_VERTICAL    = %00010
MMC1_MIRROR_HORIZONTAL  = %00011
MMC1_PRG_FIX_LAST       = %01100        ; switch $8000, last bank fixed at $C000
MMC1_CHR_8K             = %00000        ; switch 8K CHR at a time
MMC1_CHR_4K             = %10000        ; switch two separate 4K CHR banks
; Writing a value with this bit set resets the shift register.
MMC1_RESET              = %10000000

.endif
