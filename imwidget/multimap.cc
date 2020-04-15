#include "imwidget/multimap.h"

#include "imwidget/imapp.h"
#include "imwidget/map_command.h"
#include "imwidget/simplemap.h"
#include "util/config.h"
#include "util/macros.h"
#include "absl/strings/str_cat.h"
#include "alg/palace_gen.h"

#include <gflags/gflags.h>

DEFINE_bool(town_hack, true, "World 2 towns are really in world 1");

namespace z2util {
namespace {
const double kInvalidStrength = 0.000001;
}  // namespace

MultiMap* MultiMap::Spawn(Mapper* m, int world, int overworld, int subworld,
                          int map) {
    MultiMap* mm = new MultiMap(m, world, overworld, subworld, map);
    mm->Init();
    ImApp::Get()->AddDrawCallback(mm);
    return mm;
}

void MultiMap::Init() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();

    if (FLAGS_town_hack) {
        if (world_ == 2) world_ = 1;
    }
    visited_room0_ = 0;
    int n = 0;
    for(const auto& m : ri.map()) {
        if (m.type() != MapType::OVERWORLD
            && m.world() == world_
            && m.overworld() == overworld_
            // only care about the subworld in world 0 (overworlds)
            && (world_ || m.subworld() == subworld_)) {
            maps_[n] = m;
            visited_[n] = false;
            n++;
        }
    }
    location_.clear();
    graph_.Clear();

    title_ = absl::StrCat("MultiMap: ", maps_[start_].name());
    auto *sc = ConfigLoader<SessionConfig>::MutableConfig();
    mcfg_ = &(*sc->mutable_multimap())[title_];
    if (!mcfg_->initialized()) {
        mcfg_->set_initialized(true);
        mcfg_->set_scale(0.25);
        mcfg_->set_zoom_x(0.50);
        mcfg_->set_zoom_y(0.75);
        mcfg_->set_pause_converge(true);
        mcfg_->set_pre_converge(true);
        mcfg_->set_continuous_converge(true);
        mcfg_->set_show_labels(true);
        mcfg_->set_show_arrows(true);
    }
    title_ = absl::StrCat(title_, "##", id_);
    if (start_ != 0) {
        // Room 0 is often used as the destination for illegal exits
        visited_[0] = true;
    }
    if (mcfg_->pre_converge()) {
        mcfg_->clear_room();
    }
    Traverse(start_, 0, 0, -1);
    if (start_ != 0 && visited_room0_) {
        auto* node = AddRoom(0, 0, 4);
        node->set_charge(0.001);
    }

    if (mcfg_->pre_converge()) {
        for(int i=0; i<10000; i++) {
            graph_.Compute(1.0/60.0);
        }
    }

    if (pgo_.grid_width() == 0) pgo_.set_grid_width(8);
    if (pgo_.grid_height() == 0) pgo_.set_grid_height(8);
    if (pgo_.num_rooms() == 0) pgo_.set_num_rooms(14);
    pgo_.set_start_room(start_);
    pgo_.set_world(world_);
}

fdg::Node* MultiMap::AddRoom(int room, double x, double y) {
    SimpleMap simple(mapper_, maps_[room]);
    auto buffer(simple.RenderToNewBuffer());
    buffer->Update();

    const std::string& name = maps_[room].name();
    auto pos = (*mcfg_->mutable_room())[name];
    if (pos.x() == 0.0 && pos.y() == 0.0) {
        pos.set_x(double(x) + double(room) / 1000.0);
        pos.set_y(double(y) + double(room) / 1000.0);
    }
    fdg::Node *node = graph_.AddNode(room, Vec2(pos.x(), pos.y()));
    location_.emplace(std::make_pair(
                room, DrawLocation{node, std::move(buffer)}));
    return node;
}

