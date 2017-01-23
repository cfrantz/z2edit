#include "imwidget/multimap.h"

#include "imwidget/map_command.h"
#include "imwidget/simplemap.h"
#include "imapp-util.h"

#include "util/config.h"

namespace z2util {

float MultiMap::xs_ = 0.40;
float MultiMap::ys_ = 0.75;
bool MultiMap::preconverge_ = true;

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

    visited_room0_ = 0;
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
    if (start_ != 0) {
        // Room 0 is often used as the destination for illegal exits
        visited_[0] = true;
    }
    Traverse(start_, 0, 0, -1);
    if (start_ != 0 && visited_room0_) {
        auto* node = AddRoom(0, 0, 4);
        node->set_charge(0.001);
    }

    if (preconverge_) {
        for(int i=0; i<10000; i++) {
            graph_.Compute(1.0/60.0);
        }
    }
}

fdg::Node* MultiMap::AddRoom(int room, int x, int y) {
    SimpleMap simple(mapper_, maps_[room]);
    auto buffer(simple.RenderToNewBuffer());
    buffer->Update();

    double xx = double(x) + room / 1000.0;
    double yy = double(y) + room / 1000.0;
    fdg::Node *node = graph_.AddNode(room, Vec2(xx, yy));

    location_.emplace(std::make_pair(
                room, DrawLocation{node, std::move(buffer)}));
    return node;
}

void MultiMap::Traverse(int room, int x, int y, int from) {
    if (room == 0)
        visited_room0_++;
    if (room == 63 || visited_[room])
        return;
    visited_[room] = true;

    auto* node = AddRoom(room, x, y);
    MapConnection conn;
    conn.set_mapper(mapper_);
    conn.Parse(maps_[room]);

    double k, w;
    int d;
    uint32_t col;
    auto* spring = node->mutable_connection();

    // yellow
    d = conn.left().destination;
    //k = d ? 1 : 0.001;
    k = (d || d == from) ? 1 : 0.001;
    col = (k < 1.0) ? GRAY : YELLOW;
    w   = (k < 1.0) ? 1 : 3;
    spring->emplace_back(fdg::Spring{d, k, fdg::Bias::Horizontal, col, w});

    // blue
    d = conn.down().destination;
    k = (d || d == from) ? 1 : 0.001;
    col = (k < 1.0) ? GRAY : BLUE;
    w   = (k < 1.0) ? 1 : 6;
    spring->emplace_back(fdg::Spring{d, k, fdg::Bias::Vertical, col, w});

    // green
    d = conn.up().destination;
    k = (d || d == from) ? 1 : 0.001;
    col = (k < 1.0) ? GRAY : GREEN;
    w   = (k < 1.0) ? 1 : 3;
    spring->emplace_back(fdg::Spring{d, k, fdg::Bias::Vertical, col, w});

    // red
    d = conn.right().destination;
    k = (d || d == from) ? 1 : 0.001;
    col = (k < 1.0) ? GRAY : RED;
    w   = (k < 1.0) ? 1 : 6;
    spring->emplace_back(fdg::Spring{d, k, fdg::Bias::Horizontal, col, w});

    Traverse(conn.left().destination,  x-1, y, room);
    Traverse(conn.down().destination,  x, y+1, room);
    Traverse(conn.up().destination,    x, y-1, room);
    Traverse(conn.right().destination, x+1, y, room);
}

void MultiMap::Sort() {
}

Vec2 MultiMap::Position(const Vec2& pos) {
    float width = 1024.0 * scale_; //+ 8.0;
    float height = 224.0 * scale_; //+ 32.0;
    return Vec2(pos.x * xs_ * width, pos.y * ys_ * height);
}

Vec2 MultiMap::Position(const DrawLocation& dl, Direction side) {
    float w = dl.buffer->width() * scale_;
    float h = dl.buffer->height() * scale_;
    Vec2 pos = Position(dl.node->pos()) + Vec2(0, 24);
    switch(side) {
        case LEFT:  pos += Vec2(0, h/2.0); break;
        case DOWN:  pos += Vec2(w/2.0, h); break;
        case UP:    pos += Vec2(w/2.0, 0); break;
        case RIGHT: pos += Vec2(w, h/2.0); break;
        default:
            pos += Vec2(w/2.0, h/2.0);
    }
    return pos;
}

