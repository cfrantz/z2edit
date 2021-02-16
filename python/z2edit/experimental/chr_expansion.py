import z2edit
from z2edit import Address
from z2edit.util import ObjectDict

INES_CHR_LEN = 8192

def hack(edit, asm, num_chr_banks):
    meta = edit.meta
    config = ObjectDict.from_json(z2edit.config[meta['config']]) 

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
    name = meta['config'] + '-chr_expansion'
    z2edit.config[name] = config.to_json()

    # Update the header to contain the new count of CHR banks.
    edit.write(Address.file(5), chr_layout.length // INES_CHR_LEN)

    # Now tell the project about the new config
    meta['extra'] = {'next_config': name}
    edit.meta = meta