void MultiMap::Traverse(int room, double x, double y, int from, double strength) {
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
    k = (d || d == from) ? strength : kInvalidStrength;
    col = (k <= kInvalidStrength) ? GRAY : YELLOW;
    w   = (k <= kInvalidStrength) ? 1 : 3;
    spring->emplace_back(fdg::Spring{d, k, fdg::Bias::Horizontal, col, w,
                                     Direction::LEFT, conn.left().start+1});

    // blue
    d = conn.down().destination;
    k = (d || d == from) ? strength : kInvalidStrength;
    col = (k <= kInvalidStrength) ? GRAY : BLUE;
    w   = (k <= kInvalidStrength) ? 1 : 6;
    spring->emplace_back(fdg::Spring{d, k, fdg::Bias::Vertical, col, w,
                                     Direction::DOWN, Direction::UP});

    // green
    d = conn.up().destination;
    k = (d || d == from) ? strength : kInvalidStrength;
    col = (k <= kInvalidStrength) ? GRAY : GREEN;
    w   = (k <= kInvalidStrength) ? 1 : 3;
    spring->emplace_back(fdg::Spring{d, k, fdg::Bias::Vertical, col, w,
                                     Direction::UP, Direction::DOWN});

    // red
    d = conn.right().destination;
    k = (d || d == from) ? strength : kInvalidStrength;
    col = (k <= kInvalidStrength) ? GRAY : RED;
    w   = (k <= kInvalidStrength) ? 1 : 6;
    spring->emplace_back(fdg::Spring{d, k, fdg::Bias::Horizontal, col, w,
                                     Direction::RIGHT, conn.right().start+1});

    Traverse(conn.left().destination,  x-2.0, y, room, 1.0);
    Traverse(conn.down().destination,  x, y+2.0, room, 1.0);
    Traverse(conn.up().destination,    x, y-2.0, room, 1.0);
    Traverse(conn.right().destination, x+2.0, y, room, 1.0);

    if (mcfg_->show_doors()) {
        const double kWeakStrength = 0.1;
        for(int door=0; door<4; door++) {
            // orange
            d = conn.door(door).destination;
            if (d == 0 || d == 63)
                continue;

            k = (d || d == from) ? kWeakStrength : kInvalidStrength;
            col = (k <= kInvalidStrength) ? GRAY : ORANGE;
            w   = (k <= kInvalidStrength) ? 1 : 3;
            spring->emplace_back(fdg::Spring{d, k, fdg::Bias::Vertical, col, w,
                                             door+5, conn.door(door).start+1});
            Traverse(d, x-1.5+door, y+1.0, room, kWeakStrength);
        }
    }
}

void MultiMap::Sort() {
}

Vec2 MultiMap::Position(const Vec2& pos) {
    float width = 1024.0 * mcfg_->scale(); //+ 8.0;
    float height = 224.0 * mcfg_->scale(); //+ 32.0;
    return Vec2(pos.x * mcfg_->zoom_x() * width,
                pos.y * mcfg_->zoom_y() * height);
}

Vec2 MultiMap::Position(const DrawLocation& dl, Direction side) {
    float w = dl.buffer->width() * mcfg_->scale();
    float h = dl.buffer->height() * mcfg_->scale();
    Vec2 pos = Position(dl.node->pos()) + Vec2(0, 24);
    switch(side) {
        case LEFT:  pos += Vec2(0, h/2.0); break;
        case DOWN:  pos += Vec2(w/2.0, h); break;
        case UP:    pos += Vec2(w/2.0, 0); break;
        case RIGHT: pos += Vec2(w, h/2.0); break;
        case DOOR1: pos += Vec2(w*0.175, h); break;
        case DOOR2: pos += Vec2(w*0.375, h); break;
        case DOOR3: pos += Vec2(w*0.675, h); break;
        case DOOR4: pos += Vec2(w*0.875, h); break;
        default:
            pos += Vec2(w/2.0, h/2.0);
    }
    return pos;
}

void MultiMap::DrawArrow(const Vec2& a, const Vec2&b, uint32_t color,
                         float width, float arrowpos, float rootsize) {
    if (width == 0) width = 2.0f;
    Vec2 u = (b - a).unit();
    Vec2 v = u.flip();
    Vec2 p = a + u * ((b - a).length() * arrowpos);

    auto* draw = ImGui::GetWindowDrawList();
    draw->AddTriangleFilled(p-v*10.0, p+v*10.0, p+u*20.0, color);
    draw->AddCircleFilled(a, rootsize, color);
    draw->AddLine(a, b, color, width);
}

