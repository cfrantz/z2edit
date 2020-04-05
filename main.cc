#include <cstdio>
#include <string>
#include <gflags/gflags.h>
#include <SDL2/SDL.h>

#include "app.h"
#include "util/config.h"
#include "zelda2_config.h"

DEFINE_string(config, "", "ROM info config file");
DEFINE_string(keybinds, "", "Alternate keybinds for the editor");
DEFINE_bool(dump_config, false, "Dump config to stdout and exit");
DEFINE_bool(move_from_keepout, true, "Move maps out of known keepout areas");
DEFINE_bool(reminder_dialogs, true, "Pop up dialogs for discarding changes");
DECLARE_int32(bank5_enemy_list_size);
DEFINE_bool(hackjam2020, true, "Turn on features for hackjam2020");

ConfigLoader<z2util::OverworldEditorKeybinds>* keybinds;

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
            for(const auto& c : s.code()) {
                if (map >= c.offset() && map < c.offset() + c.length()) {
                    m->set_code(c.code());
                }
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

            // If the door table is not null
            if (s.doors().address()) {
                m->mutable_doors()->set_bank(s.doors().bank());
                m->mutable_doors()->set_address(
                        s.doors().address() + 4*map);
            }

            *(m->mutable_chr()) = s.chr();
            *(m->mutable_palette()) = s.palette();
            *(m->mutable_palettes()) = s.palettes();
            for(int i=0; i<4; i++) {
                auto *obj = m->add_objtable();
                obj->set_bank(s.address().bank());
                obj->set_address(0x8500 + i*2);
            }
            if (map == 0 && s.area().find("background") == std::string::npos) {
                // Add a dummy "map" for initializing the object table editor
                auto* o = config->add_objtable();
                *o = *m;
                o->set_name(s.area());
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
    uint16_t b5_enemy_end;
    for(auto& ko: *config->mutable_misc()->mutable_allocator_keepout()) {
        if (ko.bank() == 5 && ko.address() == 0x88a0) {
            ko.set_length(FLAGS_bank5_enemy_list_size);
            b5_enemy_end = 0x88a0 + FLAGS_bank5_enemy_list_size;
        }
    }
    for(auto& sr: *config->mutable_misc()->mutable_static_regions()) {
        if (sr.bank() == 5 && sr.address() == 0x8a50) {
            sr.set_address(b5_enemy_end);
            sr.set_length(0x8b50 - b5_enemy_end);
        }
    }

    if (keybinds) {
        config->mutable_overworld_editor_keybind()->Clear();
        config->mutable_overworld_editor_keybind()->MergeFrom(
            keybinds->GetConfig().overworld_editor_keybind());
    }
}

const char kUsage[] =
R"ZZZ(<optional flags> [user-supplied-zelda2.nes] [script ...]

Description:
  A ROM file edtior for Zelda II The Adventure of Link.

Flags:
  --config <filename>        Use an alternate config file.
  --hidpi <n>                Set the scaling factor on hidpi displays (try 2.0)
  --emulator <prog>          Emulator to run for File | Emulate.
  --romtmp <filename>        Temporary filename for File | Emulate.
)ZZZ";

int main(int argc, char *argv[]) {
    gflags::SetUsageMessage(kUsage);
    gflags::ParseCommandLineFlags(&argc, &argv, true);

    // Load up alternative keybinds so we can apply them in PostProcess
    if (!FLAGS_keybinds.empty()) {
        keybinds = ConfigLoader<z2util::OverworldEditorKeybinds>::Get();
        keybinds->Load(FLAGS_keybinds);
    }

    auto* config = ConfigLoader<z2util::RomInfo>::Get();
    if (!FLAGS_config.empty()) {
        config->Load(FLAGS_config, PostProcess);
    } else {
        config->Parse(kZelda2Cfg, PostProcess);
    }
    if (FLAGS_dump_config) {
        puts(config->config().DebugString().c_str());
        exit(0);
    }

    z2util::Z2Edit app("Zelda 2 ROM Editor");
    app.Init();

    if (argc > 1) {
        app.Load(argv[1]);
        for(int i=2; i<argc; ++i) {
            app.Source(argv[i]);
        }
    }
    app.Run();
    return 0;
}