void MultiMap::DrawConnections(const DrawLocation& dl) {
    Direction side = NONE;
    auto* draw = ImGui::GetWindowDrawList();
    for(const auto& spring : dl.node->connection()) {
        side = Direction(int(side) + 1);
        const auto& loc = location_.find(spring.destid);
        if (loc == location_.end())
            continue;

        Vec2 a = absolute_ + Position(dl, side);
        Vec2 b = absolute_ + Position(loc->second, NONE);
        ImU32 color = spring.color;

        float linewidth = spring.width;
        if (linewidth == 0) linewidth = 2.0f;

        Vec2 u = (b - a).unit();
        Vec2 v = u.flip();
        Vec2 p = a + u * (b - a).length() * 0.1;
        draw->AddTriangleFilled(p-v*10.0, p+v*10.0, p+u*20.0, color);
        draw->AddCircleFilled(a, 10.0, color);
        draw->AddLine(a, b, color, linewidth);
    }
}

void MultiMap::DrawOne(const DrawLocation& dl) {
    int map = dl.node->id();
    Vec2 pos = origin_ + Position(dl.node->pos());
    Vec2 button_height(0, 24);
    ImGui::SetCursorPos(pos);
    if (ImGui::Button(maps_[dl.node->id()].name().c_str())) {
        SimpleMap::New(mapper_, maps_[map]);
    }
    pos += button_height;
    ImGui::SetCursorPos(pos);
    ImGui::InvisibleButton(maps_[dl.node->id()].name().c_str(),
                           ImVec2(dl.buffer->width() * scale_,
                                  dl.buffer->height() * scale_));
    if (ImGui::IsItemActive()) {
        drag_ |= true;
        if (ImGui::IsMouseDragging()) {
            Vec2 delta = Vec2(ImGui::GetIO().MouseDelta.x / (1024.0 * xs_ * scale_),
                              ImGui::GetIO().MouseDelta.y / (224.0 * ys_ * scale_));
            dl.node->set_pos(dl.node->pos() + delta);
        }
    }
    dl.buffer->DrawAt(pos.x, pos.y, scale_);
}

void MultiMap::Draw() {
    if (!visible_)
        return;

    drag_ = false;
    ImGui::Begin(title_.c_str(), &visible_);
    ImGui::PushItemWidth(100);
    ImGui::InputFloat("Zoom", &scale_, 1.0/8.0, 1.0);
    ImGui::PopItemWidth();
    ImGui::SameLine();
    if (ImGui::Button("Properties")) {
        ImGui::OpenPopup("Properties");
    }
    if (ImGui::BeginPopup("Properties")) {
        ImGui::SliderFloat("X-Zoom", &xs_, 0.001f, 1.0f);
        ImGui::SliderFloat("Y-Zoom", &ys_, 0.001f, 1.0f);
        ImGui::Checkbox("Pause Convergence while dragging", &pauseconv_);
        ImGui::Checkbox("Converge before first draw", &preconverge_);
        ImGui::EndPopup();
    }

    Vec2 minv(1e9, 1e9);
    Vec2 maxv(-1e9, -1e9);
    for(const auto& dl : location_) {
        Vec2 p = dl.second.node->pos();
        minv.x = std::min(minv.x, p.x);
        minv.y = std::min(minv.y, p.y);
        maxv.x = std::max(maxv.x, p.x);
        maxv.y = std::max(maxv.y, p.y);
    }
    maxv += Vec2(1, 1);
    minv = Position(minv);
    maxv = Position(maxv);
    ImGui::BeginChild("image", ImVec2(0, 0), true,
                      ImGuiWindowFlags_AlwaysHorizontalScrollbar |
                      ImGuiWindowFlags_AlwaysVerticalScrollbar);
    origin_ = -minv + ImGui::GetCursorPos();
    absolute_ = -minv + ImGui::GetCursorScreenPos();

    for(const auto& dl : location_)
        DrawConnections(dl.second);
    for(const auto& dl : location_)
        DrawOne(dl.second);

    ImGui::EndChild();
    ImGui::End();
    if (!(drag_ && pauseconv_))
        graph_.Compute(1.0/60.0);
}

}  // namespace z2util