void MultiMap::DrawConnections(const DrawLocation& dl) {
    Direction side = NONE;
    for(const auto& spring : dl.node->connection()) {
        side = Direction(int(side) + 1);
        const auto& loc = location_.find(spring.destid);
        if (loc == location_.end())
            continue;

        Vec2 a = absolute_ + Position(dl, Direction(spring.srcloc));
        Vec2 b = absolute_ + Position(loc->second,
                                      spring.k <= kInvalidStrength
                                          ? Direction::NONE
                                          : Direction(spring.dstloc));
        DrawArrow(a, b, spring.color, spring.width, 0.1, 10.0);
    }
}

void MultiMap::DrawOne(const DrawLocation& dl) {
    int map = dl.node->id();
    Vec2 pos = origin_ + Position(dl.node->pos());
    Vec2 button_height(0, 24);
    ImGui::SetCursorPos(pos);
    const std::string& name = maps_[dl.node->id()].name();
    if (mcfg_->show_labels() &&
        ImGui::Button(name.c_str())) {
        SimpleMap::Spawn(mapper_, maps_[map]);
    }
    pos += button_height;
    ImGui::SetCursorPos(pos);
    ImGui::InvisibleButton(maps_[dl.node->id()].name().c_str(),
                           ImVec2(dl.buffer->width() * mcfg_->scale(),
                                  dl.buffer->height() * mcfg_->scale()));
    dl.node->set_pause(false);
    if (ImGui::IsItemActive()) {
        drag_ |= true;
        if (ImGui::IsMouseDragging()) {
            Vec2 delta = Vec2(ImGui::GetIO().MouseDelta.x /
                                  (1024.0 * mcfg_->zoom_x() * mcfg_->scale()),
                              ImGui::GetIO().MouseDelta.y /
                                  (224.0 * mcfg_->zoom_y() * mcfg_->scale()));
            dl.node->set_pos(dl.node->pos() + delta);
            dl.node->set_pause(true);
        }
    }
    (*mcfg_->mutable_room())[name].set_x(dl.node->pos().x);
    (*mcfg_->mutable_room())[name].set_y(dl.node->pos().y);
    dl.buffer->DrawAt(pos.x, pos.y, mcfg_->scale());
}

void MultiMap::DrawLegend() {
    if (ImGui::BeginPopup("Legend")) {
        ImGui::Text("Legend:");
        auto p = ImGui::GetCursorScreenPos();
        ImGui::Text("Right Exit:                    ");
        DrawArrow(Vec2(100, 8)+p, Vec2(200, 8)+p, RED);

        p = ImGui::GetCursorScreenPos();
        ImGui::Text("Left Exit: ");
        DrawArrow(Vec2(200, 8)+p, Vec2(100, 8)+p, YELLOW);

        p = ImGui::GetCursorScreenPos();
        ImGui::Text("Up Exit:   ");
        DrawArrow(Vec2(100, 8)+p, Vec2(200, 8)+p, GREEN);

        p = ImGui::GetCursorScreenPos();
        ImGui::Text("Down Exit: ");
        DrawArrow(Vec2(200, 8)+p, Vec2(100, 8)+p, BLUE);

        p = ImGui::GetCursorScreenPos();
        ImGui::Text("Illegal Exit:");
        DrawArrow(Vec2(100, 8)+p, Vec2(200, 8)+p, GRAY);

        ImGui::EndPopup();
    }
}

