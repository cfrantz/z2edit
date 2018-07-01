#include <cstdlib>
#include <algorithm>
#include "imwidget/map_command.h"

#include "imwidget/error_dialog.h"
#include "imwidget/imapp.h"
#include "imwidget/imutil.h"
#include "imwidget/overworld_encounters.h"
#include "imwidget/simplemap.h"
#include "imgui.h"
#include "nes/enemylist.h"
#include "util/config.h"
#include "util/strutil.h"

#include "IconsFontAwesome.h"

namespace z2util {

const DecompressInfo* MapCommand::info_[NR_AREAS][NR_SETS][16];
const char* MapCommand::object_names_[NR_AREAS][NR_SETS][16];
const char* MapCommand::collectable_names_[MAX_COLLECTABLE];
void MapCommand::Init() {
    static bool done;
    static char names[NR_AREAS][NR_SETS][16][64];
    static char collectable[MAX_COLLECTABLE][32];
    if (done)
        return;
    memset(info_, 0, sizeof(info_));
    memset(object_names_, 0, sizeof(object_names_));
    for(int i=0; i<NR_AREAS; i++) {
        for(int j=0; j<NR_SETS; j++) {
            for(int k=0; k<16; k++) {
                if (j==0 && k==15) {
                    sprintf(names[i][j][k], "%02x: collectable",
                            (j==0 || j==3)? k : k<<4);
                } else {
                    sprintf(names[i][j][k], "%02x: ???",
                            (j==0 || j==3)? k : k<<4);
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
    for(int i=0; i<MAX_COLLECTABLE; i++) {
        const auto& c = ri.items().info().find(i);
        if (c == ri.items().info().end()) {
            snprintf(collectable[i], 31, "%02x: ???", i);
        } else {
            snprintf(collectable[i], 31, "%02x: %s", i, c->second.name().c_str());
        }
        collectable_names_[i] = collectable[i];
    }
    done = true;
}

MapCommand::MapCommand(const MapHolder* holder, uint8_t position,
                       uint8_t object, uint8_t extra)
  : id_(UniqueID()),
    holder_(holder),
    show_origin_(true),
    position_(position),
    object_(object),
    extra_(extra),
    summary_(nullptr)
{
    data_.x = position_ & 0xf;
    data_.y = position_ >> 4;
    Init();
}

MapCommand::MapCommand(const MapHolder* holder, int x0, uint8_t position,
                       uint8_t object, uint8_t extra)
  : MapCommand(holder, position, object, extra)
{
    if (data_.y == 14) {
        data_.absx = data_.x * 16;
    } else {
        data_.absx = data_.x + x0;
    }
}

MapCommand::MapCommand(const MapCommand& other)
  : id_(UniqueID()),
    holder_(other.holder_),
    show_origin_(other.show_origin_),
    position_(other.position_),
    object_(other.object_),
    extra_(other.extra_),
    data_(other.data_),
    summary_(nullptr) {}

MapCommand MapCommand::Copy() {
    MapCommand copy = *this;
    copy.id_ = UniqueID();
    return copy;
}

bool MapCommand::Draw(bool abscoord, bool popup) {
    bool changed = false;
    const char *xpos = "x position";
    const char *ypos = "y position";
    if (data_.y == 13) {
        summary_ = ypos = "new floor ";
    } else if (data_.y == 14) {
        summary_ = ypos = "x skip    ";
        xpos = "to screen#";
    } else if (data_.y == 15) {
        ypos = "extra obj ";
    }

    ImGui::PushID(id_);
    ImGui::PushItemWidth(100);
    changed |= ImGui::InputInt(ypos, &data_.y);
    Clamp(&data_.y, 0, 15);

    if (!popup) ImGui::SameLine();
    if (data_.y != 14 && abscoord) {
        changed |= ImGui::InputInt(xpos, &data_.absx);
        Clamp(&data_.absx, 0, 63);
    } else {
        changed |= ImGui::InputInt(xpos, &data_.x);
        Clamp(&data_.x, 0, 15);
    }

    if (data_.y == 13 || data_.y == 14) {
        sprintf(obuf_, "%02x", object_);
        if (!popup) ImGui::SameLine();
        changed |= ImGui::InputText("param", obuf_, sizeof(obuf_),
                                    ImGuiInputTextFlags_CharsHexadecimal |
                                    ImGuiInputTextFlags_EnterReturnsTrue);
        object_ = strtoul(obuf_, 0, 16);
    } else {
        int n = 0;
        int large_start = 0;
        int extra_start = 0;
        int extra_small_start = 0;
        int type = holder_->map().type();
        int oindex = 1 + !!(holder_->flags() & 0x80);

        for(int i=0; i<NR_SETS; i++) {
            if (i==1) large_start = n;
            if (i==3) extra_small_start = n;
            if (i==4) extra_start = n;
            if ((i==1 || i==2) && i != oindex)
                continue;
            for(int j=0; j<16; j++) {
                names_[n++] = object_names_[type][i][j];
            }
        }

        if (data_.y == 15) {
            // y==15 means objects in the "extra" object set, which is not
            // to be confused with the extra_ data element, meant to hold
            // the ID of collectable items.
            if ((object_ & 0xF0) == 0) {
                oindex = 3;
                data_.object = extra_small_start + object_;
                data_.param = 0;
            } else {
                oindex = 4;
                data_.object = extra_start + (object_ >> 4);
                data_.param = object_ & 0x0F;
            }
        } else if ((object_ & 0xF0) == 0) {
            oindex = 0;
            data_.object = object_ & 0x0F;
            data_.param = 0;
        } else {
            data_.object = large_start + (object_ >> 4);
            data_.param = object_ & 0x0F;
        }
        data_.extra = extra_;

        ImGui::PushItemWidth(200);
        if (!popup) ImGui::SameLine();
        changed |= ImGui::Combo("id", &data_.object, names_, n);
        summary_ = names_[data_.object];
        ImGui::PopItemWidth();
        if (changed) {
            printf("changed to %d (%02x) %s\n", data_.object, data_.object,
                    names_[data_.object]);
        }

        // All of the object names start with their ID in hex, so we just
        // parse the value out of the name.
        object_ = strtoul(names_[data_.object], 0, 16);

        // If the index from the combobox is >= the "extra" items, then
        // set y to the magic 'extra items' value.
        if (data_.object >= extra_small_start) {
            data_.y = 15;
        } else if (data_.object < extra_small_start && data_.y == 15) {
            data_.y = 12;
        }
        if (oindex !=0 && oindex != 3) {
            if (!popup) ImGui::SameLine();
            changed |= ImGui::InputInt("param", &data_.param);
            Clamp(&data_.param, 0, 15);
            object_ |= data_.param;
        } else if (object_ == 15) {
            if (!popup) ImGui::SameLine();
            ImGui::PushItemWidth(200);
            changed |= ImGui::Combo("##collectable", &data_.extra,
                                    collectable_names_, MAX_COLLECTABLE);
            summary_ = collectable_names_[data_.extra];
            ImGui::PopItemWidth();
            extra_ = data_.extra;
        }
    }
    ImGui::PopItemWidth();
    ImGui::PopID();

    return changed;
}

MapCommand::DrawResult MapCommand::DrawPopup(float scale) {
    DrawResult result = DR_NONE;
    ImVec2 pos = ImGui::GetCursorPos();
    ImVec2 abs = ImGui::GetCursorScreenPos();
    uint32_t color = 0xFFFFFFFF;
    auto* draw = ImGui::GetWindowDrawList();
    float size = 16.0 * scale;
    float yp = data_.y;
    if (yp > 13) yp = 13;

    if (show_origin_) {
        ImVec2 a = ImVec2(abs.x + data_.absx * size, abs.y + yp * size);
        ImVec2 b = ImVec2(a.x + size, a.y + size);
        draw->AddRect(a, b, color, 0, ~0, 2.0f);
        if (data_.y == 13) {
            // New floor
            for(int i=0; i<16; i+=4) {
                ImVec2 a = ImVec2(abs.x + data_.absx * size + (i + 4) * scale,
                                  abs.y + yp * size + 8 * scale);
                ImVec2 b = ImVec2(abs.x + data_.absx * size + (i + 0) * scale,
                                  abs.y + yp * size + 14 * scale);
                draw->AddLine(a, b, color);
            }
        } else if (data_.y == 14) {
            // X skip
            ImVec2 a = ImVec2(abs.x + data_.absx * size + 4 * scale,
                              abs.y + yp * size + 4 * scale);
            ImVec2 b = ImVec2(abs.x + data_.absx * size + 4 * scale,
                              abs.y + yp * size + 12 * scale);
            ImVec2 c = ImVec2(abs.x + data_.absx * size + 12 * scale,
                              abs.y + yp * size + 8 * scale);
            draw->AddTriangleFilled(a, b, c, color);
        }
    }

    ImGui::PushID(-id_);
    ImGui::SetCursorPos(ImVec2(pos.x + data_.absx * size, pos.y + yp * size));
    ImGui::InvisibleButton("button", ImVec2(size, size));
    if (ImGui::IsItemHovered()) {
        ImGui::SetTooltip("%s", summary_ ? summary_ : "unknown");
        int delta = int(ImGui::GetIO().MouseWheel);
        if (delta && (object_ & 0xF0) != 0) {
            data_.param = object_ & 0x0F;
            data_.param = Clamp(data_.param - delta, 0, 15);
            object_ = (object_ & 0xF0) | data_.param;
            result = DR_CHANGED;
        }
    }
    if (ImGui::IsItemActive()) {
        if (ImGui::IsMouseDragging()) {
            int x = int((ImGui::GetIO().MousePos.x - abs.x) / size);
            int y = int((ImGui::GetIO().MousePos.y - abs.y) / size);
            x = Clamp(x, 0, 64);
            y = Clamp(y, 0, 12);
            if (x != data_.absx) {
                data_.absx = x;
                result = DR_CHANGED;
            }
            if (yp < 13 && y != yp) {
                data_.y = y;
                result = DR_CHANGED;
            }
        }
    }
    if (ImGui::BeginPopupContextItem("Properties")) {
        if (Draw(true, true)) {
            result = DR_CHANGED;
        }
        if (ImGui::Button("Copy")) {
            result = DR_COPY;
        }
        ImGui::SameLine();
        if (ImGui::Button("Delete")) {
            result = DR_DELETE;
        }
        ImGui::EndPopup();
    }
    ImGui::SetCursorPos(pos);
    ImGui::PopID();
    return result;
}

std::vector<uint8_t> MapCommand::Command() {
    position_ = (data_.y << 4) | data_.x;
    if (data_.y < 13 && object_ == 15) {
        return {position_, object_, extra_};
    }
    return {position_, object_};
}


MapHolder::MapHolder(Mapper* m) : mapper_(m) {}
MapHolder::MapHolder() : MapHolder(nullptr) {}

const char* ground_names[] = {
    "TileSet (cave)",
    "TileSet (forest)",
    "TileSet (lava)",
    "TileSet (swamp)",
    "TileSet (sand)",
    "TileSet (volcano)",
    "TileSet (north castle)",
    "TileSet (outside)",
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

MapHolder::DrawResult MapHolder::Draw() {
    DrawResult result = DR_NONE;
    char abuf[8];
    bool changed = false, achanged=false;
    Unpack();

    Address addr = mapper_->ReadAddr(map_.pointer(), 0);
    ImGui::Text("Map pointer at bank=0x%x address=0x%04x",
                map_.pointer().bank(), map_.pointer().address());

    ImGui::AlignFirstTextHeightToWidgets();
    ImGui::Text("Map address at bank=0x%x address=",
                addr.bank());

    ImGui::PushItemWidth(100);
    sprintf(abuf, "%04x", map_addr_);
    ImGui::SameLine();
    achanged = ImGui::InputText("##addr", abuf, 5,
                                ImGuiInputTextFlags_CharsHexadecimal |
                                ImGuiInputTextFlags_EnterReturnsTrue);
    if (achanged) {
        map_addr_ = strtoul(abuf, 0, 16);
        Parse(map_, map_addr_);
        addr_changed_ |= achanged;
        return DR_PALETTE_CHANGED;
    }

    ImGui::Text("Length = %d bytes.", length_);

    ImGui::Text("Flags:");
    changed |= ImGui::InputInt("Object Set", &data_.objset);
    Clamp(&data_.objset, 0, 1);

    ImGui::SameLine();
    changed |= ImGui::InputInt("Width", &data_.width);
    Clamp(&data_.width, 1, 4);

    ImGui::SameLine();
    changed |= ImGui::Checkbox("Grass", &data_.grass);

    ImGui::SameLine();
    changed |= ImGui::Checkbox("Bushes", &data_.bushes);

    ImGui::SameLine();
    changed |= ImGui::Checkbox("Cursor Moves Left", &cursor_moves_left_);

    ImGui::Text("Ground:");
    changed |= ImGui::Checkbox("Ceiling", &data_.ceiling);

    ImGui::SameLine();
    changed |= ImGui::InputInt(ground_names[data_.ground], &data_.ground);
    Clamp(&data_.ground, 0, 7);

    ImGui::SameLine();
    changed |= ImGui::InputInt("Floor", &data_.floor);
    Clamp(&data_.floor, 0, 15);

    ImGui::Text("Background:");
    if (ImGui::InputInt("Spr Palette", &data_.spal)) {
        changed |= true;
        result = DR_PALETTE_CHANGED;
    }
    Clamp(&data_.spal, 0, 3);

    ImGui::SameLine();
    if (ImGui::InputInt("BG Palette", &data_.bpal)) {
        changed |= true;
        result = DR_PALETTE_CHANGED;
    }
    Clamp(&data_.bpal, 0, 7);

    ImGui::SameLine();
    changed |= ImGui::InputInt("BG Map", &data_.bmap);
    Clamp(&data_.bmap, 0, 7);

    ImGui::Text("Command List:");
    ImGui::BeginChild("commands", ImGui::GetContentRegionAvail(), true);

    int i = 0, lastx = 0;;
    for(auto it = command_.begin(); it < command_.end(); ++it, ++i) {
        auto next = it + 1;
        bool create = false;
        bool copy = false;

        lastx = it->absx();
        ImGui::PushID(i);
        if (ImGui::Button(ICON_FA_CARET_SQUARE_O_UP)) {
            if (ImGui::GetIO().KeyCtrl)
                copy = true;
            changed = true;
            create = true;
        }
        if (ImGui::IsItemHovered())
            ImGui::SetTooltip("Insert a new command\nCtrl+Click to copy");

        ImGui::SameLine();
        if (ImGui::Button(ICON_FA_CARET_DOWN)) {
            changed = true;
            std::swap(*it, *next);
        }
        if (ImGui::IsItemHovered())
          ImGui::SetTooltip("Move command down");

        ImGui::SameLine();
        changed |= it->Draw(true);

        ImGui::SameLine();
        if (ImGui::Button(ICON_FA_TIMES_CIRCLE)) {
            changed = true;
            it = command_.erase(it);
        }
        if (ImGui::IsItemHovered())
          ImGui::SetTooltip("Delete this command");
        ImGui::PopID();
        if (create) {
            if (copy)
                it = command_.insert(it, it->Copy());
            else
                it = command_.emplace(it, this, it->absx(), 0, 0, 0);
        }
    }
    if (ImGui::Button(ICON_FA_CARET_SQUARE_O_UP)) {
        changed = true;
        command_.emplace_back(this, lastx, 0, 0, 0);
    }
    if (ImGui::IsItemHovered())
        ImGui::SetTooltip("Append a new command");

    ImGui::EndChild();
    ImGui::PopItemWidth();
    Pack();
    data_changed_ |= changed;
    return (changed && result == DR_NONE) ? DR_CHANGED : result;;
}

bool MapHolder::DrawPopup(float scale) {
    bool changed = false;
    for(auto it = command_.begin(); it < command_.end(); ++it) {
        auto result = it->DrawPopup(scale);
        switch(result) {
            case MapCommand::DR_NONE:
                break;
            case MapCommand::DR_CHANGED:
                changed |= true;
                break;
            case MapCommand::DR_COPY:
                it = command_.insert(it, it->Copy());
                changed |= true;
                break;
            case MapCommand::DR_DELETE:
                it = command_.erase(it);
                changed |= true;
                break;
        }
    }
    return changed;
}

void MapHolder::Parse(const z2util::Map& map, uint16_t altaddr) {
    map_ = map;
    // For side view maps, the map address is the address of a pointer
    // to the real address.  Read it and set the real address.
    Address address = mapper_->ReadAddr(map.pointer(), 0);
    if (altaddr) {
        address.set_address(altaddr);
    }
    *map_.mutable_address() = address;
    map_addr_ = address.address();

    length_ = mapper_->Read(address, 0);
    flags_ = mapper_->Read(address, 1);
    ground_ = mapper_->Read(address, 2);
    back_ = mapper_->Read(address, 3);

    command_.clear();
    int absx = 0;
    for(int i=4; i<length_; i+=2) {
        uint8_t pos = mapper_->Read(address, i);
        uint8_t obj = mapper_->Read(address, i+1);
        uint8_t extra = 0;

        int y = pos >> 4;
        if (y < 13 && obj == 15) {
            i++;
            extra = mapper_->Read(address, i+1);
        }
        command_.emplace_back(this, absx, pos, obj, extra);
        absx = command_.back().absx();
    }
    data_changed_ = false;
    addr_changed_ = false;
}

std::vector<uint8_t> MapHolder::MapDataWorker(std::vector<MapCommand>& list) {
    std::vector<uint8_t> map = {length_, flags_, ground_, back_};
    for(auto& cmd : list) {
        auto bytes = cmd.Command();
        map.insert(map.end(), bytes.begin(), bytes.end());
#ifndef NDEBUG
        // Turn this log message off in non-debug builds, as this method is
        // in the sideview editors draw loop.
        LOG(INFO, "CMD: op = ", HEX(bytes[0]), " ", HEX(bytes[1]));
#endif
    }
    map[0] = map.size();
    return map;
}

void MapHolder::Append(const MapCommand& cmd) {
    command_.push_back(cmd);
}

void MapHolder::Extend(const std::vector<MapCommand>& cmds) {
    command_.insert(command_.end(), cmds.begin(), cmds.end());
}

std::vector<uint8_t> MapHolder::MapData() {
    return MapDataWorker(command_);
}

std::vector<uint8_t> MapHolder::MapDataAbs() {
    std::vector<MapCommand> copy = command_;
    if (!cursor_moves_left_) {
        std::stable_sort(copy.begin(), copy.end(),
            [](const MapCommand& a, const MapCommand& b) {
                return a.absx() < b.absx();
            });
    }
    int x = 0;
    for(auto it = copy.begin(); it < copy.end(); ++it) {
        if (it->absy() == 14) {
            // Erase any "skip" commands, as we'll re-synthesize them
            // as needed.
            it = copy.erase(it);
            if (it == copy.end())
                break;
        }
        int deltax = it->absx() - x;
        if (deltax < 0 || deltax > 15) {
            int nx = it->absx() & ~15;
            it = copy.emplace(it, this, x, 0xE0 | (nx/16), 0, 0);
            it++;
            x = nx;
            deltax = it->absx() - x;
        }
        it->set_relx(deltax);
        x = it->absx();
    }

    return MapDataWorker(copy);
}

void MapHolder::Save() {
    if (addr_changed_ && !data_changed_) {
        mapper_->WriteWord(map_.pointer(), 0, map_addr_);
        addr_changed_ = false;
        return;
    }
    if (addr_changed_ && data_changed_) {
        ErrorDialog::Spawn("Unexpected Change When Saving Map",
            "Both the map data and map address have been changed.\n"
            "Allocating a new address and saving data.\n");
    }
    Pack();
    std::vector<uint8_t> data = MapDataAbs();
    LOG(INFO, "Saving ", map_.name(), " (", data.size(), " bytes)");

    Address addr = map_.address();
    // Search the entire bank and allocate memory
    addr.set_address(0);
    addr = mapper_->Alloc(addr, data.size());
    if (addr.address() == 0) {
        ErrorDialog::Spawn("Error Saving Map",
            "Can't save map: ", map_.name(), "\n\n"
            "Can't find ", data.size(), " free bytes in bank ", addr.bank());
        LOG(ERROR, "Can't save map: can't find ", data.size(), "bytes"
                   " in bank=", addr.bank());
        return;
    }
    addr.set_address(0x8000 | addr.address());

    // Free the existing memory if it was owned by the allocator.
    mapper_->Free(map_.address());
    *map_.mutable_address() = addr;
    map_addr_ = addr.address();

    LOG(INFO, "Saving map to offset ", HEX(addr.address()),
              " in bank=", addr.bank());

    for(unsigned i=0; i<data.size(); i++) {
        mapper_->Write(addr, i, data[i]);
    }
    mapper_->WriteWord(map_.pointer(), 0, addr.address());
    data_changed_ = false;
    addr_changed_ = false;
}


MapConnection::MapConnection(Mapper* m)
  : mapper_(m) {}

MapConnection::MapConnection()
  : MapConnection(nullptr) {}

void MapConnection::Parse(const Map& map) {
    uint8_t val;
    connector_ = map.connector();
    doors_ = map.doors();
    world_ = map.world();
    overworld_ = map.overworld();
    subworld_ = map.subworld();

    for(int i=0; i<4; i++) {
        val = mapper_->Read(connector_, i);
        data_[i].destination = val >> 2;
        data_[i].start = val & 3;
    }
    if (doors_.address()) {
        for(int i=0; i<4; i++) {
            val = mapper_->Read(doors_, i);
            data_[i+4].destination = val >> 2;
            data_[i+4].start = val & 3;
        }
    }
}

void MapConnection::Save() {
    for(int i=0; i<4; i++) {
        uint8_t val = (data_[i].destination << 2) | (data_[i].start & 3);
        mapper_->Write(connector_, i, val);
    }
    if (doors_.address()) {
        for(int i=0; i<4; i++) {
            uint8_t val = (data_[i+4].destination << 2) | (data_[i+4].start & 3);
            mapper_->Write(doors_, i, val);
        }
    }
}

bool MapConnection::Draw() {
    const char *destlabel[] = {
        "Left Exit    ",
        "Down Exit    ",
        "Up Exit      ",
        "Right Exit   ",
        "Screen 1 Door",
        "Screen 2 Door",
        "Screen 3 Door",
        "Screen 4 Door",
    };
    const char *startlabel[] = {
        "Left Dest Screen ",
        "Down Dest Screen ",
        "Up Dest Screen   ",
        "Right Dest Screen",
        "Door1 Dest Screen",
        "Door2 Dest Screen",
        "Door3 Dest Screen",
        "Door4 Dest Screen",
    };
    const char *buttonlabel[] = {
        "View Area##0", "View Area##1", "View Area##2", "View Area##3",
        "View Area##4", "View Area##5", "View Area##6", "View Area##7",
    };
    const char *selection = "0\0001\0002\0003\0\0";
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    const char *names[ri.map().size() + 1];
    const Map *maps[ri.map().size() + 1];
    int len = 0;
    bool chg = false;

    for(const auto& m : ri.map()) {
        if (m.type() != MapType::OVERWORLD
            && m.world() == world_
            && m.overworld() == overworld_
            && m.subworld() == subworld_) {
            names[len] = m.name().c_str();
            maps[len] = &m;
            len++;
        }
    }
    names[len++] = "Outside";

    ImGui::Text("Map exit table at bank=0x%x address=0x%04x",
            connector_.bank(), connector_.address());
    int guilen = doors_.address() ? 8 : 4;
    for(int i=0; i<guilen; i++) {
        if (i == 4) {
            ImGui::Separator();
            ImGui::Text("Map door table at bank=0x%x address=0x%04x",
                    doors_.bank(), doors_.address());
        }
        ImGui::PushItemWidth(400);
        chg |= ImGui::Combo(destlabel[i], &data_[i].destination, names, len);
        ImGui::PopItemWidth();
        ImGui::SameLine();
        ImGui::PushItemWidth(100);
        chg |= ImGui::Combo(startlabel[i], &data_[i].start, selection);
        if (data_[i].destination != len-1) {
            ImGui::SameLine();
            if (ImGui::Button(buttonlabel[i])) {
                SimpleMap::Spawn(mapper_, *maps[data_[i].destination],
                                 data_[i].start);
            }
        }
        ImGui::PopItemWidth();
    }
    return chg;
}

MapEnemyList::MapEnemyList(Mapper* m)
  : mapper_(m),
    show_origin_(true),
    is_large_(false),
    is_encounter_(false),
    world_(0),
    overworld_(0),
    subworld_(0),
    area_(0),
    display_(0) { }

MapEnemyList::MapEnemyList() : MapEnemyList(nullptr) {}

void MapEnemyList::Init() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    for(int i=0; i<256; i++)
        names_[i] = "???";

    max_names_ = 0;
    for(const auto& e : ri.enemies()) {
        if (e.world() == world_ && e.overworld() == overworld_) {
            for(const auto& info : e.info()) {
                names_[info.first] = info.second.name().c_str();
                if (info.first >= max_names_)
                    max_names_ = info.first+1;
            }
        }
    }
}

void MapEnemyList::Parse(const Map& map) {
    map_ = map;
    pointer_ = map.pointer();
    world_ = map.world();
    area_ = map.area();
    overworld_ = map.overworld();
    subworld_ = map.subworld();

    Init();
    if (map_.type() == MapType::TOWN) {
        text_.set_mapper(mapper_);
        text_.Unpack(map_.pointer().bank());
    }

    // Check if this is an overworld random encounter area
    OverworldEncounters enc;
    enc.set_mapper(mapper_);
    enc.set_map(map);
    enc.Unpack();
    is_encounter_ = enc.IsEncounter(area_);

    Address addr = mapper_->ReadAddr(pointer_, 0x7e);
    uint16_t delta = 0x18a0;

    data_.clear();
    uint8_t n = mapper_->Read(addr, delta);
    if (is_large_) {
        delta += n;
        n = mapper_->Read(addr, delta);
    }
    for(int i=1; i<n; i+=2) {
        uint8_t pos = mapper_->Read(addr, delta+i);
        uint8_t enemy = mapper_->Read(addr, delta+i+1);
        int y = pos >> 4;
        y = (y == 0) ? 1 : y+2;
        data_.emplace_back(enemy & 0x3f,
                           (pos & 0xf) | (enemy & 0xc0) >> 2, y);
        if (map_.type() == MapType::TOWN && data_.back().enemy >= 10) {
            const auto& tt = ConfigLoader<RomInfo>::GetConfig().text_table();
            const auto& ie = ConfigLoader<RomInfo>::GetConfig().item_effects();
            int townsperson = data_.back().enemy - 10;
            int town = map_.code();
            int idxtbl = (town >> 2) * 2;
            int index = townsperson * 4 + (town & 3);
            for(int j=0; j<2; j++, idxtbl++) {
                if (index < tt.index(idxtbl).length())
                    data_.back().text[j] = mapper_->Read(tt.index(idxtbl),
                                                         index);
            }
            if (townsperson >= 9 && townsperson < 9+4) {
                data_.back().condition = mapper_->Read(ie.conditions_table(), 
                        (townsperson-9)*8 + town);
            }
        }
    }

    // Encounters have 2 enemy lists, so make another widget
    large_.reset(nullptr);
    if (is_encounter_ && !is_large_) {
        large_.reset(new MapEnemyList(mapper_));
        large_->is_large_ = true;
        large_->Parse(map);
    }
}

MapEnemyList::DrawResult MapEnemyList::DrawOnePopup(Unpacked* item,
                                                    float scale) {
    DrawResult result = DR_NONE;
    ImVec2 pos = ImGui::GetCursorPos();
    ImVec2 abs = ImGui::GetCursorScreenPos();
    uint32_t color = 0x800000FF;
    auto* draw = ImGui::GetWindowDrawList();
    float size = 16.0 * scale;

    if (show_origin_) {
        ImVec2 a = ImVec2(abs.x + item->x * size, abs.y + item->y * size);
        ImVec2 b = ImVec2(a.x + size, a.y + size);
        draw->AddRect(a, b, color, 0, ~0, 2.0f);
    }

    ImGui::SetCursorPos(ImVec2(pos.x + item->x * size, pos.y + item->y * size));
    ImGui::InvisibleButton("button", ImVec2(size, size));
    if (ImGui::IsItemHovered())
        ImGui::SetTooltip("%s", names_[item->enemy]);
    if (ImGui::IsItemActive()) {
        if (ImGui::IsMouseDragging()) {
            int x = int((ImGui::GetIO().MousePos.x - abs.x) / size);
            int y = int((ImGui::GetIO().MousePos.y - abs.y) / size);
            x = Clamp(x, 0, 64);
            y = Clamp(y, 0, 12);
            if (x != item->x) {
                item->x = x;
                result = DR_CHANGED;
            }
            if (y != item->y) {
                item->y = y;
                result = DR_CHANGED;
            }
        }
    }
    if (ImGui::BeginPopupContextItem("Properties")) {
        if (DrawOne(item, true)) {
            result = DR_CHANGED;
        }
        if (ImGui::Button("Copy")) {
            result = DR_COPY;
        }
        ImGui::SameLine();
        if (ImGui::Button("Delete")) {
            result = DR_DELETE;
        }
        ImGui::EndPopup();
    }
    ImGui::SetCursorPos(pos);
    return result;
}

bool MapEnemyList::DrawPopup(float scale) {
    bool changed = false;
    int i=0;
    for(auto it=data_.begin(); it<data_.end(); ++it, ++i) {
        ImGui::PushID(-(i | (is_large_ << 8)));
        auto result = DrawOnePopup(&*it, scale);
        switch(result) {
            case MapEnemyList::DR_NONE:
                break;
            case MapEnemyList::DR_CHANGED:
                changed |= true;
                break;
            case MapEnemyList::DR_COPY:
                it = data_.insert(it, *it);
                changed |= true;
                break;
            case MapEnemyList::DR_DELETE:
                it = data_.erase(it);
                changed |= true;
                break;
        }
        ImGui::PopID();
    }
    return changed;
}

std::vector<uint8_t> MapEnemyList::Pack() {
    std::vector<uint8_t> packed;

    uint8_t n = 1 + data_.size() * 2;
    packed.push_back(n);
    for(const auto& data : data_) {
        int y = data.y;
        y = (y <= 1) ? 0 : y-2;
        uint8_t pos = (y << 4) | (data.x & 0x0F);
        uint8_t enemy = (data.enemy & 0x3f) | (data.x & 0x30) << 2;
        packed.push_back(pos);
        packed.push_back(enemy);
    }
    return packed;
}

void MapEnemyList::Save() {
    EnemyListPack ep(mapper_);
    ep.Unpack(pointer_.bank());
    auto data = Pack();
    if (large_) {
        auto more = large_->Pack();
        data.insert(data.end(), more.begin(), more.end());
    }
    ep.Add(area_ + 63 * subworld_, data);
    ep.Pack();

    if (map_.type() == MapType::TOWN) {
        const auto& tt = ConfigLoader<RomInfo>::GetConfig().text_table();
        const auto& ie = ConfigLoader<RomInfo>::GetConfig().item_effects();
        for(const auto& data : data_) {
            if (data.enemy < 10)
                continue;

            int townsperson = data.enemy - 10;
            int town = map_.code();
            int idxtbl = (town >> 2) * 2;
            int index = townsperson * 4 + (town & 3);
            for(int j=0; j<2; j++, idxtbl++) {
                if (index < tt.index(idxtbl).length())
                    mapper_->Write(tt.index(idxtbl), index, data.text[j]);
            }
            if (townsperson >= 9 && townsperson < 9+4) {
                mapper_->Write(ie.conditions_table(), (townsperson-9)*8 + town,
                        data.condition);
            }
        }
    }
}

bool MapEnemyList::DrawOne(Unpacked* item, bool popup) {
    bool chg = false;
    ImGui::PushItemWidth(100);
    chg |= ImGui::InputInt("x position", &item->x);
    if (!popup) ImGui::SameLine();
    chg |= ImGui::InputInt("y position", &item->y);
    if (!popup) ImGui::SameLine();
    ImGui::PopItemWidth();
    ImGui::PushItemWidth(400);
    chg |= ImGui::Combo("enemy", &item->enemy, names_, max_names_);
    ImGui::PopItemWidth();

    if (map_.type() == MapType::TOWN) {
        const auto& tt = ConfigLoader<RomInfo>::GetConfig().text_table();
        int townsperson = item->enemy - 10;
        for(int i=0; i<2; i++) {
            int world = map_.code() >> 2;
            int index = item->text[i];
            if (index < 0)
                continue;

            if (!popup) {
                ImGui::Text("                         ");
                ImGui::SameLine();
            }
            ImGui::PushItemWidth(100);
            chg |= ImGui::InputInt("text      ", &item->text[i]);
            Clamp(&item->text[i], 0, tt.length(world) - 1);

            ImGui::PopItemWidth();
            std::string val;
            text_.Get(world, index, &val);

            ImGui::SameLine();
            ImGui::Text("%s", val.c_str());
        }
        if (townsperson >= 9 && townsperson < 9+4) {
            bool b7 = !!(item->condition & 0x80);
            bool b6 = !!(item->condition & 0x40);
            bool b5 = !!(item->condition & 0x20);
            bool b4 = !!(item->condition & 0x10);
            bool b3 = !!(item->condition & 0x08);
            bool b2 = !!(item->condition & 0x04);
            bool b1 = !!(item->condition & 0x02);
            bool b0 = !!(item->condition & 0x01);
            ImGui::Text("%s                     b7  b6  b5  b4  b3  b2  b1  b0",
                        popup ? "" : "                         ");
            ImGui::Text("%sSatisfier condition:",
                        popup ? "" : "                         ");
            ImGui::SameLine();
            chg |= ImGui::Checkbox("##b7", &b7); ImGui::SameLine();
            chg |= ImGui::Checkbox("##b6", &b6); ImGui::SameLine();
            chg |= ImGui::Checkbox("##b5", &b5); ImGui::SameLine();
            chg |= ImGui::Checkbox("##b4", &b4); ImGui::SameLine();
            chg |= ImGui::Checkbox("##b3", &b3); ImGui::SameLine();
            chg |= ImGui::Checkbox("##b2", &b2); ImGui::SameLine();
            chg |= ImGui::Checkbox("##b1", &b1); ImGui::SameLine();
            chg |= ImGui::Checkbox("##b0", &b0);
            item->condition =
                (b7<<7 | b6<<6 | b5<<5 | b4<<4 | b3<<3 | b2<<2 | b1<<1 | b0);
        }
    }
    return chg;
}

bool MapEnemyList::Draw() {
    bool chg = false;

    Address addr = mapper_->ReadAddr(pointer_, 0x7e);
    ImGui::Text("Map enemy table pointer at bank=0x%x address=0x%04x",
                pointer_.bank(), pointer_.address() + 0x7e);
    ImGui::Text("Map enemy table address at bank=0x%x address=0x%04x",
                addr.bank(), addr.address() + 0x18a0);
    ImGui::Text("Map enemy table RAM addresss=0x%04x", addr.address());

    if (is_encounter_) {
        if (is_large_) {
            ImGui::RadioButton("Large Enemy Encounter", &display_, 1);
        } else {
            ImGui::RadioButton("Small Enemy Encounter", &large_->display_, 0);
        }
    }

    int i=0;
    for(auto it=data_.begin(); it<data_.end(); ++it, ++i) {
        ImGui::PushID(i | (is_large_ << 8));
        chg |= DrawOne(&*it, false);

        ImGui::SameLine();
        if (ImGui::Button(ICON_FA_TIMES_CIRCLE)) {
            chg = true;
            data_.erase(it);
        }
        if (ImGui::IsItemHovered())
            ImGui::SetTooltip("Delete this enemy");
        ImGui::PopID();
        if (map_.type() == MapType::TOWN) {
            ImGui::Separator();
        }
    }
    if (ImGui::Button(ICON_FA_CARET_SQUARE_O_UP)) {
        chg = true;
        data_.emplace_back(0, 0, 0);
    }
    if (ImGui::IsItemHovered())
        ImGui::SetTooltip("Add a new enemy");

    if (large_) {
        ImGui::Separator();
        chg |= large_->Draw();
    }
    return chg;
}

const std::vector<MapEnemyList::Unpacked>& MapEnemyList::data() {
    return (large_ && large_->display_) ? large_->data_ : data_;
}


void MapItemAvailable::Parse(const Map& map) {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();

    area_ = map.area();
    for(const auto& a : ri.available()) {
        if (map.world() == a.world()) {
            if (map.world() != 0) {
                avail_ = a;
            } else if (map.overworld() == a.overworld() &&
                       map.subworld() == a.subworld()) {
                avail_ = a;
            }
        }
    }

    uint8_t a = mapper_->Read(avail_.address(), area_ / 2);
    for(int i=0; i<4; i++) {
        int bit = 7 - (i + 4 * (area_ & 1));
        data_.avail[i] = !!(a & (1 << bit));
    }
}

void MapItemAvailable::Save() {
    uint8_t a = mapper_->Read(avail_.address(), area_ / 2);
    for(int i=0; i<4; i++) {
        int bit = 7 - (i + 4 * (area_ & 1));
        a &= ~(1 << bit);
        a |= data_.avail[i] << bit;
    }
    mapper_->Write(avail_.address(), area_ / 2, a);
}

bool MapItemAvailable::Draw() {
    bool chg = false;
    const char* names[] = {
        "Screen 1",
        "Screen 2",
        "Screen 3",
        "Screen 4",
    };
    for(int i=0; i<4; i++) {
        chg |= ImGui::Checkbox(names[i], &data_.avail[i]);
    }
    return chg;
}

bool MapSwapper::Draw() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    const char *names[ri.map().size() + 1];
    int len = 0;
    bool chg = false;

    for(const auto& m : ri.map()) {
        if (m.type() != MapType::OVERWORLD
            && m.world() == map_.world()
            && m.overworld() == map_.overworld()
            && m.subworld() == map_.subworld()) {
            names[len] = m.name().c_str();
            len++;
        }
    }

    ImGui::PushID(id_);
    ImGui::PushItemWidth(400);
    ImGui::Combo("Source Area", &srcarea_, names, len);
    ImGui::Combo("Destination Area", &dstarea_, names, len);
    ImGui::PopItemWidth();
    ImGui::Checkbox("Map", &swap_map_);
    ImGui::Checkbox("Connections", &swap_conn_);
    ImGui::Checkbox("Enemies", &swap_enemies_);
    ImGui::Checkbox("Item Avalability", &swap_itemav_);
    ImGui::Text("\n");
    ImGui::Text("Note: swapping maps does not modify the overworld connection "
        "table, or the connection table on other maps.");
    ImGui::Text("If you swap maps, you must fix connections yourself.");
    ImGui::Text("\n");
    ImGui::Text("Swap & Copy take effect immediately.  You do not need to \"Commit to ROM\".");
    if (ImGui::Button("Swap")) {
        int src = srcarea_, dst = dstarea_;
        Swap();
        chg = true;
        ImApp::Get()->ProcessMessage("commit", StrCat("Swap ",
                    names[src], " with ", names[dst]).c_str());
    }
    ImGui::SameLine();
    if (ImGui::Button("Copy")) {
        int src = srcarea_, dst = dstarea_;
        Copy();
        chg = true;
        ImApp::Get()->ProcessMessage("commit", StrCat("Copy ",
                    names[src], " to ", names[dst]).c_str());
    }
    ImGui::PopID();
    return chg;
}

void MapSwapper::Swap() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    const Map *a = nullptr;
    const Map *b = nullptr;
    AvailableBitmap avail;

    for(const auto& m : ri.map()) {
        if (m.type() != MapType::OVERWORLD
            && m.world() == map_.world()
            && m.overworld() == map_.overworld()
            && m.subworld() == map_.subworld()) {
            if (m.area() == srcarea_) {
                a = &m;
            } else if (m.area() == dstarea_) {
                b = &m;
            }
        }
    }
    for(const auto& a : ri.available()) {
        if (map_.world() == a.world()
            && map_.overworld() == a.overworld()
            && map_.subworld() == a.subworld()) {
            avail = a;
        }
    }
    LOG(INFO, "Swapping ", a->name(), " with ", b->name());

    if (swap_map_) {
        uint16_t pa = mapper_->ReadWord(a->pointer(), 0);
        uint16_t pb = mapper_->ReadWord(b->pointer(), 0);
        mapper_->WriteWord(a->pointer(), 0, pb);
        mapper_->WriteWord(b->pointer(), 0, pa);
    }
    if (swap_conn_) {
        for(int i=0; i<4; i++) {
            uint8_t ca = mapper_->Read(a->connector(), i);
            uint8_t cb = mapper_->Read(b->connector(), i);
            mapper_->Write(a->connector(), i, cb);
            mapper_->Write(b->connector(), i, ca);
        }
    }
    if (swap_enemies_) {
        uint16_t pa = mapper_->ReadWord(a->pointer(), 0x7e);
        uint16_t pb = mapper_->ReadWord(b->pointer(), 0x7e);
        mapper_->WriteWord(a->pointer(), 0x7e, pb);
        mapper_->WriteWord(b->pointer(), 0x7e, pa);
    }
    if (swap_itemav_) {
        uint8_t aa = mapper_->Read(avail.address(), a->area() / 2);
        uint8_t ba = mapper_->Read(avail.address(), b->area() / 2);
        uint8_t adata = aa, bdata = ba;
        // {a,b}data is the data to swap, while aa,ba is the data to keep
        // in that byte
        if (a->area() & 1) {
            adata &= 0x0F; aa &= 0xF0;
        } else {
            adata >>= 4; aa &= 0x0F;
        }
        if (b->area() & 1) {
            bdata &= 0x0F; ba &= 0xF0;
        } else {
            bdata >>= 4; ba &= 0x0F;
        }
        // Swap the data into the keeper word
        if (a->area() & 1) {
            aa |= bdata;
        } else {
            aa |= bdata << 4;
        }
        if (b->area() & 1) {
            ba |= adata;
        } else {
            ba |= adata << 4;
        }
        // Write back to memory
        mapper_->Write(avail.address(), a->area() / 2, aa);
        mapper_->Write(avail.address(), b->area() / 2, ba);
    }
    set_map(*b);
}

void MapSwapper::Copy() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    const Map *a = nullptr;
    const Map *b = nullptr;
    AvailableBitmap avail;

    for(const auto& m : ri.map()) {
        if (m.type() != MapType::OVERWORLD
            && m.world() == map_.world()
            && m.overworld() == map_.overworld()
            && m.subworld() == map_.subworld()) {
            if (m.area() == srcarea_) {
                a = &m;
            } else if (m.area() == dstarea_) {
                b = &m;
            }
        }
    }
    for(const auto& a : ri.available()) {
        if (map_.world() == a.world()
            && map_.overworld() == a.overworld()
            && map_.subworld() == a.subworld()) {
            avail = a;
        }
    }
    LOG(INFO, "Copying ", a->name(), " to ", b->name());

    if (swap_map_) {
        uint16_t pa = mapper_->ReadWord(a->pointer(), 0);
        mapper_->WriteWord(b->pointer(), 0, pa);
    }
    if (swap_conn_) {
        for(int i=0; i<4; i++) {
            uint8_t ca = mapper_->Read(a->connector(), i);
            mapper_->Write(b->connector(), i, ca);
        }
    }
    if (swap_enemies_) {
        uint16_t pa = mapper_->ReadWord(a->pointer(), 0x7e);
        mapper_->WriteWord(b->pointer(), 0x7e, pa);
    }
    if (swap_itemav_) {
        uint8_t aa = mapper_->Read(avail.address(), a->area() / 2);
        uint8_t ba = mapper_->Read(avail.address(), b->area() / 2);
        uint8_t adata = aa;
        // adata is the data to copy, while aa,ba is the data to keep
        // in that byte
        if (a->area() & 1) {
            adata &= 0x0F; aa &= 0xF0;
        } else {
            adata >>= 4; aa &= 0x0F;
        }
        if (b->area() & 1) {
            ba &= 0xF0;
        } else {
            ba &= 0x0F;
        }

        // Copy the data into the keeper word
        if (b->area() & 1) {
            ba |= adata;
        } else {
            ba |= adata << 4;
        }
        // Write back to memory
        mapper_->Write(avail.address(), b->area() / 2, ba);
    }
    set_map(*b);
}


}  // namespace z2util
