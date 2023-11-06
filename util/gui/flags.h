#ifndef EMPTY_PROJECT_UTIL_GUI_FLAGS_H
#define EMPTY_PROJECT_UTIL_GUI_FLAGS_H

#include "imgui.h"
#include "proto/gui_extension.pb.h"
namespace gui {

ImGuiColorEditFlags GetColorEditFlags(const proto::ColorEditFlags& flags);
ImGuiInputTextFlags GetInputTextFlags(const proto::InputTextFlags& flags);
ImGuiPopupFlags GetPopupFlags(const proto::PopupFlags& flags);
ImGuiSliderFlags GetSliderFlags(const proto::SliderFlags& flags);
ImGuiTableFlags GetTableFlags(const proto::TableFlags& flags);
ImGuiWindowFlags GetWindowFlags(const proto::WindowFlags& flags);
int PushStyleColor(const proto::Col& v);
int PushStyleVar(const proto::StyleVar& v);

}  // namespace gui

#endif  // EMPTY_PROJECT_UTIL_GUI_FLAGS_H
