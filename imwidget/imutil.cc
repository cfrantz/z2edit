#include "imwidget/imutil.h"
#include <cstdio>
#include <cstdarg>

void TextOutlinedV(const ImVec4& col, const char* fmt, va_list args) {
    char buf[1024];

    vsnprintf(buf, sizeof(buf), fmt, args);
    ImVec2 cursor = ImGui::GetCursorPos();
    for(int y=-1; y<2; y++) {
        for(int x=-1; x<2; x++) {
            ImGui::SetCursorPos(cursor + ImVec2(x, y));
            ImGui::TextColored(ImColor(0xFF000000), "%s", buf);
        }
    }
    ImGui::SetCursorPos(cursor);
    ImGui::TextColored(col, "%s", buf);
}

void TextOutlined(const ImVec4& col, const char* fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    TextOutlinedV(col, fmt, ap);
    va_end(ap);
}
