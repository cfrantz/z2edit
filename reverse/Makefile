# Build the Zelda II disassembly and verify it against the reference ROM.
#
#   make           assemble and link build/zelda2.nes and build/zelda2.sym.json
#   make verify    check that build/zelda2.nes is identical to the reference
#   make status    update the generated section of STATUS.md
#   make clean     remove build products (the CDL logs in build/ are kept)

CC65_BIN ?= /opt/cc65/bin
CA65     := $(CC65_BIN)/ca65
LD65     := $(CC65_BIN)/ld65

ROM      ?= ../private/zelda2.nes
ROM_SHA256 := ad8c0fbcf092bf84b48e69fd3964eea4ed91bfe62abc352943d537979782680c

SRC      := src
BUILD    := build
MODULES  := header bank0 bank1 bank2 bank3 bank4 bank5 bank6 bank7
OBJS     := $(MODULES:%=$(BUILD)/%.o)
INCS     := $(wildcard $(SRC)/*.inc)
OUT      := $(BUILD)/zelda2.nes
SYMS     := $(BUILD)/zelda2.sym.json

.PHONY: all verify status clean

all: $(OUT) $(SYMS)

$(BUILD):
	mkdir -p $@

# CHR ROM is the last 128K of the reference ROM.
$(BUILD)/chr.bin: $(ROM) | $(BUILD)
	tail -c 131072 $< > $@

$(BUILD)/header.o: $(BUILD)/chr.bin

$(BUILD)/%.o: $(SRC)/%.s $(INCS) | $(BUILD)
	$(CA65) -g -I $(SRC) --bin-include-dir $(BUILD) -o $@ $<

$(OUT): $(OBJS) $(SRC)/zelda2.cfg
	$(LD65) -C $(SRC)/zelda2.cfg --dbgfile $(BUILD)/zelda2.dbg \
		-Ln $(BUILD)/zelda2.labels -m $(BUILD)/zelda2.map -o $@ $(OBJS)

$(SYMS): $(OUT) tools/symbols.py
	python3 tools/symbols.py $(BUILD)/zelda2.dbg $@

verify: $(OUT) $(SYMS)
	@echo "$(ROM_SHA256)  $(ROM)" | sha256sum --quiet -c - \
		|| (echo "Reference ROM $(ROM) is not the expected Zelda II (U) ROM"; exit 1)
	@if cmp -s $(OUT) $(ROM); then \
		echo "OK: $(OUT) is identical to the reference ROM"; \
	else \
		echo "FAIL: $(OUT) differs from the reference ROM"; \
		cmp -l $(OUT) $(ROM) | awk '{ printf "  file offset $$%05X: built %02X, expected %02X\n", $$1-1, strtonum("0"$$2), strtonum("0"$$3) }' | head -20; \
		exit 1; \
	fi

status: $(SYMS)
	python3 tools/status.py --cdl $(BUILD)/zelda2.cdl --symbols $(SYMS) --src $(SRC) --status STATUS.md

clean:
	rm -f $(OBJS) $(BUILD)/chr.bin $(OUT) $(BUILD)/zelda2.dbg $(BUILD)/zelda2.labels $(BUILD)/zelda2.map $(SYMS)
