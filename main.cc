#include <string>
#include <gflags/gflags.h>

#include "imapp.h"
#include "util/config.h"
#include "zelda2_config.h"

DEFINE_string(config, "", "ROM info config file");
DEFINE_bool(move_from_keepout, true, "Move maps out of known keepout areas");

void GetName(const z2util::RomInfo* config, int world,
             int overworld, int subworld, int id, std::string* name) {
    for(const auto& area : config->areas()) {
        if (world == area.world()
            && overworld == area.overworld()
            && subworld == area.subworld()) {
            const auto& it = area.info().find(id);
            if (it != area.info().end())
                *name = it->second.name();
        }
    }
}

void PostProcess(z2util::RomInfo* config) {
    char buf[128];
    for(const auto& s : config->sideview()) {
        for(int map=0; map<s.length(); map++) {
            auto* m = config->add_map();

            std::string name = "";
            GetName(config, s.world(), s.overworld(), s.subworld(), map, &name);
            m->set_area(s.area_offset() + map);
            if (name.empty()) {
                snprintf(buf, sizeof(buf), "%02d: %s %02d",
                         m->area(), s.area().c_str(), map);
            } else {
                snprintf(buf, sizeof(buf), "%02d: %s %02d - %s",
                         m->area(), s.area().c_str(), map, name.c_str());
            }
            m->set_name(buf);
            m->set_type(s.type());
            m->set_world(s.world());
            m->set_overworld(s.overworld());
            m->set_subworld(s.subworld());
            m->mutable_pointer()->set_bank(s.address().bank());
            m->mutable_pointer()->set_address(s.address().address() + 2*map);

            // If the connector table is not null
            if (s.connector().address()) {
                m->mutable_connector()->set_bank(s.connector().bank());
                m->mutable_connector()->set_address(
                        s.connector().address() + 4*map);
            }
            *(m->mutable_chr()) = s.chr();
            *(m->mutable_palette()) = s.palette();
            for(int i=0; i<4; i++) {
                auto *obj = m->add_objtable();
                obj->set_bank(s.address().bank());
                obj->set_address(0x500 + i*2);
            }
        }
    }
    for(auto& elist: *config->mutable_enemies()) {
        for(auto& e: *elist.mutable_info()) {
            snprintf(buf, sizeof(buf), "%02x: %s",
                     e.first, e.second.name().c_str());
            e.second.set_name(buf);
        }
    }
}

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

    auto* config = ConfigLoader<z2util::RomInfo>::Get();
    if (!FLAGS_config.empty()) {
        config->Load(FLAGS_config, PostProcess);
    } else {
        config->Parse(kZelda2Cfg, PostProcess);
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



