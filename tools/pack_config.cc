#include <string>
#include <gflags/gflags.h>

#include "proto/rominfo.pb.h"
#include "util/config.h"

DEFINE_string(config, "", "ROM info config file");
DEFINE_string(symbol, "kConfigText", "Symbol name of config");
DEFINE_string(delimeter, "ZCFGZ", "C++ raw string delimiter");

int main(int argc, char *argv[]) {
    gflags::ParseCommandLineFlags(&argc, &argv, true);

    auto* loader = ConfigLoader<z2util::RomInfo>::Get();
    if (FLAGS_config.empty()) {
        LOG(FATAL, "Need a config file.");
    }

    loader->Load(FLAGS_config);

    z2util::RomInfo* config = loader->MutableConfig();
    config->mutable_load()->Clear();
    std::string data = config->DebugString();
    
    printf("const char %s[] = R\"%s(%s)%s\";\n",
           FLAGS_symbol.c_str(),
           FLAGS_delimeter.c_str(),
           data.c_str(),
           FLAGS_delimeter.c_str());
    return 0;
}
