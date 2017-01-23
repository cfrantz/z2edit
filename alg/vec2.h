#ifndef FDG_VEC2_H
#define FDG_VEC2_H
#include <cmath>

// FIXME(cfrantz): make imgui optional
#include "imgui.h"

class Vec2 {
  public:
    double x, y;

    Vec2(): x(0), y(0) {}
    explicit Vec2(double x_, double y_): x(x_), y(y_) {}
    Vec2(const Vec2& other): x(other.x), y(other.y) {}

    static const Vec2 origin;

    inline double length() const {
        return std::sqrt(x*x + y*y);
    }
    inline Vec2 unit() const {
        return *this / length();
    }
    inline Vec2 flip() const {
        return Vec2(-y, x);
    }
    inline Vec2 rotate(double radians) const {
        return Vec2(x * std::cos(radians) - y * std::sin(radians),
                    x * std::sin(radians) + y * std::cos(radians));
    }

    inline Vec2 operator-() const {
        return Vec2(-x, -y);
    }
    inline Vec2 operator+(const Vec2& rhs) const {
        return Vec2(x+rhs.x, y+rhs.y);
    }
    inline Vec2 operator-(const Vec2& rhs) const {
        return Vec2(x-rhs.x, y-rhs.y);
    }

    inline Vec2 operator*(double scalar) const {
        return Vec2(x*scalar, y*scalar);
    }
    inline Vec2 operator/(double scalar) const {
        return Vec2(x/scalar, y/scalar);
    }

    inline Vec2& operator+=(const Vec2& rhs) {
        x += rhs.x; y += rhs.y;
        return *this;
    }
    inline Vec2& operator-=(const Vec2& rhs) {
        x -= rhs.x; y -= rhs.y;
        return *this;
    }
    inline Vec2& operator*=(double scalar) {
        x *= scalar; y *= scalar;
        return *this;
    }
    inline Vec2& operator/=(double scalar) {
        x /= scalar; y /= scalar;
        return *this;
    }

    // Make dealing with ImGui's ImVec2 easier
    Vec2(const ImVec2& other) : x(other.x), y(other.y) {}
    inline ImVec2 im() const {
        return ImVec2(x, y);
    }
    inline Vec2 operator+(const ImVec2& rhs) const {
        return Vec2(x+rhs.x, y+rhs.y);
    }
    inline Vec2 operator-(const ImVec2& rhs) const {
        return Vec2(x-rhs.x, y-rhs.y);
    }
    inline Vec2& operator+=(const ImVec2& rhs) {
        x += rhs.x; y += rhs.y;
        return *this;
    }
    inline Vec2& operator-=(const ImVec2& rhs) {
        x -= rhs.x; y -= rhs.y;
        return *this;
    }
    inline operator ImVec2() const {
        return ImVec2(x, y);
    }
};

#endif // FDG_VEC2_H
