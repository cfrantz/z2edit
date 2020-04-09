#include "imwidget/neschrview.h"
#include "nes/cartridge.h"
#include "nes/mapper.h"

int main(int argc, char **argv) {
  if (argc != 2) {
    fprintf(stderr, "Usage: %s <filename>\n", argv[0]);
    return 1;
  }

  char filename[7] = "00.bmp";

  Cartridge cart;
  cart.LoadFile(argv[1]);
  auto mapper = MapperRegistry::New(&cart, cart.mapper());

  for (int bank = 0; bank < 26; ++bank) {
    snprintf(filename, sizeof(filename), "%02x.bmp", bank);

    NesChrView chr(bank);
    chr.set_mapper(mapper);
    chr.RenderChr();
    chr.Export(filename);

    printf("Saved CHR bank %i to %s\n", bank, filename);
  }

  return 0;
}
