#include "alg/palace_gen.h"

#include "imwidget/map_command.h"
#include "util/config.h"
#include "proto/rominfo.pb.h"

namespace z2util {

PalaceGenerator::PalaceGenerator(PalaceGeneratorOptions& opt)
  : opt_(opt),
    map_(opt.grid_height(),
         std::vector<Room>(opt.grid_width(), Room{-1, })),
    real_(0.0, 1.0),
    room_(0) {
    rng_.seed(opt.seed());
    if (opt_.horizontal_bias() == 0.0)
        opt_.set_horizontal_bias(0.75);
    InitPalaceMaps();
}

void PalaceGenerator::InitPalaceMaps() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    int n = 0;
    for(const auto& m : ri.map()) {
        if (m.type() != MapType::OVERWORLD
            && m.world() == opt_.world()) {
            palace_maps_[n] = m;
            n++;
        }
    }
}

void PalaceGenerator::Generate() {
    GenerateMaze();
    MapToRooms();
    SimplePrint();
    for(const auto& r : rooms_) {
        PrepareRoom(r.room);
    }
}

void PalaceGenerator::GenerateMaze() {
    int x, y;
    int x0, y0;
    if (opt_.enter_on_first_row()) {
        y = 0;
    } else {
        y = integer(opt_.grid_height());
    }
    x = integer(opt_.grid_width());

    x0 = x; y0 = y;

    map_[y][x].room = room_++;
    while(room_ < opt_.num_rooms()) {
        int nx = x, ny = y;
        if (bit()) {
            nx += bit() ? 1 : -1;
            if (map_[y][x].room == 0 && nx < x) {
                // Not allowed exit left from room 0
                continue;
            }
        } else {
            ny += bit() ? 1 : -1;
        }
        if (nx >= 0 && ny >= 0
            && nx < opt_.grid_width() && ny < opt_.grid_height()) {
            x = nx; y = ny;
        }
        if (map_[y][x].room == -1) {
            map_[y][x].room = room_++;
        }
    }
    VisitRooms(x0, y0);
}

void PalaceGenerator::VisitRooms(int x, int y) {
    int unvisited = opt_.num_rooms() - 1;
    map_[y][x].visited = true;

    struct Node {
        int y, x;
    };

    std::vector<Node> stack;
    while(unvisited) {
        std::vector<Node> hneighbors;
        std::vector<Node> vneighbors;
        Room* r;

        if (x > 0) {
            r = &map_[y][x-1];
            if (r->room >=0 && !r->visited)
                hneighbors.emplace_back(Node{y, x-1});
        }

        if (x < opt_.grid_width() - 1) {
            r = &map_[y][x+1];
            if (r->room >=0 && !r->visited)
                hneighbors.emplace_back(Node{y, x+1});
        }

        if (y > 0) {
            r = &map_[y-1][x];
            if (r->room >=0 && !r->visited)
                vneighbors.emplace_back(Node{y-1, x});
        }

        if (y < opt_.grid_height() - 1) {
            r = &map_[y+1][x];
            if (r->room >=0 && !r->visited)
                vneighbors.emplace_back(Node{y+1, x});
        }

        size_t hn = hneighbors.size();
        size_t vn = vneighbors.size();
        if (hn + vn) {
            Node next;
            if (hn && vn) {
                if (bit(opt_.horizontal_bias())) {
                    next = hneighbors.at(integer(hn));
                } else {
                    next = vneighbors.at(integer(vn));
                }
            } else if (hn) { 
                next = hneighbors.at(integer(hn));
            } else {
                next = vneighbors.at(integer(vn));
            }
            stack.emplace_back(Node{y, x});
            if (next.x > x) {
                map_[y][x].has_right = true;
                map_[y][x].right = map_[next.y][next.x].room;
                map_[next.y][next.x].has_left = true;
                map_[next.y][next.x].left = map_[y][x].room;
            } else if (next.x < x) {
                map_[y][x].has_left = true;
                map_[y][x].left = map_[next.y][next.x].room;
                map_[next.y][next.x].has_right = true;
                map_[next.y][next.x].right = map_[y][x].room;
            } else if (next.y > y) {
                map_[y][x].has_down = true;
                map_[y][x].down = map_[next.y][next.x].room;
                map_[next.y][next.x].has_up = true;
                map_[next.y][next.x].up = map_[y][x].room;
            } else if (next.y < y) {
                map_[y][x].has_up = true;
                map_[y][x].up = map_[next.y][next.x].room;
                map_[next.y][next.x].has_down = true;
                map_[next.y][next.x].down = map_[y][x].room;
            }
            map_[next.y][next.x].visited = true;
            unvisited--;
            y = next.y; x = next.x;
        } else if (!stack.empty()) {
            Node next = stack.back();
            y = next.y; x = next.x;
            stack.pop_back();
        }
    }
}

void PalaceGenerator::MapToRooms() {
    for(const auto& row : map_) {
        for(const auto& room : row) {
            if (room.room != -1) {
                rooms_.push_back(room);
            }
        }
    }
    std::stable_sort(rooms_.begin(), rooms_.end(),
        [](const Room& a, const Room& b) { return a.room < b.room; });
}

