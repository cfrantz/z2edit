#include "alg/fdg.h"

namespace fdg {

const Bias Bias::None =       {0, 0};
const Bias Bias::Horizontal = {0.1, 1.0};
const Bias Bias::Vertical   = {1.0, 0.1};

void Node::ComputeForces(
        const std::map<int32_t, std::unique_ptr<Node>>& nodes) {

    // The charges on all the nodes repel each other
    for(const auto& other : nodes) {
        if (other.first == id_)
            continue;

        // Coulomb's law: F = k_e * c1 * c2 / r^2
        // For us, k_e = 1.0
        Vec2 diff = pos_ - other.second->pos_;
        double r = diff.length(); // < 0.01 ? 0.01 : diff.length();
        double f = (charge_ * other.second->charge_) / (r * r);
        force_ += diff.unit() * f;
    }

    // The springs attract connected nodes to each other
    for(const auto& spring : connection_) {
        const auto& other = nodes.find(spring.destid);
        if (other == nodes.end()) {
            // If we can't find a spring's destination node, skip.
            continue;
        }
        // If the node is connected to itself, skip
        if (other->first == id_)
            continue;

        // Hooke's law: F = kx
        // Since we apply the spring's force to both nodes, divide by 2.
        Vec2 diff = pos_ - other->second->pos_;
        double x = diff.length();
        double f = spring.k * x / 2.0;
        Vec2 force = diff.unit() * f;

        // Apply any bias to the force
        if (spring.bias.x != 0.0 || spring.bias.y != 0.0) {
            force.x *= spring.bias.x;
            force.y *= spring.bias.y;
        }
        // Apply the force.  The other node feels the opposite force
        force_ -= force;
        other->second->force_ += force;
    }
}

void Node::Print() {
    printf("node {\n");
    printf("  id: %d\n", id_);
    printf("  x: %f y: %f\n", pos_.x, pos_.y);
    for (const auto& c : connection_) {
        printf("  connection { destid: %d k: %f bias: {x:%f y:%f} color: %#08x width: %f }\n",
                c.destid, c.k, c.bias.x, c.bias.y, c.color, c.width);
    }
    printf("}\n");
}

void Node::ApplyForces(double deltaT) {
    // Newton's 2nd Law: F = ma
    acc_ = force_ / mass_;
    force_ = Vec2(0, 0);

    // Equations of motion
    vel_ = vel_ * (1.0 - friction_) + acc_ * deltaT;
    Vec2 delta = vel_ * deltaT;
    if (delta.length() < 1e-6)
        return;

    pos_ += delta;
}

void Graph::Print() {
    for(const auto& n : nodes_)
        n.second->Print();
}

void Graph::Compute(double deltaT) {
    for(const auto& n : nodes_)
        n.second->ComputeForces(nodes_);
    for(const auto& n : nodes_)
        n.second->ApplyForces(deltaT);
}

}  // namepsace fdg
