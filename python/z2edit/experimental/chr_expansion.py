import z2edit
from z2edit import Address
from z2edit.util import ObjectDict, Config

INES_CHR_LEN = 8192
INES_PRG_LEN = 16384

def hack(edit, num_prg_banks=0, num_chr_banks=0, next_config=None):
    meta = edit.meta
    config = Config.get(meta['config']) 

    if num_prg_banks and num_chr_banks:
        raise Exception('Cannot expand PRG and CHR in the same commit.')

    config = prg_expand(edit, config, num_prg_banks)
    config = chr_expand(edit, config, num_chr_banks)

    if next_config:
        name = next_config
    elif num_prg_banks:
        name = meta['config'] + '-prg_expansion'
    else:
        name = meta['config'] + '-chr_expansion'

    Config.put(name, config)
    # Now tell the project about the new config
    meta['extra'] = {'next_config': name, 'layout_changed': 'true'}
    edit.meta = meta


def prg_expand(edit, config, num_prg_banks):
    prg_layout = config.layout[1].Banked
    if prg_layout.name != 'prg':
        raise Exception('Expecteded segment named "prg"', prg_layout)
    chr_layout = config.layout[2].Banked
    if chr_layout.name != 'chr':
        raise Exception('Expecteded segment named "chr"', chr_layout)

    # The iNES header considers PRG banks as 8KiB segments in the PRG
    # section of the file.  Mappers may consider them differently.
    length = INES_PRG_LEN * num_prg_banks
    blank = b'\xFF' * length

    # The game needs to locate the reset vector in the last bank.
    # In vanilla with MMC1, this is bank 7.  We'll insert our banks
    # before bank 7, thus preserving the property that vanilla bank 7
    # is always the last bank.
    banks = prg_layout.length // prg_layout.banksize
    address = Address.prg(banks - 1, 0)
    edit.insert(address, blank)

    # Adjust the PRG layout in the config.
    prg_layout.length += length
    # Adjust the start of the CHR layout now that we've expanded PRG.
    chr_layout.offset += length
    # Update the header to contain the new count of PRG banks.
    edit.write(Address.file(4), prg_layout.length // INES_PRG_LEN)
    return config


def chr_expand(edit, config, num_chr_banks):
    chr_layout = config.layout[2].Banked
    if chr_layout.name != 'chr':
        raise Exception('Expecteded segment named "chr"', chr_layout)

    # The iNES header considers CHR banks as 8KiB segments in the CHR
    # section of the file.  Mappers may consider them differently.
    length = INES_CHR_LEN * num_chr_banks
    blank = b'\xFF' * length

    # The vanilla layout considers the CHR banks as MMC1 considers them:
    # 4KiB banks.  Since we want to add banks to the end, we use the last
    # bank number.
    address = Address.chr(chr_layout.length // chr_layout.banksize, 0)
    edit.insert(address, blank)

    # Adjust the CHR layout in the config.
    chr_layout.length += length
    # Update the header to contain the new count of CHR banks.
    edit.write(Address.file(5), chr_layout.length // INES_CHR_LEN)
    return config

