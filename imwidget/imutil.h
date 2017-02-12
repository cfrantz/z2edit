#ifndef Z2UTIL_IMWIDGET_IMUTIL_H
#define Z2UTIL_IMWIDGET_IMUTIL_H
#include "imgui.h"

inline ImVec2 operator*(const ImVec2& a, float b) {
    return ImVec2(a.x * b, a.y * b);
}

inline ImVec2& operator*=(ImVec2& a, float b) {
    a.x *= b;  a.y *= b;
    return a;
}

inline ImVec2 operator/(const ImVec2& a, float b) {
    return ImVec2(a.x / b, a.y / b);
}

inline ImVec2& operator/=(ImVec2& a, float b) {
    a.x /= b;  a.y /= b;
    return a;
}

inline ImVec2 operator+(const ImVec2& a, const ImVec2& b) {
    return ImVec2(a.x + b.x, a.y + b.y);
}

inline ImVec2& operator+=(ImVec2& a, ImVec2& b) {
    a.x += b.x;  a.y += b.y;
    return a;
}

inline ImVec2 operator-(const ImVec2& a, const ImVec2& b) {
    return ImVec2(a.x - b.x, a.y - b.y);
}

inline ImVec2& operator-=(ImVec2& a, ImVec2& b) {
    a.x -= b.x;  a.y -= b.y;
    return a;
}

template<typename T>
inline T Clamp(T v, T mn, T mx) {
    return (v < mn) ? mn : (v > mx) ? mx : v;
}

template<typename T>
inline void Clamp(T* v, T mn, T mx) {
    *v = Clamp(*v, mn, mx);
}

void TextOutlined(const ImVec4& col, const char* fmt, ...);
void TextOutlinedV(const ImVec4& col, const char* fmt, va_list args);


#endif // Z2UTIL_IMWIDGET_IMUTIL_H