#define WINDOW(x, y)           MapCommand(&holder, x, (y)<<4, 0x00, 0x00)
#define UNICORNHEAD(x, y)      MapCommand(&holder, x, (y)<<4, 0x01, 0x00)
#define WOLFHEAD(x, y)         MapCommand(&holder, x, (y)<<4, 0x02, 0x00)
#define CRYSTALRETURN(x, y)    MapCommand(&holder, x, (y)<<4, 0x03, 0x00)
#define LOCKEDDOOR(x, y)       MapCommand(&holder, x, (y)<<4, 0x05, 0x00)
#define CLOUD1(x, y)           MapCommand(&holder, x, (y)<<4, 0x07, 0x00)
#define CLOUD2(x, y)           MapCommand(&holder, x, (y)<<4, 0x08, 0x00)
#define IKSTATUE(x, y)         MapCommand(&holder, x, (y)<<4, 0x09, 0x00)

#define HORIZPIT1(x, y, w)    MapCommand(&holder, x, (y)<<4, 0x10|((w)-1), 0x00)
#define PALACEBRICKS1(x, y, w) MapCommand(&holder, x, (y)<<4, 0x20|((w)-1), 0x00)
#define BREAKBLOCK1(x, y, w)  MapCommand(&holder, x, (y)<<4, 0x30|((w)-1), 0x00)
#define STEELBRICKS(x, y, w)  MapCommand(&holder, x, (y)<<4, 0x40|((w)-1), 0x00)
#define CRUMBLEBRIDGE(x, y, w) MapCommand(&holder, x, (y)<<4, 0x50|((w)-1), 0x00)
#define PALACEBRICKS2(x, y, w) MapCommand(&holder, x, (y)<<4, 0x70|((w)-1), 0x00)
#define CURTAINS(x, y, w)     MapCommand(&holder, x, (y)<<4, 0x80|((w)-1), 0x00)
#define BREAKBLOCK2(x, y, w)  MapCommand(&holder, x, (y)<<4, 0x90|((w)-1), 0x00)
#define FAKEWALL(x, y, w)     MapCommand(&holder, x, (y)<<4, 0xA0|((w)-1), 0x00)
#define BREAKBLOCKV(x, y, h)  MapCommand(&holder, x, (y)<<4, 0xB0|((h)-1), 0x00)
#define COLUMN(x, y, h)       MapCommand(&holder, x, (y)<<4, 0xC0|((h)-1), 0x00)
#define BRIDGE(x, y, w)       MapCommand(&holder, x, (y)<<4, 0xD0|((w)-1), 0x00)
#define PIT(x, y, w)          MapCommand(&holder, x, (y)<<4, 0xF0|((w)-1), 0x00)

#define LAVA(x, w)            MapCommand(&holder, x, 0xF0, 0x10|((w)-1), 0x00)
#define ELEVATOR(x)           MapCommand(&holder, x, 0xF0, 0x50, 0x00)
#define SET_FLOOR(x, arg)     MapCommand(&holder, x, 0xD0, arg, 0x00)


void PalaceGenerator::PrepareRoom(int r) {
    int start = opt_.start_room();
    MapHolder holder(mapper_);
    MapConnection connection(mapper_);

    holder.Parse(palace_maps_[start+r]);
    connection.Parse(palace_maps_[start+r]);
    int bg = (r == 0) ? 0 : 1;
    int tileset = (r == 0) ? 1 : 0;
    holder.Clear({0, 4, false, false, true, tileset, 0, 0, bg, 0});

    if (rooms_[r].has_left) {
    } else {
        // Start with a wall, then reset the floor to 0.
        holder.set_floor(15);
        holder.Append(SET_FLOOR(2, 0));
    }
    if (rooms_[r].has_right) {
    } else {
        // Put a wall at the last 2 columns of tho room.
        holder.Append(SET_FLOOR(62, 15));
    }

    if (rooms_[r].has_up && rooms_[r].has_down) {
        // Elevator shaft (pit) floor to ceiling.
        holder.Append(PIT(39, 0, 2));
        holder.Append(ELEVATOR(39));
    } else if (rooms_[r].has_up) {
        // Elevator shaft (pit) floor to ceiling.
        // Fill in floor with palace bricks.
        holder.Append(PIT(39, 0, 2));
        holder.Append(PALACEBRICKS2(39, 11, 2));
        holder.Append(ELEVATOR(39));
    } else if (rooms_[r].has_down) {
        // Elevator shaft (pit) from 7 on down.
        holder.Append(PIT(39, 7, 2));
        holder.Append(ELEVATOR(39));
    }

    // Window
    holder.Append(WINDOW(4, 6));
    connection.set_left(rooms_[r].left, 3);
    connection.set_right(rooms_[r].right, 0);
    connection.set_up(rooms_[r].up, 2);
    connection.set_down(rooms_[r].down, 2);

    holder.Save();
    connection.Save();
}

void PalaceGenerator::SimplePrint() {
    for(int y=0; y<opt_.grid_height(); y++) {
        for(int x=0; x<opt_.grid_width(); x++) {
            if (map_[y][x].room == -1) {
                printf("XXXXXXX");
            } else {
                char buf[8] = "[     ]";
                if (map_[y][x].left) buf[0] = '<';
                if (map_[y][x].right) buf[6] = '>';
                if (map_[y][x].room == 0) buf[0] = 'E';
                if (map_[y][x].up) buf[3] = '^';
                if (map_[y][x].down) buf[3] = 'v';
                if (map_[y][x].up && map_[y][x].down) buf[3] = '|';

                int r = map_[y][x].room;
                buf[1] = '0' + (r/10);
                buf[2] = '0' + (r%10);
                printf("%s", buf);
            }
        }
        printf("\n");
    }
}



}  // namespace
