#include <cstdio>
#include <string>
#include <gflags/gflags.h>

#include "app.h"
#include "util/config.h"

const char kUsage[] =
R"ZZZ(<optional flags> [user-supplied-zelda2.nes]

Description:
  A ROM file edtior for Zelda II The Adventure of Link.

Flags:
  --config <filename> Use an alternate config file.
  --hidpi <n>         Set the scaling factor on hidpi displays (try 2.0).
  --emulator <prog>   Emulator to run for File | Emulate.
  --romtmp <filename> Temporary filename for File | Emulate.
)ZZZ";

int main(int argc, char *argv[]) {
    gflags::SetUsageMessage(kUsage);
    gflags::ParseCommandLineFlags(&argc, &argv, true);

    z2util::App app("Zelda 2 ROM Editor");
    app.Init();
    app.Run();
    return 0;
}
