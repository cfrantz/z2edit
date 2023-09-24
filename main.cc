#include <cstdio>
#include <string>

#include "app.h"
#include "absl/flags/parse.h"
#include "absl/flags/usage.h"
#include "util/config.h"

const char kUsage[] =
R"ZZZ(<optional flags>

Description:
  An empty project.

Flags:
  --hidpi <n>         Set the scaling factor on hidpi displays (try 2.0).
)ZZZ";

int main(int argc, char *argv[]) {
    absl::SetProgramUsageMessage(kUsage);
    absl::ParseCommandLine(argc, argv);

    project::App app("Empty Project");
    app.Init();
    app.Run();
    return 0;
}
