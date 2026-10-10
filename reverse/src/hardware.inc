; NES hardware and MMC1 mapper registers.
; Names follow the NESdev wiki.

.ifndef HARDWARE_INC
HARDWARE_INC = 1

; PPU
PPUCTRL         = $2000
PPUMASK         = $2001
PPUSTATUS       = $2002
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
JOY2            = $4017         ; read: controller 2
APU_FRAME       = $4017         ; write: APU frame counter

; MMC1.  Registers are loaded serially, one bit per write, LSB first.
; Any address within each 8K range selects the register.
MMC1_CONTROL    = $8000
MMC1_CHR0       = $A000
MMC1_CHR1       = $C000
MMC1_PRG        = $E000

.endif
