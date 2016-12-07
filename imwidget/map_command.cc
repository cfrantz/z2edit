#include <cstdlib>

#include "imwidget/map_command.h"
#include "imgui.h"
#include "util/config.h"

namespace z2util {

int MapCommand::newid() {
    static int id = 1;
    return id++;
}

const DecompressInfo* MapCommand::info_[NR_AREAS][NR_SETS][16];
const char* MapCommand::object_names_[NR_AREAS][NR_SETS][16];
void MapCommand::Init() {
    static bool done;
    static char names[NR_AREAS][NR_SETS][16][64];
    if (done)
        return;
    memset(info_, 0, sizeof(info_));
    memset(object_names_, 0, sizeof(object_names_));
    for(int i=0; i<NR_AREAS; i++) {
        for(int j=0; j<NR_SETS; j++) {
            for(int k=0; k<16; k++) {
                if (j==0 && k==15) {
                    sprintf(names[i][j][k], "%02x: collectable",
                            j==0 ? k : k<<4);
                } else {
                    sprintf(names[i][j][k], "%02x: ???", 
                            j==0 ? k : k<<4);
                }
                object_names_[i][j][k] = names[i][j][k];
            }
        }
    }
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    for(const auto& d : ri.decompress()) {
        int id = d.id();
        int val = id;
        if (id & 0xF0) {
            id >>= 4;
        }
        snprintf(names[d.area()][d.type()][id], 63,
                 "%02x: %s", val, d.comment().c_str());
        info_[d.area()][d.type()][id] = &d;
    }
    done = true;
}

MapCommand::MapCommand(const MapHolder& holder, uint8_t position,
                       uint8_t object, uint8_t extra)
  : id_(newid()),
    holder_(holder),
    position_(position),
    object_(object),
    extra_(extra)
{
    data_.x = position_ & 0xf;
    data_.y = position_ >> 4;
    Init();
}

bool MapCommand::Draw() {
    bool changed = false;
    const char *yfmt;
    if (data_.y == 13) {
        yfmt = "%.0f (new floor)";
    } else if (data_.y == 14) {
        yfmt = "%.0f (x skip)";
    } else if (data_.y == 15) {
        yfmt = "%.0f (extra obj)";
    } else {
        yfmt = "%.0f";
    }

    ImGui::PushItemWidth(100);
    sprintf(name_.ypos, "ypos##%d", id_);
    changed |= ImGui::DragInt(name_.ypos, &data_.y, 1, 0, 15, yfmt); 


    ImGui::SameLine();
    sprintf(name_.xpos, "xpos##%d", id_);
    changed |= ImGui::DragInt(name_.xpos, &data_.x, 1, 0, 15);

    if (data_.y == 13 || data_.y == 14) {
        sprintf(name_.object, "param##%d", id_);
        sprintf(obuf_, "%02x", object_);
        ImGui::SameLine();
        changed |= ImGui::InputText(name_.object, obuf_, sizeof(obuf_),
                                    ImGuiInputTextFlags_CharsHexadecimal |
                                    ImGuiInputTextFlags_EnterReturnsTrue);
        object_ = strtoul(obuf_, 0, 16);
    } else {
        int n = 0;
        int large_start = 0;
        int extra_start = 0;
        int type = holder_.map().type();
        int oindex = 1 + !!(holder_.flags() & 0x80);

        for(int i=0; i<NR_SETS; i++) {
            if (i==1) large_start = n;
            if (i==3) extra_start = n;
            if ((i==1 || i==2) && i != oindex)
                continue;
            for(int j=0; j<16; j++) {
                name_.sel[n++] = object_names_[type][i][j];
            }
        }

        if (data_.y == 15) {
            oindex = 3;
            data_.object = extra_start + (object_ >> 4);
            data_.param = object_ & 0x0F;
        } else if ((object_ & 0xF0) == 0) {
            oindex = 0;
            data_.object = object_ & 0x0F;
            data_.param = 0;
        } else {
            data_.object = large_start + (object_ >> 4);
            data_.param = object_ & 0x0F;
        }

        sprintf(name_.object, "id##%d", id_);
        ImGui::PushItemWidth(200);
        ImGui::SameLine();
        changed |= ImGui::Combo(name_.object, &data_.object, name_.sel, n);
        ImGui::PopItemWidth();
        if (changed) {
            printf("changed to %d (%02x) %s\n", data_.object, data_.object,
                    name_.sel[data_.object]);
        }
                               
        // All of the object names start with their ID in hex, so we just
        // parse the value out of the name.
        object_ = strtoul(name_.sel[data_.object], 0, 16);

        // If the index from the combobox is >= the "extra" items, then
        // set y to the magic 'extra items' value.
        if (data_.object >= extra_start) {
            data_.y = 15;
        } else if (data_.object < extra_start && data_.y == 15) {
            data_.y = 12;
        }
        if (oindex) {
            ImGui::SameLine();
            sprintf(name_.param, "param##%d", id_);
            changed |= ImGui::DragInt(name_.param, &data_.param, 1, 0, 15);
            object_ |= data_.param;
        } else if (object_ == 15) {
            ImGui::SameLine();
            sprintf(name_.param, "collectable##%d", id_);
            sprintf(ebuf_, "%02x", extra_);
            ImGui::SameLine();
            changed |= ImGui::InputText(name_.param, ebuf_, sizeof(ebuf_),
                                        ImGuiInputTextFlags_CharsHexadecimal |
                                        ImGuiInputTextFlags_EnterReturnsTrue);
            extra_ = strtoul(ebuf_, 0, 16);
        }
    }

    ImGui::PopItemWidth();
    return changed;
}

std::vector<uint8_t> MapCommand::Command() {
    position_ = (data_.y << 4) | data_.x;
    if (data_.y < 13 && object_ == 15) {
        return {position_, object_, extra_};
    }
    return {position_, object_};
}


MapHolder::MapHolder() {}

const char* ground_names[] = {
    "%.0f (cave)",
    "%.0f (forest)",
    "%.0f (lava)",
    "%.0f (swamp)",
    "%.0f (sand)",
    "%.0f (volcano)",
    "%.0f (north castle)",
    "%.0f (outside)",
};

void MapHolder::Unpack() {
    data_.objset = !!(flags_ & 0x80);
    data_.width = 1 + ((flags_ >> 5) & 3);
    data_.grass = !!(flags_ & 0x10);
    data_.bushes = !!(flags_ & 0x08);
    data_.ceiling = !(ground_ & 0x80);
    data_.ground = (ground_ >> 4) & 7;
    data_.floor = ground_ & 0xf;
    data_.spal = (back_ >> 6) & 3;
    data_.bpal = (back_ >> 3) & 7;
    data_.bmap = back_ & 7;
}

void MapHolder::Pack() {
    flags_ = (data_.objset << 7) |
             ((data_.width-1) << 5) |
             (int(data_.grass) << 4) |
             (int(data_.bushes) << 3);
    ground_ = (int(!data_.ceiling) << 7) | (data_.ground << 4) | (data_.floor);
    back_ = (data_.spal << 6) | (data_.bpal << 3) | data_.bmap;
}

bool MapHolder::Draw() {
    bool changed = false;
    Unpack();

    ImGui::PushItemWidth(100);
    ImGui::Text("Flags:");
    changed |= ImGui::DragInt("Object Set", &data_.objset, 1, 0, 1);
    ImGui::SameLine(); changed |= ImGui::DragInt("Width", &data_.width, 1, 1, 4);
    ImGui::SameLine(); changed |= ImGui::Checkbox("Grass", &data_.grass);
    ImGui::SameLine(); changed |= ImGui::Checkbox("Bushes", &data_.bushes);

    ImGui::Text("Ground:");
    changed |= ImGui::Checkbox("Ceiling", &data_.ceiling);
    ImGui::PushItemWidth(140);
    ImGui::SameLine(); changed |= ImGui::DragInt("Tiles", &data_.ground, 1, 0, 7,
                                      ground_names[data_.ground]);
    ImGui::PopItemWidth();
    ImGui::SameLine(); changed |= ImGui::DragInt("Floor", &data_.floor, 1, 0, 15);

    ImGui::Text("Background:");
    changed |= ImGui::DragInt("Spr Palette", &data_.spal, 1, 0, 3);
    ImGui::SameLine(); changed |= ImGui::DragInt("BG Palette", &data_.bpal, 1, 0, 7);
    ImGui::SameLine(); changed |= ImGui::DragInt("BG Map", &data_.bmap, 1, 0, 7);

    ImGui::Text("Command List:");
    ImGui::BeginChild("Command List",
                      ImVec2(ImGui::GetWindowContentRegionWidth() * 0.67f, 300),
                      false, ImGuiWindowFlags_HorizontalScrollbar);
    for(auto& cmd : command_) {
        changed |= cmd.Draw();
    }
    ImGui::EndChild();
    ImGui::PopItemWidth();
    Pack();
    return changed;
}

void MapHolder::Parse(const z2util::Map& map) {
    map_ = map;
    // FIXME: for side view maps, the map address is the address of a pointer
    // to the real address.  Read it and set the real address.
    Address address = map.address();
    address.set_address(ReadWord(map.address(), 0));

    length_ = Read(address, 0);
    flags_ = Read(address, 1);
    ground_ = Read(address, 2);
    back_ = Read(address, 3);

    command_.clear();
    for(int i=4; i<length_; i+=2) {
        uint8_t pos = Read(address, i);
        uint8_t obj = Read(address, i+1);
        uint8_t extra = 0;

        int y = pos >> 4;
        if (y < 13 && obj == 15) {
            i++;
            extra = Read(address, i+1);
        }
        command_.emplace_back(*this, pos, obj, extra);
    }
}

std::vector<uint8_t> MapHolder::MapData() {
    std::vector<uint8_t> map = {length_, flags_, ground_, back_};
    for(auto& cmd : command_) {
        auto bytes = cmd.Command();
        map.insert(map.end(), bytes.begin(), bytes.end());
    }
    map[0] = map.size();
    return map;
}

}  // namespace z2util
