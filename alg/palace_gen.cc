#include "alg/palace_gen.h"

#include "imwidget/map_command.h"
#include "util/config.h"
#include "proto/rominfo.pb.h"

namespace z2util {

const PalaceGenerator::FloorCeiling PalaceGenerator::fpos_[] = {
        {0, 11}, {0, 10}, {0, 9}, {0, 8}, {0, 7}, {0, 6}, {0, 5}, {0, 4},
        {1, 11}, {2, 11}, {3, 11}, {4, 11}, {5, 11}, {6, 11}, {7, 11},
};

PalaceGenerator::PalaceGenerator(PalaceGeneratorOptions& opt)
  : opt_(opt),
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
    bool ok;
    do {
        GenerateMaze();
        MapToRooms();
        WalkMaze(0, 0, LEFT);
        ok = SelectSpecialRooms();
        printf("special rooms = %d\n", ok);
    } while(!ok);

    SimplePrint();
    holder_.reset(new MapHolder(mapper_));
    for(const auto& r : rooms_) {
        PrepareRoom(r.room);
    }
    FixElevatorConnections();
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
    room_ = 0;
    map_.clear();
    map_.resize(opt_.grid_height(),
                std::vector<Room>(opt_.grid_width(), Room{-1, })),

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
    printf("Basic Layout\n");
    SimplePrint();
    VisitRooms(x0, y0);
    printf("With Connections\n");
    SimplePrint();
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
            // Special case: room0 not allowed to have left-exit
            if (map_[y][x].room !=0 && r->room >=0 && !r->visited)
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
    rooms_.clear();
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

void PalaceGenerator::WalkMaze(int r, int distance, Direction from) {
    Room& room = rooms_[r];
    room.distance = distance;
    int exits = 1;

    if (room.has_left && from != LEFT) {
        exits++;
        WalkMaze(room.left, distance+1, RIGHT);
    }
    if (room.has_right && from != RIGHT) {
        exits++;
        WalkMaze(room.right, distance+1, LEFT);
    }
    if (room.has_up && from != UP) {
        exits++;
        WalkMaze(room.up, distance+1, DOWN);
    }
    if (room.has_down && from != DOWN) {
        exits++;
        WalkMaze(room.down, distance+1, UP);
    }

    if (exits == 1)
        room.dead_end = true;
}

bool PalaceGenerator::SelectSpecialRooms() {
    int distance = 0;
    int room = 0;
    // The boss room is the farthest room which is a rightwards dead end.
    for(const auto& r : rooms_) {
        if (r.dead_end && !r.has_right && r.distance > distance) {
            room = r.room;
            distance = r.distance;
        }
    }
    if (room == 0)
        return false;
    rooms_[room].boss_room = true;

    distance = 0;
    room = 0;
    // The item room is the farthest room which is any kind of dead-end.
    for(const auto& r : rooms_) {
        if (r.dead_end && !r.boss_room && r.distance > distance) {
            room = r.room;
            distance = r.distance;
        }
    }
    if (room == 0)
        return false;
    rooms_[room].item_room = true;
    return true;
}

#define WINDOW(x, y)           MapCommand(holder_.get(), x, (y)<<4, 0x00, 0x00)
#define UNICORNHEAD(x, y)      MapCommand(holder_.get(), x, (y)<<4, 0x01, 0x00)
#define WOLFHEAD(x, y)         MapCommand(holder_.get(), x, (y)<<4, 0x02, 0x00)
#define CRYSTALRETURN(x, y)    MapCommand(holder_.get(), x, (y)<<4, 0x03, 0x00)
#define LOCKEDDOOR(x, y)       MapCommand(holder_.get(), x, (y)<<4, 0x05, 0x00)
#define CLOUD1(x, y)           MapCommand(holder_.get(), x, (y)<<4, 0x07, 0x00)
#define CLOUD2(x, y)           MapCommand(holder_.get(), x, (y)<<4, 0x08, 0x00)
#define IKSTATUE(x, y)         MapCommand(holder_.get(), x, (y)<<4, 0x09, 0x00)

#define HORIZPIT1(x, y, w)     MapCommand(holder_.get(), x, (y)<<4, 0x10|((w)-1), 0x00)
#define PALACEBRICKS1(x, y, w) MapCommand(holder_.get(), x, (y)<<4, 0x20|((w)-1), 0x00)
#define BREAKBLOCK1(x, y, w)   MapCommand(holder_.get(), x, (y)<<4, 0x30|((w)-1), 0x00)
#define STEELBRICKS(x, y, w)   MapCommand(holder_.get(), x, (y)<<4, 0x40|((w)-1), 0x00)
#define CRUMBLEBRIDGE(x, y, w) MapCommand(holder_.get(), x, (y)<<4, 0x50|((w)-1), 0x00)
#define PALACEBRICKS2(x, y, w) MapCommand(holder_.get(), x, (y)<<4, 0x70|((w)-1), 0x00)
#define CURTAINS(x, y, w)      MapCommand(holder_.get(), x, (y)<<4, 0x80|((w)-1), 0x00)
#define BREAKBLOCK2(x, y, w)   MapCommand(holder_.get(), x, (y)<<4, 0x90|((w)-1), 0x00)
#define FAKEWALL(x, y, w)      MapCommand(holder_.get(), x, (y)<<4, 0xA0|((w)-1), 0x00)
#define BREAKBLOCKV(x, y, h)   MapCommand(holder_.get(), x, (y)<<4, 0xB0|((h)-1), 0x00)
#define COLUMN(x, y, h)        MapCommand(holder_.get(), x, (y)<<4, 0xC0|((h)-1), 0x00)
// Apparently bridge doesn't work.
#define BRIDGE(x, y, w)        MapCommand(holder_.get(), x, (y)<<4, 0xD0|((w)-1), 0x00)
#define PIT(x, y, w)           MapCommand(holder_.get(), x, (y)<<4, 0xF0|((w)-1), 0x00)
#define LAVA(x, w)             MapCommand(holder_.get(), x, 0xF0, 0x10|((w)-1), 0x00)
#define ELEVATOR(x)            MapCommand(holder_.get(), x, 0xF0, 0x50, 0x00)
#define SET_FLOOR(x, arg)      MapCommand(holder_.get(), x, 0xD0, arg, 0x00)

void PalaceGenerator::MakeFakeCeiling(int r, int x, int w, int downto) {
    int odd = downto & 1;
    downto &= ~1;
    for(int h=0; h<downto; h+=2) {
        holder_->Append(PALACEBRICKS2(x, h, w));
    }
    if (odd) {
        holder_->Append(PALACEBRICKS1(x, downto, w));
    }
}

void PalaceGenerator::MakeFakeFloor(int r, int x, int w, int upto) {
    int odd = upto & 1;
    if (!odd) {
        holder_->Append(PALACEBRICKS1(x, upto, w));
        upto += 1;
    }
    for(int h=upto; h<11; h+=2) {
        holder_->Append(PALACEBRICKS2(x, h, w));
    }
}

void PalaceGenerator::CleanNearElevator(int r, int x) {
    auto* command = holder_->mutable_command();
    auto it=command->begin();
    while(it < command->end()) {
        if (it->absx() >= x && it->absy() < 13) {
            int object = it->object() & 0xF0;
            if (object == 0x00 || object == 0xC0) {
                it = command->erase(it);
                continue;
            }
        }
        ++it;
    }
}

int PalaceGenerator::MakeElevator(int r, int x, int floor_val) {
    FloorCeiling fpos = fpos_[floor_val & 15];
    if (rooms_[r].has_up && rooms_[r].has_down) {
        if (fpos.floor > 10) {
            // Elevator step-up.
            MakeFakeFloor(r, x-2, 6, 10);
        }
        if (floor_val == 0 || floor_val >= 8) {
            // Bring the ceiling down around the elevator shaft.
            holder_->Append(SET_FLOOR(x-3, 0xe));
            holder_->Append(SET_FLOOR(x+5, floor_val));
        } else {
            // Create a fake ceiling around the elevator shaft.
            MakeFakeCeiling(r, x-3, 8, fpos.floor - 3);
        }
        // Elevator shaft (pit) floor to ceiling.
        holder_->Append(PIT(x, 0, 2));
        holder_->Append(ELEVATOR(x));
        rooms_[r].elevator = x;
    } else if (rooms_[r].has_up) {
        if (floor_val == 0 || floor_val >= 8) {
            // Bring the ceiling down around the elevator shaft.
            holder_->Append(SET_FLOOR(x-3, 0xe));
            holder_->Append(SET_FLOOR(x+5, floor_val));
        } else {
            // Create a fake ceiling around the elevator shaft.
            MakeFakeCeiling(r, x-3, 8, fpos.floor - 3);
        }
        // Elevator shaft (pit) floor to ceiling.

        // Elevator shaft (pit) floor to ceiling.
        // Fill in floor with palace bricks.
        holder_->Append(PIT(x, 0, 2));
        // Cannot bring floor up to fpos level as the elevator has a default
        // spawn position and the elevator pit must go at least to there.
        // MakeFakeFloor(r, x, 2, fpos.floor);
        holder_->Append(PALACEBRICKS2(x, 11, 2));
        holder_->Append(ELEVATOR(x));
        rooms_[r].elevator = x;
    } else if (rooms_[r].has_down) {
        if (fpos.floor > 10) {
            // Elevator step-up.
            MakeFakeFloor(r, x-2, 6, 10);
        }
        int h = fpos.floor - 5;
        if (h < 0) h = 0;
        holder_->Append(PALACEBRICKS2(x-3, h, 8));
        holder_->Append(COLUMN(x-4, fpos.ceiling+1, fpos.floor-fpos.ceiling-1));
        holder_->Append(COLUMN(x+5, fpos.ceiling+1, fpos.floor-fpos.ceiling-1));
        // Elevator shaft (pit) from floor-3 on down.
        holder_->Append(PIT(x, fpos.floor-3, 2));
        holder_->Append(ELEVATOR(x));
        rooms_[r].elevator = x;
    }
    return x+6;
}

void PalaceGenerator::PrepareEntranceRoom(int r) {
    int start = opt_.start_room();
    MapConnection connection(mapper_);

    holder_->Parse(palace_maps_[start+r]);
    connection.Parse(palace_maps_[start+r]);
    holder_->Clear({0, 4, false, false, false, 1, 0, 0, 0, 0});

    // Screen 0
    holder_->Append(PALACEBRICKS1(6, 10, 10));
    holder_->Append(PALACEBRICKS1(8, 9, 8));
    holder_->Append(STEELBRICKS(10, 8, 2));
    holder_->Append(IKSTATUE(10, 6));
    holder_->Append(PALACEBRICKS1(10, 0, 1));
    holder_->Append(PALACEBRICKS2(11, 0, 5));

    // Screen 1
    holder_->Append(PALACEBRICKS2(16, 0, 16));
    holder_->Append(PALACEBRICKS2(16, 9, 16));

    // Screen 2
    holder_->Append(PALACEBRICKS2(32, 0, 16));
    holder_->Append(PALACEBRICKS1(32, 10, 2));
    holder_->Append(PALACEBRICKS1(46, 10, 2));
    holder_->Append(PALACEBRICKS2(32, 11, 16));
    // 0x80 - magic "no ceiling" bit.
    MakeElevator(r, 39, 0x80);

    if (rooms_[r].has_right) {
        holder_->Append(PALACEBRICKS2(48, 0, 16));
        holder_->Append(PALACEBRICKS2(48, 9, 16));
    } else {
        holder_->Append(PALACEBRICKS2(48, 0, 9));
        holder_->Append(PALACEBRICKS1(48, 10, 14));
        holder_->Append(PALACEBRICKS1(48, 9, 12));
        holder_->Append(STEELBRICKS(56, 8, 2));
        holder_->Append(IKSTATUE(57, 6));
        holder_->Append(PALACEBRICKS1(57, 0, 1));
    }

    holder_->Append(COLUMN(15, 2, 7));
    holder_->Append(COLUMN(19, 2, 7));
    holder_->Append(COLUMN(23, 2, 7));
    holder_->Append(COLUMN(27, 2, 7));
    holder_->Append(COLUMN(31, 2, 7));
    holder_->Append(COLUMN(48, 2, 7));
    holder_->Append(COLUMN(52, 2, 7));

    // Connect the entrance room to the next rooms, or connect to outside.
    if (rooms_[r].has_left) {
        connection.set_left(rooms_[r].left, 3);
    } else {
        connection.set_left(63, 0);
    }
    if (rooms_[r].has_right) {
        connection.set_right(rooms_[r].right, 0);
    } else {
        connection.set_right(63, 0);
    }
    if (rooms_[r].has_up) {
        connection.set_up(rooms_[r].up, 0);
    } else {
        connection.set_up(63, 0);
    }
    if (rooms_[r].has_down) {
        connection.set_down(rooms_[r].down, 0);
    } else {
        connection.set_down(63, 0);
    }
    holder_->Save();
    connection.Save();
}

void PalaceGenerator::PrepareBossRoom(int r) {
    int start = opt_.start_room();
    MapConnection connection(mapper_);
    memset(&gen_, 0, sizeof(gen_));

    holder_->Parse(palace_maps_[start+r]);
    connection.Parse(palace_maps_[start+r]);
    gen_.floor_val = 0xe;
    holder_->Clear({0, 4, false, false, true, 0, gen_.floor_val, 0, 1, 0});

    // Check if we have a left exit.
    if (rooms_[r].has_left) {
    } else {
        // Start with a wall, then reset the floor to 0.
        holder_->set_floor(15);
        holder_->Append(SET_FLOOR(2, gen_.floor_val));
    }

    // Place elevators (if they exist)
    MakeElevator(r, 7, gen_.floor_val);

    // Boss fight room.
    gen_.xpos = 16;
    gen_.floor_val = 0;
    holder_->Append(SET_FLOOR(gen_.xpos, gen_.floor_val));
    holder_->Append(CURTAINS(gen_.xpos, 1, 16));
    holder_->Append(COLUMN(gen_.xpos+2, 1, 10));
    holder_->Append(COLUMN(gen_.xpos+13, 1, 10));

    // Low overhang
    gen_.xpos = 32;
    gen_.floor_val = 0xe;
    holder_->Append(SET_FLOOR(gen_.xpos, gen_.floor_val));

    // Statue room
    gen_.xpos = 48;
    gen_.floor_val = 0;
    holder_->Append(SET_FLOOR(gen_.xpos, gen_.floor_val));

    holder_->Append(STEELBRICKS(gen_.xpos+2, 10, 10));
    holder_->Append(STEELBRICKS(gen_.xpos+4, 9, 6));
    holder_->Append(CRYSTALRETURN(gen_.xpos+5, 4));

    // Right exit.
    gen_.xpos = 62;
    gen_.floor_val = 0xe;
    holder_->Append(SET_FLOOR(gen_.xpos, gen_.floor_val));
    holder_->Append(IKSTATUE(gen_.xpos, 9));

    connection.set_left(rooms_[r].left, 3);
    connection.set_right(63, 0);
    connection.set_up(rooms_[r].up, 0);
    connection.set_down(rooms_[r].down, 0);
    holder_->Save();
    connection.Save();
}

void PalaceGenerator::PrepareItemRoom(int r) {
    Direction side = rooms_[r].has_left ? RIGHT :
                     rooms_[r].has_right ? LEFT :
                     bit() ? LEFT : RIGHT;
    int start = opt_.start_room();
    MapConnection connection(mapper_);
    memset(&gen_, 0, sizeof(gen_));

    holder_->Parse(palace_maps_[start+r]);
    connection.Parse(palace_maps_[start+r]);
    gen_.floor_val = 0x0;
    holder_->Clear({0, 4, false, false, true, 0, gen_.floor_val, 0, 1, 0});

    // Check if we have a left exit or not.
    if (!rooms_[r].has_left) {
        // Start with a wall, then reset the floor to 0.
        holder_->set_floor(15);
        holder_->Append(SET_FLOOR(2, gen_.floor_val));
    }
    gen_.xpos += 2;

    int elevator_pos = 0;
    if (rooms_[r].has_up || rooms_[r].has_down) {
        elevator_pos = (side == LEFT) ? 55 : 7;
    }

    if (side == LEFT) {
        gen_.xpos += 2;
        holder_->Append(STEELBRICKS(gen_.xpos, 10, 6));
        holder_->Append(COLUMN(gen_.xpos+4, 8, 2));
        gen_.xpos += 8;
        gen_.floor_val = 0xe;
        holder_->Append(SET_FLOOR(gen_.xpos, gen_.floor_val));
        gen_.xpos += 4;
        MakeGallery(r, 32);
        MakeElevator(r, elevator_pos, gen_.floor_val);
    } else {
        MakeElevator(r, elevator_pos, gen_.floor_val);
        gen_.xpos = 16;
        MakeGallery(r, 32);
        holder_->Append(SET_FLOOR(gen_.xpos, 0xe));
        gen_.xpos += 4;
        holder_->Append(SET_FLOOR(gen_.xpos, gen_.floor_val));
        gen_.xpos += 2;
        holder_->Append(STEELBRICKS(gen_.xpos, 10, 6));
        holder_->Append(COLUMN(gen_.xpos+1, 8, 2));
        gen_.xpos += 8;
    }

    if (!rooms_[r].has_right) {
        // Put a wall at the last 2 columns of tho room.
        holder_->Append(SET_FLOOR(MAXX, 15));
    }

    connection.set_left(rooms_[r].left, 3);
    connection.set_right(rooms_[r].right, 0);
    connection.set_up(rooms_[r].up, 2);
    connection.set_down(rooms_[r].down, 2);
    holder_->Save();
    connection.Save();
}

void PalaceGenerator::PrepareRoom(int r) {
    if (r == 0) {
        PrepareEntranceRoom(r);
        return;
    } else if (rooms_[r].boss_room) {
        PrepareBossRoom(r);
        return;
    } else if (rooms_[r].item_room) {
        PrepareItemRoom(r);
        return;
    }
    int start = opt_.start_room();
    MapConnection connection(mapper_);
    memset(&gen_, 0, sizeof(gen_));

    holder_->Parse(palace_maps_[start+r]);
    connection.Parse(palace_maps_[start+r]);
    gen_.floor_val = integer(15);
    holder_->Clear({0, 4, false, false, true, 0, gen_.floor_val, 0, 1, 0});

    // Check if we have a left exit or not.
    if (rooms_[r].has_left) {
    } else {
        // Start with a wall, then reset the floor to 0.
        holder_->set_floor(15);
        holder_->Append(SET_FLOOR(2, gen_.floor_val));
    }
    gen_.xpos += 4;

    // If there is an elevator, decide where to put it.
    int elevator_pos = 0;
    if (rooms_[r].has_up || rooms_[r].has_down) {
        int allowed[] = {7, 23, 39, 55};
        elevator_pos = allowed[integer(4)];
    }

    while(gen_.xpos < MAXX) {
        if (elevator_pos && gen_.xpos >= elevator_pos - 3) {
            CleanNearElevator(r, elevator_pos - 3);
            gen_.xpos = MakeElevator(r, elevator_pos, gen_.floor_val);
            elevator_pos = 0;
        }
        int feature = integer(8);
        switch(feature) {
            case 0:
                MakeGallery(r); break;
            case 1:
                MakeLavaPit(r); break;
            case 3:
                MakeCubby(r); break;
            default:
                gen_.xpos += 2;
        }
    }

    if (rooms_[r].has_right) {
        // If there is a right exit, leave the right side open.
    } else {
        // Put a wall at the last 2 columns of tho room.
        holder_->Append(SET_FLOOR(MAXX, 15));
    }

    connection.set_left(rooms_[r].left, 3);
    connection.set_right(rooms_[r].right, 0);
    connection.set_up(rooms_[r].up, 2);
    connection.set_down(rooms_[r].down, 2);
    holder_->Save();
    connection.Save();

}

void PalaceGenerator::MakeGallery(int r, int w) {
    if (w < 0) {
        w = MAXX - gen_.xpos;
        if (w > 32) w = 32;
        w = 4 * integer(w/4);
        if (w == 0)
            return;
    }

    FloorCeiling opos = fpos_[gen_.floor_val & 15];
    FloorCeiling fpos;
    int new_floor;
    
    for(;;) {
        new_floor = integer(5) + (bit() ? 0 : 8);
        fpos = fpos_[new_floor];
    
        if (abs(fpos.ceiling - opos.floor - 1) < 2 ||
            abs(fpos.floor - opos.ceiling - 1) < 2)
            continue;
        if (abs(fpos.floor - opos.floor) > 3 && !opt_.jump_required())
            continue;
        if (abs(fpos.floor - opos.floor) > 4 && !opt_.fairy_required())
            continue;
        break;
    }

    gen_.floor_val = new_floor;
    holder_->Append(SET_FLOOR(gen_.xpos, gen_.floor_val));

    int delta = (fpos.ceiling > 4) ? 2 : 3;
    int decoration = integer(3);
    while(w > 0) {
        gen_.xpos += 2;
        switch(decoration) {
            case 0:
                holder_->Append(WINDOW(gen_.xpos, fpos.ceiling+3));
                break;
            case 1:
                holder_->Append(COLUMN(gen_.xpos, fpos.ceiling+1,
                                       fpos.floor-fpos.ceiling-1));
                break;
            case 2:
                holder_->Append(IKSTATUE(gen_.xpos, fpos.ceiling + delta));
                holder_->Append(COLUMN(gen_.xpos, fpos.ceiling + (delta+2),
                                       fpos.floor-fpos.ceiling - (delta+2)));
                break;
        }
        gen_.xpos += 2;
        w -= 4;
    }
}

void PalaceGenerator::MakeLavaPit(int r) {
    int w = MAXX - gen_.xpos;
    if (w < 6) return;
    FloorCeiling fpos = fpos_[gen_.floor_val & 15];
    if (fpos.floor > 9) return;

    if (w > 31) w = 31;
    w = integer(w);
    int bridge = 0;
    for(;;) {
        bridge = integer(4);
        if (w > 3 && !opt_.jump_required() && bridge == 0)
            continue;
        if (w > 5 && !opt_.fairy_required() && bridge == 0)
            continue;
        break;
    }
    holder_->Append(SET_FLOOR(gen_.xpos, (gen_.floor_val & 0x80)));
    while(w) {
        int chunk = (w > 16) ? 16 : w;;
        holder_->Append(LAVA(gen_.xpos, chunk));
        switch(bridge) {
            case 0:
                break;
            case 1:
                holder_->Append(CRUMBLEBRIDGE(gen_.xpos, fpos.floor, chunk));
                break;
            case 2:
                holder_->Append(BREAKBLOCK1(gen_.xpos, fpos.floor, chunk));
                break;
            case 3:
                holder_->Append(STEELBRICKS(gen_.xpos, fpos.floor, chunk));
                break;
        }
        w -= chunk;
        gen_.xpos += chunk;
    }
    holder_->Append(SET_FLOOR(gen_.xpos, gen_.floor_val));
    gen_.xpos += (w & 1) ? 1 : 2;;
}

void PalaceGenerator::MakeCubby(int r, int w) {
    if (w < 0) {
        w = MAXX - gen_.xpos;
        if (w > 16) w = 12;
        if (w < 4) return;
        w = 4 + 2 * integer(w/2);
    }
    FloorCeiling fpos = fpos_[gen_.floor_val & 15];
    int hole = 2 + 2 * integer(w/4);
    int a = (w - hole) / 2;
    
    if (fpos.floor < 9) {
        holder_->Append(SET_FLOOR(gen_.xpos, 0));
        holder_->Append(PALACEBRICKS1(gen_.xpos, 6, a));
        gen_.xpos += a;
        holder_->Append(BREAKBLOCK1(gen_.xpos, 6, hole));
        gen_.xpos += hole;
        holder_->Append(PALACEBRICKS1(gen_.xpos, 6, a));
        gen_.xpos += a;
        gen_.floor_val = 5;
        holder_->Append(SET_FLOOR(gen_.xpos-1, gen_.floor_val));
    } else {
        gen_.floor_val = 0;
        holder_->Append(SET_FLOOR(gen_.xpos, gen_.floor_val));
        holder_->Append(PALACEBRICKS2(gen_.xpos, 0, 1));
        holder_->Append(PALACEBRICKS2(gen_.xpos, 2, 1));
        holder_->Append(PALACEBRICKS2(gen_.xpos, 4, 1));
        holder_->Append(PALACEBRICKS1(gen_.xpos, 6, a));
//        if (fpos.floor < 9) 
//            holder_->Append(PALACEBRICKS2(gen_.xpos, 9, 1));
        gen_.xpos += a;
        holder_->Append(BREAKBLOCK1(gen_.xpos, 6, hole));
        gen_.xpos += hole;
        holder_->Append(PALACEBRICKS1(gen_.xpos, 6, a));
        gen_.xpos += a;
        holder_->Append(PALACEBRICKS2(gen_.xpos-1, 0, 1));
        holder_->Append(PALACEBRICKS2(gen_.xpos-1, 2, 1));
        holder_->Append(PALACEBRICKS2(gen_.xpos-1, 4, 1));
    }
}

void PalaceGenerator::FixElevatorConnections() {
    int start = opt_.start_room();
    MapConnection connection(mapper_);

    for(const auto& r : rooms_) {
        connection.Parse(palace_maps_[start+r.room]);
        if (r.elevator) {
            if (r.has_up) {
                connection.set_up(r.up, rooms_[r.up].elevator / 16);
            }
            if (r.has_down) {
                connection.set_down(r.down, rooms_[r.down].elevator / 16);
            }
        }
        connection.Save();
    }
}

void PalaceGenerator::SimplePrint() {
    for(int y=0; y<opt_.grid_height(); y++) {
        for(int x=0; x<opt_.grid_width(); x++) {
            if (map_[y][x].room == -1) {
                printf("XXXXXXX");
            } else {
                char buf[8] = "[     ]";
                if (map_[y][x].has_left) buf[0] = '<';
                if (map_[y][x].has_right) buf[6] = '>';
                //if (map_[y][x].room == 0) buf[0] = 'E';
                if (map_[y][x].has_up) buf[3] = '^';
                if (map_[y][x].has_down) buf[3] = 'v';
                if (map_[y][x].has_up && map_[y][x].has_down) buf[3] = '|';

                int r = map_[y][x].room;
                buf[1] = '0' + (r/10);
                buf[2] = '0' + (r%10);
                printf("%s", buf);
            }
        }
        printf("\n");
    }
    
    printf("Dead end rooms:\n");
    for(const auto& r : rooms_) {
        if (r.dead_end) {
            printf("Room %02d: dist=%d %s%s\n", r.room, r.distance,
                   r.has_left ? "" : "L", r.has_right ? "" : "R");
        }
    }
}


}  // namespace
