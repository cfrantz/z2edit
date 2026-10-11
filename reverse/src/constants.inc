; Game-wide constants and enumerations.

.ifndef CONSTANTS_INC
CONSTANTS_INC = 1

; nmiMode values.  Nmi dispatches on bits 7 and 6.
NMI_MODE_SOUND_ONLY     = $40   ; bit 7 clear: Nmi_SoundOnly
NMI_MODE_BANK5          = $80   ; bit 7 set, bit 6 clear: Nmi_Bank5
NMI_MODE_GAME           = $C0   ; bits 7 and 6 set: game NMI

.endif