void MultiMap::DrawGen() {
    if (ImGui::BeginPopup("Generate")) {
        int seed = pgo_.seed();
        if (ImGui::InputInt("Seed", &seed)) { pgo_.set_seed(seed); }
        int w = pgo_.grid_width();
        if (ImGui::InputInt("Width", &w)) {  pgo_.set_grid_width(w); }
        ImGui::SameLine();
        int h = pgo_.grid_height();
        if (ImGui::InputInt("Height", &h)) {  pgo_.set_grid_height(h); }

        int n = pgo_.num_rooms();
        if (ImGui::InputInt("Rooms", &n)) {  pgo_.set_num_rooms(n); }

        bool efr = pgo_.enter_on_first_row();
        if (ImGui::Checkbox("Enter on first row", &efr)) { 
            pgo_.set_enter_on_first_row(efr);
        }

        bool jump = pgo_.jump_required();
        if (ImGui::Checkbox("Jump required", &jump)) {
            pgo_.set_jump_required(jump);
        }
        bool glove = pgo_.glove_required();
        if (ImGui::Checkbox("Glove required", &glove)) {
            pgo_.set_glove_required(glove);
        }
        bool fairy = pgo_.fairy_required();
        if (ImGui::Checkbox("Fairy required", &fairy)) {
            pgo_.set_fairy_required(fairy);
        }



        if (ImGui::Button("Generate")) {
            PalaceGenerator pgen(pgo_);
            pgen.set_mapper(mapper_);
            pgen.Generate();
            Init();
        }
        ImGui::EndPopup();
    }
}

bool MultiMap::Draw() {
    if (!visible_)
        return false;

    drag_ = false;
    ImGui::SetNextWindowSize(ImVec2(1024, 700), ImGuiCond_FirstUseEver);
    ImGui::Begin(title_.c_str(), &visible_);
    ImGui::PushItemWidth(100);
    WITH_PROTO_FIELD(*mcfg_, scale,
        ImGui::InputFloat("Zoom", &scale, 1.0/8.0, 1.0));
    ImGui::PopItemWidth();

    ImGui::SameLine();
    if (ImGui::Button("Refresh")) {
        Init();
    }
    ImGui::SameLine();
    if (ImGui::Button("Properties")) {
        ImGui::OpenPopup("Properties");
    }
    if (ImGui::BeginPopup("Properties")) {
        WITH_PROTO_FIELD(*mcfg_, zoom_x,
            ImGui::SliderFloat("X-Zoom", &zoom_x, 0.001f, 1.0f));
        WITH_PROTO_FIELD(*mcfg_, zoom_y,
            ImGui::SliderFloat("Y-Zoom", &zoom_y, 0.001f, 1.0f));
        WITH_PROTO_FIELD(*mcfg_, pause_converge,
            ImGui::Checkbox("Pause Convergence while dragging", &pause_converge));
        WITH_PROTO_FIELD(*mcfg_, pre_converge,
            ImGui::Checkbox("Converge before first draw", &pre_converge));
        WITH_PROTO_FIELD(*mcfg_, continuous_converge,
            ImGui::Checkbox("Converge during draw", &continuous_converge));
        WITH_PROTO_FIELD(*mcfg_, show_labels,
            ImGui::Checkbox("Show labels", &show_labels));
        WITH_PROTO_FIELD(*mcfg_, show_arrows,
            ImGui::Checkbox("Show arrows", &show_arrows));
        WITH_PROTO_FIELD(*mcfg_, show_doors,
            ImGui::Checkbox("Show door connections", &show_doors));
        ImGui::EndPopup();
    }

    ImGui::SameLine();
    if (ImGui::Button("Legend")) {
        ImGui::OpenPopup("Legend");
    }
    DrawLegend();

    ImGui::SameLine();
    if (ImGui::Button("Generate")) {
        ImGui::OpenPopup("Generate");
    }
    DrawGen();

    ImGui::SameLine();
    ImApp::Get()->HelpButton("overworld-editor");

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

    if (mcfg_->show_arrows()) {
        for(const auto& dl : location_)
            DrawConnections(dl.second);
    }
    for(const auto& dl : location_) {
        DrawOne(dl.second);
    }

    ImGui::EndChild();
    ImGui::End();

    if (mcfg_->continuous_converge() && !(drag_ && mcfg_->pause_converge()))
        graph_.Compute(1.0/60.0);

    return false;
}

}  // namespace z2util
