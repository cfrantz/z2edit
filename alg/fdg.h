#ifndef Z2UTIL_ALG_FDG_H
#define Z2UTIL_ALG_FDG_H

#include <cmath>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "alg/vec2.h"

namespace fdg {

struct Bias {
    double x, y;
    static const Bias None;
    static const Bias Horizontal;
    static const Bias Vertical;
};

struct Spring {
    int32_t destid;
    double k;
    Bias bias;
    // color and width don't properly belong here, but its just easier
    // to put them here.
    uint32_t color;
    double width;
    int srcloc;
    int dstloc;
};

class Node {
  public:
    explicit Node(int32_t id, const Vec2& position)
      : id_(id), start_pos_(position), pos_(position),
        vel_(0, 0), acc_(0, 0), force_(0, 0),
        mass_(1.0), charge_(1.0), friction_(0.01), pause_(false) {}

    explicit Node(const Vec2& position) : Node(0, position) {}
    explicit Node() : Node(0, Vec2(0, 0)) {}

    inline int32_t id() const { return id_; }
    inline void set_id(int32_t id) { id_ = id; }

    inline const Vec2& pos() const { return pos_; }
    inline void set_pos(const Vec2& pos) { pos_ = pos; }
    inline const Vec2& force() const { return force_; }

    inline double mass() const { return mass_; }
    inline void set_mass(double mass) { mass_ = mass; }
    inline double charge() const { return charge_; }
    inline void set_charge(double charge) { charge_ = charge; }
    inline double friction() const { return friction_; }
    inline void set_friction(double friction) { friction_ = friction; }

    inline bool pause() const { return pause_; }
    inline void set_pause(bool pause) { pause_ = pause; }

    inline const std::vector<Spring>& connection() const { return connection_; }
    inline std::vector<Spring>* mutable_connection() { return &connection_; }

    void Print();
    void ComputeForces(
            const std::map<int32_t, std::unique_ptr<Node>>& nodes);
    void ApplyForces(double deltaT);
  private:
    int32_t id_;
    Vec2 start_pos_;
    Vec2 pos_;
    Vec2 vel_;
    Vec2 acc_;
    Vec2 force_;
    double mass_;
    double charge_;
    double friction_;
    bool pause_;
    std::vector<Spring> connection_;
};

class Graph {
  public:
    explicit Graph() {}

    inline Node* AddNode(Node* node) {
        nodes_[node->id()].reset(node);
        return node;
    }
    inline Node* AddNode(int32_t id, const Vec2& position) {
        return AddNode(new Node(id, position));
    }
    inline const std::map<int32_t, std::unique_ptr<Node>>& nodes() const {
        return nodes_;
    }
    inline std::map<int32_t, std::unique_ptr<Node>>* mutable_nodes() {
        return &nodes_;
    }
    inline Node* find(int32_t id) {
        const auto& it = nodes_.find(id);
        if (it == nodes_.end())
            return nullptr;
        return it->second.get();
    }

    void Print();
    void Compute(double deltaT);
    void Clear() { nodes_.clear(); }
  private:
    std::map<int32_t, std::unique_ptr<Node>> nodes_;
};

}  // namespace fdg
#endif // Z2UTIL_ALG_FDG_H
