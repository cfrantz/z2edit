#include "imwidget/multimap.h"
#include "imwidget/map_command.h"
#include "imwidget/simplemap.h"
#include "imapp-util.h"

#include "util/config.h"

namespace z2util {

MultiMap* MultiMap::New(Mapper* m, int world, int map) {
    MultiMap* mm = new MultiMap(m, world, map);
    mm->Init();
    AddDrawCallback([mm]() {
        bool vis = mm->visible_;
        if (vis) {
            mm->Draw();
        } else {
            delete mm;
        }
        return vis;
    });
    return mm;
}

void MultiMap::Init() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();

    int n = 0;
    for(const auto& m : ri.map()) {
        if (m.type() != MapType::OVERWORLD &&
            m.valid_worlds() & (1UL << world_)) {
            maps_[n] = m;
            visited_[n] = false;
            n++;
        }
    }
    location_.clear();

    title_ = "MultiMap: " + maps_[start_].name();
    Traverse(start_, 0, 0, NONE);
    Sort();
}

void MultiMap::Traverse(int room, int x, int y, From from) {
    if (room == 63 || visited_[room])
        return;
    visited_[room] = true;
    SimpleMap simple(mapper_, maps_[room]);
    auto buffer(simple.RenderToNewBuffer());
    buffer->Update();
    location_.emplace_back(room, x, y, buffer.release());

    MapConnection conn;
    conn.set_mapper(mapper_);
    conn.Parse(maps_[room]);
    if (from != LEFT)  Traverse(conn.left().destination,  x-1, y, RIGHT);
    if (from != DOWN)  Traverse(conn.down().destination,  x, y+1, UP);
    if (from != UP)    Traverse(conn.up().destination,    x, y-1, DOWN);
    if (from != RIGHT) Traverse(conn.right().destination, x+1, y, LEFT);
}

void MultiMap::Sort() {
    int minx = 0, miny = 0;
    int maxx = 0, maxy = 0;
    for(const auto& dl : location_) {
        minx = std::min(minx, dl.x);
        miny = std::min(miny, dl.y);
        maxx = std::max(maxx, dl.x);
        maxy = std::max(maxy, dl.y);
    }
    minx = (minx < 0) ? -minx : 0;
    miny = (miny < 0) ? -miny : 0;

    for(auto& dl : location_) {
        dl.x += minx;
        dl.y += miny;
    }
    maxx_ = maxx + minx;
    maxy_ = maxy + miny;
}

void MultiMap::DrawOne(ImVec2 pos, const DrawLocation& dl) {
    float width = 1024.0 * scale_ + 8.0;
    float height = 224.0 * scale_ + 32.0;

    pos.x += dl.x * width;
    pos.y += dl.y * height;
    ImGui::SetCursorPos(pos);
    if (ImGui::Button(maps_[dl.map].name().c_str())) {
        SimpleMap::New(mapper_, maps_[dl.map]);
    }
    pos.y += 24;
    dl.buffer->DrawAt(pos.x, pos.y, scale_);
}

void MultiMap::Draw() {
    if (!visible_)
        return;

    ImGui::Begin(title_.c_str(), &visible_);
    ImGui::PushItemWidth(100);
    ImGui::InputFloat("Zoom", &scale_, 0.25, 1.0);
    ImGui::PopItemWidth();

    ImGui::BeginChild("image", ImVec2(0, 16 + (maxy_ + 1) * (224.0 * scale_ + 32)),
                      true, ImGuiWindowFlags_HorizontalScrollbar);
    ImVec2 cursor = ImGui::GetCursorPos();
    for(const auto& dl : location_) {
        DrawOne(cursor, dl);
    }
    ImGui::EndChild();
    ImGui::End();
}

}  // namespace z2util
