#include <string>
#include <gflags/gflags.h>

#include "imapp.h"
#include "util/config.h"

DEFINE_string(config, "", "ROM info config file");

void PostProcess(z2util::RomInfo* config) {
    for(const auto& s : config->sideview()) {
        for(int map=0; map<s.length(); map++) {
            auto* m = config->add_map();
            char buf[64];
            sprintf(buf, "%s %02x", s.area().c_str(), map);
            //m->set_name(s.name());
            m->set_name(buf);
            m->set_type(s.type());
            m->mutable_address()->set_bank(s.address().bank());
            m->mutable_address()->set_address(s.address().address() + 2*map);
            *(m->mutable_chr()) = s.chr();
            *(m->mutable_palette()) = s.palette();
            for(int i=0; i<4; i++) {
                auto *obj = m->add_objtable();
                obj->set_bank(s.address().bank());
                obj->set_address(0x500 + i*2);
            }
        }
    }
}


int main(int argc, char *argv[]) {
    gflags::ParseCommandLineFlags(&argc, &argv, true);

    auto* config = ConfigLoader<z2util::RomInfo>::Get();
    if (!FLAGS_config.empty()) {
        config->Load(FLAGS_config, PostProcess);
    }

    ImApp app("ROM Explorer");
    app.Init();
    app.set_rominfo(config->config());

    if (argc > 1) {
        app.Load(argv[1]);
    }
    app.Run();
    return 0;
}



