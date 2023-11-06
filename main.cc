#include <cstdio>
#include <string>

#include "app.h"
#include "absl/flags/parse.h"
#include "absl/flags/usage.h"
#include "absl/log/log.h"
#include "util/config.h"
#include "proto/gui_extension.pb.h"

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
    gui::proto::Foo foo;

    project::App app("Empty Project");
    app.Init();
    app.SetMessage(&foo);
    app.Run();
    return 0;
}
