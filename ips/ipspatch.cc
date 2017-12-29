#include <cstdio>
#include <string>
#include <gflags/gflags.h>

#include "ips/ips.h"
#include "util/file.h"

DEFINE_bool(create, false, "Create an IPS patch");
DEFINE_bool(apply, false, "Apply an IPS patch");

const char kUsage[] =
R"ZZZ(<flags> [files...]

Description:
  A Simple IPS patch utility.

Usage:
  ipspatch -create [original] [modified] [patch-output-file]
  ipspatch -apply [original] [patch-file] [modified-output-file]
)ZZZ";

int main(int argc, char *argv[]) {
    gflags::SetUsageMessage(kUsage);
    gflags::ParseCommandLineFlags(&argc, &argv, true);
    std::string original, modified, patch;

    if (FLAGS_create && argc == 4) {
        File::GetContents(argv[1], &original);
        File::GetContents(argv[2], &modified);
        patch = ips::CreatePatch(original, modified);
        File::SetContents(argv[3], patch);
        printf("Wrote patch to %s\n", argv[3]);
    } else if (FLAGS_apply && argc == 4) {
        File::GetContents(argv[1], &original);
        File::GetContents(argv[2], &patch);
        auto mod = ips::ApplyPatch(original, patch);
        if (mod.ok()) {
            File::SetContents(argv[3], mod.ValueOrDie());
            printf("Applied patch and wrote new file %s\n", argv[3]);
        } else {
            printf("Error: %s\n", mod.status().ToString().c_str());
        }
    } else {
        printf("%s %s\n", argv[0], kUsage);
    }
    return 0;
}
