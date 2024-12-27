
#include <pybind11/pybind11.h>
#include <pybind11/functional.h>
#include <pybind11/stl.h>
#include <limits>

//#define IMGUI_HAS_DOCK       1
#include "imgui.h"
#include "imgui_internal.h"
namespace py = pybind11;

template<typename T>
void template_ImVector(py::module &module, const char* name)
{
    py::class_< ImVector<T> >(module, name)
        .def_property_readonly_static("stride", [](py::object)
        {
            return sizeof(T);
        })
        .def_property_readonly("data", [](const ImVector<T>& self)
        {
            return long((void*)self.Data);
        })
        .def("__len__", [](const ImVector<T>& self)
        {
            return self.size();
        })
        .def("__iter__", [](const ImVector<T>& self)
        {
            return py::make_iterator(self.begin(), self.end());
        })
        .def("__getitem__", [](const ImVector<T>& self, size_t i)
        {
            if ((int)i >= self.size()) throw py::index_error();
            return self[i];
        })
        ;
}

PYBIND11_MODULE(gui, gui)
{
    /***
     * For some reason, just the existence of the binding causes
     * some badness during program shutdown.
     *
    py::class_<ImGuiContext>(gui, "Context");
    ***/
    template_ImVector<char>(gui, "Vector_char");
    template_ImVector<float>(gui, "Vector_float");
    template_ImVector<unsigned char>(gui, "Vector_unsignedchar");
    template_ImVector<unsigned short>(gui, "Vector_unsignedshort");
    template_ImVector<ImDrawCmd>(gui, "Vector_DrawCmd");
    template_ImVector<ImDrawVert>(gui, "Vector_DrawVert");
    template_ImVector<ImFontGlyph>(gui, "Vector_FontGlyph");

    py::class_<ImVec2> Vec2(gui, "Vec2");
    Vec2.def_readwrite("x", &ImVec2::x);
    Vec2.def_readwrite("y", &ImVec2::y);
    Vec2.def(py::init<>());
    Vec2.def(py::init<float, float>()
    , py::arg("_x")
    , py::arg("_y")
    );
    py::class_<ImVec4> Vec4(gui, "Vec4");
    Vec4.def_readwrite("x", &ImVec4::x);
    Vec4.def_readwrite("y", &ImVec4::y);
    Vec4.def_readwrite("z", &ImVec4::z);
    Vec4.def_readwrite("w", &ImVec4::w);
    Vec4.def(py::init<>());
    Vec4.def(py::init<float, float, float, float>()
    , py::arg("_x")
    , py::arg("_y")
    , py::arg("_z")
    , py::arg("_w")
    );
    gui.def("get_io", &ImGui::GetIO
    , py::return_value_policy::reference);
    gui.def("get_style", &ImGui::GetStyle
    , py::return_value_policy::reference);
    gui.def("new_frame", &ImGui::NewFrame
    , py::return_value_policy::automatic_reference);
    gui.def("end_frame", &ImGui::EndFrame
    , py::return_value_policy::automatic_reference);
    gui.def("render", &ImGui::Render
    , py::return_value_policy::automatic_reference);
    gui.def("get_draw_data", &ImGui::GetDrawData
    , py::return_value_policy::automatic_reference);
    gui.def("show_demo_window", [](bool * p_open)
    {
        ImGui::ShowDemoWindow(p_open);
        return p_open;
    }
    , py::arg("p_open") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("show_metrics_window", [](bool * p_open)
    {
        ImGui::ShowMetricsWindow(p_open);
        return p_open;
    }
    , py::arg("p_open") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("show_debug_log_window", [](bool * p_open)
    {
        ImGui::ShowDebugLogWindow(p_open);
        return p_open;
    }
    , py::arg("p_open") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("show_id_stack_tool_window", [](bool * p_open)
    {
        ImGui::ShowIDStackToolWindow(p_open);
        return p_open;
    }
    , py::arg("p_open") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("show_about_window", [](bool * p_open)
    {
        ImGui::ShowAboutWindow(p_open);
        return p_open;
    }
    , py::arg("p_open") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("show_style_editor", &ImGui::ShowStyleEditor
    , py::arg("ref") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("show_style_selector", &ImGui::ShowStyleSelector
    , py::arg("label")
    , py::return_value_policy::automatic_reference);
    gui.def("show_font_selector", &ImGui::ShowFontSelector
    , py::arg("label")
    , py::return_value_policy::automatic_reference);
    gui.def("show_user_guide", &ImGui::ShowUserGuide
    , py::return_value_policy::automatic_reference);
    gui.def("get_version", &ImGui::GetVersion
    , py::return_value_policy::automatic_reference);
    gui.def("style_colors_dark", &ImGui::StyleColorsDark
    , py::arg("dst") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("style_colors_light", &ImGui::StyleColorsLight
    , py::arg("dst") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("style_colors_classic", &ImGui::StyleColorsClassic
    , py::arg("dst") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("begin", [](const char * name, bool * p_open, ImGuiWindowFlags flags)
    {
        auto ret = ImGui::Begin(name, p_open, flags);
        return std::make_tuple(ret, p_open);
    }
    , py::arg("name")
    , py::arg("p_open") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("end", &ImGui::End
    , py::return_value_policy::automatic_reference);
    gui.def("begin_child", py::overload_cast<const char *, const ImVec2 &, ImGuiChildFlags, ImGuiWindowFlags>(&ImGui::BeginChild)
    , py::arg("str_id")
    , py::arg("size") = ImVec2(0,0)
    , py::arg("child_flags") = 0
    , py::arg("window_flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("begin_child", py::overload_cast<ImGuiID, const ImVec2 &, ImGuiChildFlags, ImGuiWindowFlags>(&ImGui::BeginChild)
    , py::arg("id")
    , py::arg("size") = ImVec2(0,0)
    , py::arg("child_flags") = 0
    , py::arg("window_flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("end_child", &ImGui::EndChild
    , py::return_value_policy::automatic_reference);
    gui.def("is_window_appearing", &ImGui::IsWindowAppearing
    , py::return_value_policy::automatic_reference);
    gui.def("is_window_collapsed", &ImGui::IsWindowCollapsed
    , py::return_value_policy::automatic_reference);
    gui.def("is_window_focused", &ImGui::IsWindowFocused
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("is_window_hovered", &ImGui::IsWindowHovered
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("get_window_draw_list", &ImGui::GetWindowDrawList
    , py::return_value_policy::automatic_reference);
    gui.def("get_window_dpi_scale", &ImGui::GetWindowDpiScale
    , py::return_value_policy::automatic_reference);
    gui.def("get_window_pos", &ImGui::GetWindowPos
    , py::return_value_policy::automatic_reference);
    gui.def("get_window_size", &ImGui::GetWindowSize
    , py::return_value_policy::automatic_reference);
    gui.def("get_window_width", &ImGui::GetWindowWidth
    , py::return_value_policy::automatic_reference);
    gui.def("get_window_height", &ImGui::GetWindowHeight
    , py::return_value_policy::automatic_reference);
    gui.def("get_window_viewport", &ImGui::GetWindowViewport
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_window_pos", &ImGui::SetNextWindowPos
    , py::arg("pos")
    , py::arg("cond") = 0
    , py::arg("pivot") = ImVec2(0,0)
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_window_size", &ImGui::SetNextWindowSize
    , py::arg("size")
    , py::arg("cond") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_window_content_size", &ImGui::SetNextWindowContentSize
    , py::arg("size")
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_window_collapsed", &ImGui::SetNextWindowCollapsed
    , py::arg("collapsed")
    , py::arg("cond") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_window_focus", &ImGui::SetNextWindowFocus
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_window_scroll", &ImGui::SetNextWindowScroll
    , py::arg("scroll")
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_window_bg_alpha", &ImGui::SetNextWindowBgAlpha
    , py::arg("alpha")
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_window_viewport", &ImGui::SetNextWindowViewport
    , py::arg("viewport_id")
    , py::return_value_policy::automatic_reference);
    gui.def("set_window_pos", py::overload_cast<const ImVec2 &, ImGuiCond>(&ImGui::SetWindowPos)
    , py::arg("pos")
    , py::arg("cond") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_window_size", py::overload_cast<const ImVec2 &, ImGuiCond>(&ImGui::SetWindowSize)
    , py::arg("size")
    , py::arg("cond") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_window_collapsed", py::overload_cast<bool, ImGuiCond>(&ImGui::SetWindowCollapsed)
    , py::arg("collapsed")
    , py::arg("cond") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_window_focus", py::overload_cast<>(&ImGui::SetWindowFocus)
    , py::return_value_policy::automatic_reference);
    gui.def("set_window_font_scale", &ImGui::SetWindowFontScale
    , py::arg("scale")
    , py::return_value_policy::automatic_reference);
    gui.def("set_window_pos", py::overload_cast<const char *, const ImVec2 &, ImGuiCond>(&ImGui::SetWindowPos)
    , py::arg("name")
    , py::arg("pos")
    , py::arg("cond") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_window_size", py::overload_cast<const char *, const ImVec2 &, ImGuiCond>(&ImGui::SetWindowSize)
    , py::arg("name")
    , py::arg("size")
    , py::arg("cond") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_window_collapsed", py::overload_cast<const char *, bool, ImGuiCond>(&ImGui::SetWindowCollapsed)
    , py::arg("name")
    , py::arg("collapsed")
    , py::arg("cond") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_window_focus", py::overload_cast<const char *>(&ImGui::SetWindowFocus)
    , py::arg("name")
    , py::return_value_policy::automatic_reference);
    gui.def("get_scroll_x", &ImGui::GetScrollX
    , py::return_value_policy::automatic_reference);
    gui.def("get_scroll_y", &ImGui::GetScrollY
    , py::return_value_policy::automatic_reference);
    gui.def("set_scroll_x", py::overload_cast<float>(&ImGui::SetScrollX)
    , py::arg("scroll_x")
    , py::return_value_policy::automatic_reference);
    gui.def("set_scroll_y", py::overload_cast<float>(&ImGui::SetScrollY)
    , py::arg("scroll_y")
    , py::return_value_policy::automatic_reference);
    gui.def("get_scroll_max_x", &ImGui::GetScrollMaxX
    , py::return_value_policy::automatic_reference);
    gui.def("get_scroll_max_y", &ImGui::GetScrollMaxY
    , py::return_value_policy::automatic_reference);
    gui.def("set_scroll_here_x", &ImGui::SetScrollHereX
    , py::arg("center_x_ratio") = 0.5f
    , py::return_value_policy::automatic_reference);
    gui.def("set_scroll_here_y", &ImGui::SetScrollHereY
    , py::arg("center_y_ratio") = 0.5f
    , py::return_value_policy::automatic_reference);
    gui.def("set_scroll_from_pos_x", py::overload_cast<float, float>(&ImGui::SetScrollFromPosX)
    , py::arg("local_x")
    , py::arg("center_x_ratio") = 0.5f
    , py::return_value_policy::automatic_reference);
    gui.def("set_scroll_from_pos_y", py::overload_cast<float, float>(&ImGui::SetScrollFromPosY)
    , py::arg("local_y")
    , py::arg("center_y_ratio") = 0.5f
    , py::return_value_policy::automatic_reference);
    gui.def("push_font", &ImGui::PushFont
    , py::arg("font")
    , py::return_value_policy::automatic_reference);
    gui.def("pop_font", &ImGui::PopFont
    , py::return_value_policy::automatic_reference);
    gui.def("push_style_color", py::overload_cast<ImGuiCol, ImU32>(&ImGui::PushStyleColor)
    , py::arg("idx")
    , py::arg("col")
    , py::return_value_policy::automatic_reference);
    gui.def("push_style_color", py::overload_cast<ImGuiCol, const ImVec4 &>(&ImGui::PushStyleColor)
    , py::arg("idx")
    , py::arg("col")
    , py::return_value_policy::automatic_reference);
    gui.def("pop_style_color", &ImGui::PopStyleColor
    , py::arg("count") = 1
    , py::return_value_policy::automatic_reference);
    gui.def("push_style_var", py::overload_cast<ImGuiStyleVar, float>(&ImGui::PushStyleVar)
    , py::arg("idx")
    , py::arg("val")
    , py::return_value_policy::automatic_reference);
    gui.def("push_style_var", py::overload_cast<ImGuiStyleVar, const ImVec2 &>(&ImGui::PushStyleVar)
    , py::arg("idx")
    , py::arg("val")
    , py::return_value_policy::automatic_reference);
    gui.def("push_style_var_x", &ImGui::PushStyleVarX
    , py::arg("idx")
    , py::arg("val_x")
    , py::return_value_policy::automatic_reference);
    gui.def("push_style_var_y", &ImGui::PushStyleVarY
    , py::arg("idx")
    , py::arg("val_y")
    , py::return_value_policy::automatic_reference);
    gui.def("pop_style_var", &ImGui::PopStyleVar
    , py::arg("count") = 1
    , py::return_value_policy::automatic_reference);
    gui.def("push_item_flag", &ImGui::PushItemFlag
    , py::arg("option")
    , py::arg("enabled")
    , py::return_value_policy::automatic_reference);
    gui.def("pop_item_flag", &ImGui::PopItemFlag
    , py::return_value_policy::automatic_reference);
    gui.def("push_item_width", &ImGui::PushItemWidth
    , py::arg("item_width")
    , py::return_value_policy::automatic_reference);
    gui.def("pop_item_width", &ImGui::PopItemWidth
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_item_width", &ImGui::SetNextItemWidth
    , py::arg("item_width")
    , py::return_value_policy::automatic_reference);
    gui.def("calc_item_width", &ImGui::CalcItemWidth
    , py::return_value_policy::automatic_reference);
    gui.def("push_text_wrap_pos", &ImGui::PushTextWrapPos
    , py::arg("wrap_local_pos_x") = 0.0f
    , py::return_value_policy::automatic_reference);
    gui.def("pop_text_wrap_pos", &ImGui::PopTextWrapPos
    , py::return_value_policy::automatic_reference);
    gui.def("get_font", &ImGui::GetFont
    , py::return_value_policy::automatic_reference);
    gui.def("get_font_size", &ImGui::GetFontSize
    , py::return_value_policy::automatic_reference);
    gui.def("get_font_tex_uv_white_pixel", &ImGui::GetFontTexUvWhitePixel
    , py::return_value_policy::automatic_reference);
    gui.def("get_color_u32", py::overload_cast<ImGuiCol, float>(&ImGui::GetColorU32)
    , py::arg("idx")
    , py::arg("alpha_mul") = 1.0f
    , py::return_value_policy::automatic_reference);
    gui.def("get_color_u32", py::overload_cast<const ImVec4 &>(&ImGui::GetColorU32)
    , py::arg("col")
    , py::return_value_policy::automatic_reference);
    gui.def("get_color_u32", py::overload_cast<ImU32, float>(&ImGui::GetColorU32)
    , py::arg("col")
    , py::arg("alpha_mul") = 1.0f
    , py::return_value_policy::automatic_reference);
    gui.def("get_style_color_vec4", &ImGui::GetStyleColorVec4
    , py::arg("idx")
    , py::return_value_policy::reference);
    gui.def("get_cursor_screen_pos", &ImGui::GetCursorScreenPos
    , py::return_value_policy::automatic_reference);
    gui.def("set_cursor_screen_pos", &ImGui::SetCursorScreenPos
    , py::arg("pos")
    , py::return_value_policy::automatic_reference);
    gui.def("get_content_region_avail", &ImGui::GetContentRegionAvail
    , py::return_value_policy::automatic_reference);
    gui.def("get_cursor_pos", &ImGui::GetCursorPos
    , py::return_value_policy::automatic_reference);
    gui.def("get_cursor_pos_x", &ImGui::GetCursorPosX
    , py::return_value_policy::automatic_reference);
    gui.def("get_cursor_pos_y", &ImGui::GetCursorPosY
    , py::return_value_policy::automatic_reference);
    gui.def("set_cursor_pos", &ImGui::SetCursorPos
    , py::arg("local_pos")
    , py::return_value_policy::automatic_reference);
    gui.def("set_cursor_pos_x", &ImGui::SetCursorPosX
    , py::arg("local_x")
    , py::return_value_policy::automatic_reference);
    gui.def("set_cursor_pos_y", &ImGui::SetCursorPosY
    , py::arg("local_y")
    , py::return_value_policy::automatic_reference);
    gui.def("get_cursor_start_pos", &ImGui::GetCursorStartPos
    , py::return_value_policy::automatic_reference);
    gui.def("separator", &ImGui::Separator
    , py::return_value_policy::automatic_reference);
    gui.def("same_line", &ImGui::SameLine
    , py::arg("offset_from_start_x") = 0.0f
    , py::arg("spacing") = -1.0f
    , py::return_value_policy::automatic_reference);
    gui.def("new_line", &ImGui::NewLine
    , py::return_value_policy::automatic_reference);
    gui.def("spacing", &ImGui::Spacing
    , py::return_value_policy::automatic_reference);
    gui.def("dummy", &ImGui::Dummy
    , py::arg("size")
    , py::return_value_policy::automatic_reference);
    gui.def("indent", &ImGui::Indent
    , py::arg("indent_w") = 0.0f
    , py::return_value_policy::automatic_reference);
    gui.def("unindent", &ImGui::Unindent
    , py::arg("indent_w") = 0.0f
    , py::return_value_policy::automatic_reference);
    gui.def("begin_group", &ImGui::BeginGroup
    , py::return_value_policy::automatic_reference);
    gui.def("end_group", &ImGui::EndGroup
    , py::return_value_policy::automatic_reference);
    gui.def("align_text_to_frame_padding", &ImGui::AlignTextToFramePadding
    , py::return_value_policy::automatic_reference);
    gui.def("get_text_line_height", &ImGui::GetTextLineHeight
    , py::return_value_policy::automatic_reference);
    gui.def("get_text_line_height_with_spacing", &ImGui::GetTextLineHeightWithSpacing
    , py::return_value_policy::automatic_reference);
    gui.def("get_frame_height", &ImGui::GetFrameHeight
    , py::return_value_policy::automatic_reference);
    gui.def("get_frame_height_with_spacing", &ImGui::GetFrameHeightWithSpacing
    , py::return_value_policy::automatic_reference);
    gui.def("push_id", py::overload_cast<const char *>(&ImGui::PushID)
    , py::arg("str_id")
    , py::return_value_policy::automatic_reference);
    gui.def("push_id", py::overload_cast<const char *, const char *>(&ImGui::PushID)
    , py::arg("str_id_begin")
    , py::arg("str_id_end")
    , py::return_value_policy::automatic_reference);
    gui.def("push_id", py::overload_cast<const void *>(&ImGui::PushID)
    , py::arg("ptr_id")
    , py::return_value_policy::automatic_reference);
    gui.def("push_id", py::overload_cast<int>(&ImGui::PushID)
    , py::arg("int_id")
    , py::return_value_policy::automatic_reference);
    gui.def("pop_id", &ImGui::PopID
    , py::return_value_policy::automatic_reference);
    gui.def("get_id", py::overload_cast<const char *>(&ImGui::GetID)
    , py::arg("str_id")
    , py::return_value_policy::automatic_reference);
    gui.def("get_id", py::overload_cast<const char *, const char *>(&ImGui::GetID)
    , py::arg("str_id_begin")
    , py::arg("str_id_end")
    , py::return_value_policy::automatic_reference);
    gui.def("get_id", py::overload_cast<const void *>(&ImGui::GetID)
    , py::arg("ptr_id")
    , py::return_value_policy::automatic_reference);
    gui.def("get_id", py::overload_cast<int>(&ImGui::GetID)
    , py::arg("int_id")
    , py::return_value_policy::automatic_reference);
    gui.def("text_unformatted", &ImGui::TextUnformatted
    , py::arg("text")
    , py::arg("text_end") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("text", [](const char * fmt)
    {
        ImGui::Text(fmt);
        return ;
    }
    , py::arg("fmt")
    , py::return_value_policy::automatic_reference);
    gui.def("text_colored", [](const ImVec4 & col, const char * fmt)
    {
        ImGui::TextColored(col, fmt);
        return ;
    }
    , py::arg("col")
    , py::arg("fmt")
    , py::return_value_policy::automatic_reference);
    gui.def("text_disabled", [](const char * fmt)
    {
        ImGui::TextDisabled(fmt);
        return ;
    }
    , py::arg("fmt")
    , py::return_value_policy::automatic_reference);
    gui.def("text_wrapped", [](const char * fmt)
    {
        ImGui::TextWrapped(fmt);
        return ;
    }
    , py::arg("fmt")
    , py::return_value_policy::automatic_reference);
    gui.def("label_text", [](const char * label, const char * fmt)
    {
        ImGui::LabelText(label, fmt);
        return ;
    }
    , py::arg("label")
    , py::arg("fmt")
    , py::return_value_policy::automatic_reference);
    gui.def("bullet_text", [](const char * fmt)
    {
        ImGui::BulletText(fmt);
        return ;
    }
    , py::arg("fmt")
    , py::return_value_policy::automatic_reference);
    gui.def("separator_text", &ImGui::SeparatorText
    , py::arg("label")
    , py::return_value_policy::automatic_reference);
    gui.def("button", &ImGui::Button
    , py::arg("label")
    , py::arg("size") = ImVec2(0,0)
    , py::return_value_policy::automatic_reference);
    gui.def("small_button", &ImGui::SmallButton
    , py::arg("label")
    , py::return_value_policy::automatic_reference);
    gui.def("invisible_button", &ImGui::InvisibleButton
    , py::arg("str_id")
    , py::arg("size")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("arrow_button", &ImGui::ArrowButton
    , py::arg("str_id")
    , py::arg("dir")
    , py::return_value_policy::automatic_reference);
    gui.def("checkbox", [](const char * label, bool * v)
    {
        auto ret = ImGui::Checkbox(label, v);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::return_value_policy::automatic_reference);
    gui.def("checkbox_flags", [](const char * label, int * flags, int flags_value)
    {
        auto ret = ImGui::CheckboxFlags(label, flags, flags_value);
        return std::make_tuple(ret, flags);
    }
    , py::arg("label")
    , py::arg("flags")
    , py::arg("flags_value")
    , py::return_value_policy::automatic_reference);
    gui.def("checkbox_flags", [](const char * label, unsigned int * flags, unsigned int flags_value)
    {
        auto ret = ImGui::CheckboxFlags(label, flags, flags_value);
        return std::make_tuple(ret, flags);
    }
    , py::arg("label")
    , py::arg("flags")
    , py::arg("flags_value")
    , py::return_value_policy::automatic_reference);
    gui.def("radio_button", py::overload_cast<const char *, bool>(&ImGui::RadioButton)
    , py::arg("label")
    , py::arg("active")
    , py::return_value_policy::automatic_reference);
    gui.def("radio_button", [](const char * label, int * v, int v_button)
    {
        auto ret = ImGui::RadioButton(label, v, v_button);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_button")
    , py::return_value_policy::automatic_reference);
    gui.def("progress_bar", &ImGui::ProgressBar
    , py::arg("fraction")
    , py::arg("size_arg") = ImVec2(-FLT_MIN,0)
    , py::arg("overlay") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("bullet", &ImGui::Bullet
    , py::return_value_policy::automatic_reference);
    gui.def("text_link", &ImGui::TextLink
    , py::arg("label")
    , py::return_value_policy::automatic_reference);
    gui.def("text_link_open_url", &ImGui::TextLinkOpenURL
    , py::arg("label")
    , py::arg("url") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("image", &ImGui::Image
    , py::arg("user_texture_id")
    , py::arg("image_size")
    , py::arg("uv0") = ImVec2(0,0)
    , py::arg("uv1") = ImVec2(1,1)
    , py::arg("tint_col") = ImVec4(1,1,1,1)
    , py::arg("border_col") = ImVec4(0,0,0,0)
    , py::return_value_policy::automatic_reference);
    gui.def("image_button", py::overload_cast<const char *, ImTextureID, const ImVec2 &, const ImVec2 &, const ImVec2 &, const ImVec4 &, const ImVec4 &>(&ImGui::ImageButton)
    , py::arg("str_id")
    , py::arg("user_texture_id")
    , py::arg("image_size")
    , py::arg("uv0") = ImVec2(0,0)
    , py::arg("uv1") = ImVec2(1,1)
    , py::arg("bg_col") = ImVec4(0,0,0,0)
    , py::arg("tint_col") = ImVec4(1,1,1,1)
    , py::return_value_policy::automatic_reference);
    gui.def("begin_combo", &ImGui::BeginCombo
    , py::arg("label")
    , py::arg("preview_value")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("end_combo", &ImGui::EndCombo
    , py::return_value_policy::automatic_reference);
    gui.def("drag_float", [](const char * label, float * v, float v_speed, float v_min, float v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::DragFloat(label, v, v_speed, v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_speed") = 1.0f
    , py::arg("v_min") = 0.0f
    , py::arg("v_max") = 0.0f
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("drag_float2", [](const char * label, std::array<float, 2>& v, float v_speed, float v_min, float v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::DragFloat2(label, &v[0], v_speed, v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_speed") = 1.0f
    , py::arg("v_min") = 0.0f
    , py::arg("v_max") = 0.0f
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("drag_float3", [](const char * label, std::array<float, 3>& v, float v_speed, float v_min, float v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::DragFloat3(label, &v[0], v_speed, v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_speed") = 1.0f
    , py::arg("v_min") = 0.0f
    , py::arg("v_max") = 0.0f
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("drag_float4", [](const char * label, std::array<float, 4>& v, float v_speed, float v_min, float v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::DragFloat4(label, &v[0], v_speed, v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_speed") = 1.0f
    , py::arg("v_min") = 0.0f
    , py::arg("v_max") = 0.0f
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("drag_float_range2", [](const char * label, float * v_current_min, float * v_current_max, float v_speed, float v_min, float v_max, const char * format, const char * format_max, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::DragFloatRange2(label, v_current_min, v_current_max, v_speed, v_min, v_max, format, format_max, flags);
        return std::make_tuple(ret, v_current_min, v_current_max);
    }
    , py::arg("label")
    , py::arg("v_current_min")
    , py::arg("v_current_max")
    , py::arg("v_speed") = 1.0f
    , py::arg("v_min") = 0.0f
    , py::arg("v_max") = 0.0f
    , py::arg("format") = nullptr
    , py::arg("format_max") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("drag_int", [](const char * label, int * v, float v_speed, int v_min, int v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::DragInt(label, v, v_speed, v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_speed") = 1.0f
    , py::arg("v_min") = 0
    , py::arg("v_max") = 0
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("drag_int2", [](const char * label, std::array<int, 2>& v, float v_speed, int v_min, int v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::DragInt2(label, &v[0], v_speed, v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_speed") = 1.0f
    , py::arg("v_min") = 0
    , py::arg("v_max") = 0
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("drag_int3", [](const char * label, std::array<int, 3>& v, float v_speed, int v_min, int v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::DragInt3(label, &v[0], v_speed, v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_speed") = 1.0f
    , py::arg("v_min") = 0
    , py::arg("v_max") = 0
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("drag_int4", [](const char * label, std::array<int, 4>& v, float v_speed, int v_min, int v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::DragInt4(label, &v[0], v_speed, v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_speed") = 1.0f
    , py::arg("v_min") = 0
    , py::arg("v_max") = 0
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("drag_int_range2", [](const char * label, int * v_current_min, int * v_current_max, float v_speed, int v_min, int v_max, const char * format, const char * format_max, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::DragIntRange2(label, v_current_min, v_current_max, v_speed, v_min, v_max, format, format_max, flags);
        return std::make_tuple(ret, v_current_min, v_current_max);
    }
    , py::arg("label")
    , py::arg("v_current_min")
    , py::arg("v_current_max")
    , py::arg("v_speed") = 1.0f
    , py::arg("v_min") = 0
    , py::arg("v_max") = 0
    , py::arg("format") = nullptr
    , py::arg("format_max") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("drag_scalar", &ImGui::DragScalar
    , py::arg("label")
    , py::arg("data_type")
    , py::arg("p_data")
    , py::arg("v_speed") = 1.0f
    , py::arg("p_min") = nullptr
    , py::arg("p_max") = nullptr
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("drag_scalar_n", &ImGui::DragScalarN
    , py::arg("label")
    , py::arg("data_type")
    , py::arg("p_data")
    , py::arg("components")
    , py::arg("v_speed") = 1.0f
    , py::arg("p_min") = nullptr
    , py::arg("p_max") = nullptr
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("slider_float", [](const char * label, float * v, float v_min, float v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::SliderFloat(label, v, v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_min")
    , py::arg("v_max")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("slider_float2", [](const char * label, std::array<float, 2>& v, float v_min, float v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::SliderFloat2(label, &v[0], v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_min")
    , py::arg("v_max")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("slider_float3", [](const char * label, std::array<float, 3>& v, float v_min, float v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::SliderFloat3(label, &v[0], v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_min")
    , py::arg("v_max")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("slider_float4", [](const char * label, std::array<float, 4>& v, float v_min, float v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::SliderFloat4(label, &v[0], v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_min")
    , py::arg("v_max")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("slider_angle", [](const char * label, float * v_rad, float v_degrees_min, float v_degrees_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::SliderAngle(label, v_rad, v_degrees_min, v_degrees_max, format, flags);
        return std::make_tuple(ret, v_rad);
    }
    , py::arg("label")
    , py::arg("v_rad")
    , py::arg("v_degrees_min") = -360.0f
    , py::arg("v_degrees_max") = +360.0f
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("slider_int", [](const char * label, int * v, int v_min, int v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::SliderInt(label, v, v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_min")
    , py::arg("v_max")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("slider_int2", [](const char * label, std::array<int, 2>& v, int v_min, int v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::SliderInt2(label, &v[0], v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_min")
    , py::arg("v_max")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("slider_int3", [](const char * label, std::array<int, 3>& v, int v_min, int v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::SliderInt3(label, &v[0], v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_min")
    , py::arg("v_max")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("slider_int4", [](const char * label, std::array<int, 4>& v, int v_min, int v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::SliderInt4(label, &v[0], v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("v_min")
    , py::arg("v_max")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("slider_scalar", &ImGui::SliderScalar
    , py::arg("label")
    , py::arg("data_type")
    , py::arg("p_data")
    , py::arg("p_min")
    , py::arg("p_max")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("slider_scalar_n", &ImGui::SliderScalarN
    , py::arg("label")
    , py::arg("data_type")
    , py::arg("p_data")
    , py::arg("components")
    , py::arg("p_min")
    , py::arg("p_max")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("v_slider_float", [](const char * label, const ImVec2 & size, float * v, float v_min, float v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::VSliderFloat(label, size, v, v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("size")
    , py::arg("v")
    , py::arg("v_min")
    , py::arg("v_max")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("v_slider_int", [](const char * label, const ImVec2 & size, int * v, int v_min, int v_max, const char * format, ImGuiSliderFlags flags)
    {
        auto ret = ImGui::VSliderInt(label, size, v, v_min, v_max, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("size")
    , py::arg("v")
    , py::arg("v_min")
    , py::arg("v_max")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("v_slider_scalar", &ImGui::VSliderScalar
    , py::arg("label")
    , py::arg("size")
    , py::arg("data_type")
    , py::arg("p_data")
    , py::arg("p_min")
    , py::arg("p_max")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("input_float", [](const char * label, float * v, float step, float step_fast, const char * format, ImGuiInputTextFlags flags)
    {
        auto ret = ImGui::InputFloat(label, v, step, step_fast, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("step") = 0.0f
    , py::arg("step_fast") = 0.0f
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("input_float2", [](const char * label, std::array<float, 2>& v, const char * format, ImGuiInputTextFlags flags)
    {
        auto ret = ImGui::InputFloat2(label, &v[0], format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("input_float3", [](const char * label, std::array<float, 3>& v, const char * format, ImGuiInputTextFlags flags)
    {
        auto ret = ImGui::InputFloat3(label, &v[0], format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("input_float4", [](const char * label, std::array<float, 4>& v, const char * format, ImGuiInputTextFlags flags)
    {
        auto ret = ImGui::InputFloat4(label, &v[0], format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("input_int", [](const char * label, int * v, int step, int step_fast, ImGuiInputTextFlags flags)
    {
        auto ret = ImGui::InputInt(label, v, step, step_fast, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("step") = 1
    , py::arg("step_fast") = 100
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("input_int2", [](const char * label, std::array<int, 2>& v, ImGuiInputTextFlags flags)
    {
        auto ret = ImGui::InputInt2(label, &v[0], flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("input_int3", [](const char * label, std::array<int, 3>& v, ImGuiInputTextFlags flags)
    {
        auto ret = ImGui::InputInt3(label, &v[0], flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("input_int4", [](const char * label, std::array<int, 4>& v, ImGuiInputTextFlags flags)
    {
        auto ret = ImGui::InputInt4(label, &v[0], flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("input_double", [](const char * label, double * v, double step, double step_fast, const char * format, ImGuiInputTextFlags flags)
    {
        auto ret = ImGui::InputDouble(label, v, step, step_fast, format, flags);
        return std::make_tuple(ret, v);
    }
    , py::arg("label")
    , py::arg("v")
    , py::arg("step") = 0.0
    , py::arg("step_fast") = 0.0
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("input_scalar", &ImGui::InputScalar
    , py::arg("label")
    , py::arg("data_type")
    , py::arg("p_data")
    , py::arg("p_step") = nullptr
    , py::arg("p_step_fast") = nullptr
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("input_scalar_n", &ImGui::InputScalarN
    , py::arg("label")
    , py::arg("data_type")
    , py::arg("p_data")
    , py::arg("components")
    , py::arg("p_step") = nullptr
    , py::arg("p_step_fast") = nullptr
    , py::arg("format") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("color_edit3", [](const char * label, std::array<float, 3>& col, ImGuiColorEditFlags flags)
    {
        auto ret = ImGui::ColorEdit3(label, &col[0], flags);
        return std::make_tuple(ret, col);
    }
    , py::arg("label")
    , py::arg("col")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("color_edit4", [](const char * label, std::array<float, 4>& col, ImGuiColorEditFlags flags)
    {
        auto ret = ImGui::ColorEdit4(label, &col[0], flags);
        return std::make_tuple(ret, col);
    }
    , py::arg("label")
    , py::arg("col")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("color_picker3", [](const char * label, std::array<float, 3>& col, ImGuiColorEditFlags flags)
    {
        auto ret = ImGui::ColorPicker3(label, &col[0], flags);
        return std::make_tuple(ret, col);
    }
    , py::arg("label")
    , py::arg("col")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("color_picker4", [](const char * label, std::array<float, 4>& col, ImGuiColorEditFlags flags, const float * ref_col)
    {
        auto ret = ImGui::ColorPicker4(label, &col[0], flags, ref_col);
        return std::make_tuple(ret, col);
    }
    , py::arg("label")
    , py::arg("col")
    , py::arg("flags") = 0
    , py::arg("ref_col") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("color_button", &ImGui::ColorButton
    , py::arg("desc_id")
    , py::arg("col")
    , py::arg("flags") = 0
    , py::arg("size") = ImVec2(0,0)
    , py::return_value_policy::automatic_reference);
    gui.def("set_color_edit_options", &ImGui::SetColorEditOptions
    , py::arg("flags")
    , py::return_value_policy::automatic_reference);
    gui.def("tree_node", py::overload_cast<const char *>(&ImGui::TreeNode)
    , py::arg("label")
    , py::return_value_policy::automatic_reference);
    gui.def("tree_node", [](const char * str_id, const char * fmt)
    {
        auto ret = ImGui::TreeNode(str_id, fmt);
        return ret;
    }
    , py::arg("str_id")
    , py::arg("fmt")
    , py::return_value_policy::automatic_reference);
    gui.def("tree_node", [](const void * ptr_id, const char * fmt)
    {
        auto ret = ImGui::TreeNode(ptr_id, fmt);
        return ret;
    }
    , py::arg("ptr_id")
    , py::arg("fmt")
    , py::return_value_policy::automatic_reference);
    gui.def("tree_node_ex", py::overload_cast<const char *, ImGuiTreeNodeFlags>(&ImGui::TreeNodeEx)
    , py::arg("label")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("tree_node_ex", [](const char * str_id, ImGuiTreeNodeFlags flags, const char * fmt)
    {
        auto ret = ImGui::TreeNodeEx(str_id, flags, fmt);
        return ret;
    }
    , py::arg("str_id")
    , py::arg("flags")
    , py::arg("fmt")
    , py::return_value_policy::automatic_reference);
    gui.def("tree_node_ex", [](const void * ptr_id, ImGuiTreeNodeFlags flags, const char * fmt)
    {
        auto ret = ImGui::TreeNodeEx(ptr_id, flags, fmt);
        return ret;
    }
    , py::arg("ptr_id")
    , py::arg("flags")
    , py::arg("fmt")
    , py::return_value_policy::automatic_reference);
    gui.def("tree_push", py::overload_cast<const char *>(&ImGui::TreePush)
    , py::arg("str_id")
    , py::return_value_policy::automatic_reference);
    gui.def("tree_push", py::overload_cast<const void *>(&ImGui::TreePush)
    , py::arg("ptr_id")
    , py::return_value_policy::automatic_reference);
    gui.def("tree_pop", &ImGui::TreePop
    , py::return_value_policy::automatic_reference);
    gui.def("get_tree_node_to_label_spacing", &ImGui::GetTreeNodeToLabelSpacing
    , py::return_value_policy::automatic_reference);
    gui.def("collapsing_header", py::overload_cast<const char *, ImGuiTreeNodeFlags>(&ImGui::CollapsingHeader)
    , py::arg("label")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("collapsing_header", [](const char * label, bool * p_visible, ImGuiTreeNodeFlags flags)
    {
        auto ret = ImGui::CollapsingHeader(label, p_visible, flags);
        return std::make_tuple(ret, p_visible);
    }
    , py::arg("label")
    , py::arg("p_visible")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_item_open", &ImGui::SetNextItemOpen
    , py::arg("is_open")
    , py::arg("cond") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_item_storage_id", &ImGui::SetNextItemStorageID
    , py::arg("storage_id")
    , py::return_value_policy::automatic_reference);
    gui.def("selectable", py::overload_cast<const char *, bool, ImGuiSelectableFlags, const ImVec2 &>(&ImGui::Selectable)
    , py::arg("label")
    , py::arg("selected") = false
    , py::arg("flags") = 0
    , py::arg("size") = ImVec2(0,0)
    , py::return_value_policy::automatic_reference);
    gui.def("selectable", [](const char * label, bool * p_selected, ImGuiSelectableFlags flags, const ImVec2 & size)
    {
        auto ret = ImGui::Selectable(label, p_selected, flags, size);
        return std::make_tuple(ret, p_selected);
    }
    , py::arg("label")
    , py::arg("p_selected")
    , py::arg("flags") = 0
    , py::arg("size") = ImVec2(0,0)
    , py::return_value_policy::automatic_reference);
    gui.def("begin_multi_select", &ImGui::BeginMultiSelect
    , py::arg("flags")
    , py::arg("selection_size") = -1
    , py::arg("items_count") = -1
    , py::return_value_policy::automatic_reference);
    gui.def("end_multi_select", &ImGui::EndMultiSelect
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_item_selection_user_data", &ImGui::SetNextItemSelectionUserData
    , py::arg("selection_user_data")
    , py::return_value_policy::automatic_reference);
    gui.def("is_item_toggled_selection", &ImGui::IsItemToggledSelection
    , py::return_value_policy::automatic_reference);
    gui.def("begin_list_box", &ImGui::BeginListBox
    , py::arg("label")
    , py::arg("size") = ImVec2(0,0)
    , py::return_value_policy::automatic_reference);
    gui.def("end_list_box", &ImGui::EndListBox
    , py::return_value_policy::automatic_reference);
    gui.def("value", py::overload_cast<const char *, bool>(&ImGui::Value)
    , py::arg("prefix")
    , py::arg("b")
    , py::return_value_policy::automatic_reference);
    gui.def("value", py::overload_cast<const char *, int>(&ImGui::Value)
    , py::arg("prefix")
    , py::arg("v")
    , py::return_value_policy::automatic_reference);
    gui.def("value", py::overload_cast<const char *, unsigned int>(&ImGui::Value)
    , py::arg("prefix")
    , py::arg("v")
    , py::return_value_policy::automatic_reference);
    gui.def("value", py::overload_cast<const char *, float, const char *>(&ImGui::Value)
    , py::arg("prefix")
    , py::arg("v")
    , py::arg("float_format") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("begin_menu_bar", &ImGui::BeginMenuBar
    , py::return_value_policy::automatic_reference);
    gui.def("end_menu_bar", &ImGui::EndMenuBar
    , py::return_value_policy::automatic_reference);
    gui.def("begin_main_menu_bar", &ImGui::BeginMainMenuBar
    , py::return_value_policy::automatic_reference);
    gui.def("end_main_menu_bar", &ImGui::EndMainMenuBar
    , py::return_value_policy::automatic_reference);
    gui.def("begin_menu", &ImGui::BeginMenu
    , py::arg("label")
    , py::arg("enabled") = true
    , py::return_value_policy::automatic_reference);
    gui.def("end_menu", &ImGui::EndMenu
    , py::return_value_policy::automatic_reference);
    gui.def("menu_item", py::overload_cast<const char *, const char *, bool, bool>(&ImGui::MenuItem)
    , py::arg("label")
    , py::arg("shortcut") = nullptr
    , py::arg("selected") = false
    , py::arg("enabled") = true
    , py::return_value_policy::automatic_reference);
    gui.def("menu_item", [](const char * label, const char * shortcut, bool * p_selected, bool enabled)
    {
        auto ret = ImGui::MenuItem(label, shortcut, p_selected, enabled);
        return std::make_tuple(ret, p_selected);
    }
    , py::arg("label")
    , py::arg("shortcut")
    , py::arg("p_selected")
    , py::arg("enabled") = true
    , py::return_value_policy::automatic_reference);
    gui.def("begin_tooltip", &ImGui::BeginTooltip
    , py::return_value_policy::automatic_reference);
    gui.def("end_tooltip", &ImGui::EndTooltip
    , py::return_value_policy::automatic_reference);
    gui.def("set_tooltip", [](const char * fmt)
    {
        ImGui::SetTooltip(fmt);
        return ;
    }
    , py::arg("fmt")
    , py::return_value_policy::automatic_reference);
    gui.def("begin_item_tooltip", &ImGui::BeginItemTooltip
    , py::return_value_policy::automatic_reference);
    gui.def("set_item_tooltip", [](const char * fmt)
    {
        ImGui::SetItemTooltip(fmt);
        return ;
    }
    , py::arg("fmt")
    , py::return_value_policy::automatic_reference);
    gui.def("begin_popup", &ImGui::BeginPopup
    , py::arg("str_id")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("begin_popup_modal", [](const char * name, bool * p_open, ImGuiWindowFlags flags)
    {
        auto ret = ImGui::BeginPopupModal(name, p_open, flags);
        return std::make_tuple(ret, p_open);
    }
    , py::arg("name")
    , py::arg("p_open") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("end_popup", &ImGui::EndPopup
    , py::return_value_policy::automatic_reference);
    gui.def("open_popup", py::overload_cast<const char *, ImGuiPopupFlags>(&ImGui::OpenPopup)
    , py::arg("str_id")
    , py::arg("popup_flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("open_popup", py::overload_cast<ImGuiID, ImGuiPopupFlags>(&ImGui::OpenPopup)
    , py::arg("id")
    , py::arg("popup_flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("open_popup_on_item_click", &ImGui::OpenPopupOnItemClick
    , py::arg("str_id") = nullptr
    , py::arg("popup_flags") = 1
    , py::return_value_policy::automatic_reference);
    gui.def("close_current_popup", &ImGui::CloseCurrentPopup
    , py::return_value_policy::automatic_reference);
    gui.def("begin_popup_context_item", &ImGui::BeginPopupContextItem
    , py::arg("str_id") = nullptr
    , py::arg("popup_flags") = 1
    , py::return_value_policy::automatic_reference);
    gui.def("begin_popup_context_window", &ImGui::BeginPopupContextWindow
    , py::arg("str_id") = nullptr
    , py::arg("popup_flags") = 1
    , py::return_value_policy::automatic_reference);
    gui.def("begin_popup_context_void", &ImGui::BeginPopupContextVoid
    , py::arg("str_id") = nullptr
    , py::arg("popup_flags") = 1
    , py::return_value_policy::automatic_reference);
    gui.def("is_popup_open", py::overload_cast<const char *, ImGuiPopupFlags>(&ImGui::IsPopupOpen)
    , py::arg("str_id")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("begin_table", &ImGui::BeginTable
    , py::arg("str_id")
    , py::arg("columns")
    , py::arg("flags") = 0
    , py::arg("outer_size") = ImVec2(0.0f,0.0f)
    , py::arg("inner_width") = 0.0f
    , py::return_value_policy::automatic_reference);
    gui.def("end_table", &ImGui::EndTable
    , py::return_value_policy::automatic_reference);
    gui.def("table_next_row", &ImGui::TableNextRow
    , py::arg("row_flags") = 0
    , py::arg("min_row_height") = 0.0f
    , py::return_value_policy::automatic_reference);
    gui.def("table_next_column", &ImGui::TableNextColumn
    , py::return_value_policy::automatic_reference);
    gui.def("table_set_column_index", &ImGui::TableSetColumnIndex
    , py::arg("column_n")
    , py::return_value_policy::automatic_reference);
    gui.def("table_setup_column", &ImGui::TableSetupColumn
    , py::arg("label")
    , py::arg("flags") = 0
    , py::arg("init_width_or_weight") = 0.0f
    , py::arg("user_id") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("table_setup_scroll_freeze", &ImGui::TableSetupScrollFreeze
    , py::arg("cols")
    , py::arg("rows")
    , py::return_value_policy::automatic_reference);
    gui.def("table_header", &ImGui::TableHeader
    , py::arg("label")
    , py::return_value_policy::automatic_reference);
    gui.def("table_headers_row", &ImGui::TableHeadersRow
    , py::return_value_policy::automatic_reference);
    gui.def("table_angled_headers_row", &ImGui::TableAngledHeadersRow
    , py::return_value_policy::automatic_reference);
    gui.def("table_get_sort_specs", &ImGui::TableGetSortSpecs
    , py::return_value_policy::automatic_reference);
    gui.def("table_get_column_count", &ImGui::TableGetColumnCount
    , py::return_value_policy::automatic_reference);
    gui.def("table_get_column_index", &ImGui::TableGetColumnIndex
    , py::return_value_policy::automatic_reference);
    gui.def("table_get_row_index", &ImGui::TableGetRowIndex
    , py::return_value_policy::automatic_reference);
    gui.def("table_get_column_name", py::overload_cast<int>(&ImGui::TableGetColumnName)
    , py::arg("column_n") = -1
    , py::return_value_policy::automatic_reference);
    gui.def("table_get_column_flags", &ImGui::TableGetColumnFlags
    , py::arg("column_n") = -1
    , py::return_value_policy::automatic_reference);
    gui.def("table_set_column_enabled", &ImGui::TableSetColumnEnabled
    , py::arg("column_n")
    , py::arg("v")
    , py::return_value_policy::automatic_reference);
    gui.def("table_get_hovered_column", &ImGui::TableGetHoveredColumn
    , py::return_value_policy::automatic_reference);
    gui.def("table_set_bg_color", &ImGui::TableSetBgColor
    , py::arg("target")
    , py::arg("color")
    , py::arg("column_n") = -1
    , py::return_value_policy::automatic_reference);
    gui.def("columns", &ImGui::Columns
    , py::arg("count") = 1
    , py::arg("id") = nullptr
    , py::arg("borders") = true
    , py::return_value_policy::automatic_reference);
    gui.def("next_column", &ImGui::NextColumn
    , py::return_value_policy::automatic_reference);
    gui.def("get_column_index", &ImGui::GetColumnIndex
    , py::return_value_policy::automatic_reference);
    gui.def("get_column_width", &ImGui::GetColumnWidth
    , py::arg("column_index") = -1
    , py::return_value_policy::automatic_reference);
    gui.def("set_column_width", &ImGui::SetColumnWidth
    , py::arg("column_index")
    , py::arg("width")
    , py::return_value_policy::automatic_reference);
    gui.def("get_column_offset", &ImGui::GetColumnOffset
    , py::arg("column_index") = -1
    , py::return_value_policy::automatic_reference);
    gui.def("set_column_offset", &ImGui::SetColumnOffset
    , py::arg("column_index")
    , py::arg("offset_x")
    , py::return_value_policy::automatic_reference);
    gui.def("get_columns_count", &ImGui::GetColumnsCount
    , py::return_value_policy::automatic_reference);
    gui.def("begin_tab_bar", &ImGui::BeginTabBar
    , py::arg("str_id")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("end_tab_bar", &ImGui::EndTabBar
    , py::return_value_policy::automatic_reference);
    gui.def("begin_tab_item", [](const char * label, bool * p_open, ImGuiTabItemFlags flags)
    {
        auto ret = ImGui::BeginTabItem(label, p_open, flags);
        return std::make_tuple(ret, p_open);
    }
    , py::arg("label")
    , py::arg("p_open") = nullptr
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("end_tab_item", &ImGui::EndTabItem
    , py::return_value_policy::automatic_reference);
    gui.def("tab_item_button", &ImGui::TabItemButton
    , py::arg("label")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_tab_item_closed", &ImGui::SetTabItemClosed
    , py::arg("tab_or_docked_window_label")
    , py::return_value_policy::automatic_reference);
    gui.def("dock_space", &ImGui::DockSpace
    , py::arg("dockspace_id")
    , py::arg("size") = ImVec2(0,0)
    , py::arg("flags") = 0
    , py::arg("window_class") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("dock_space_over_viewport", &ImGui::DockSpaceOverViewport
    , py::arg("dockspace_id") = 0
    , py::arg("viewport") = nullptr
    , py::arg("flags") = 0
    , py::arg("window_class") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_window_dock_id", &ImGui::SetNextWindowDockID
    , py::arg("dock_id")
    , py::arg("cond") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_window_class", &ImGui::SetNextWindowClass
    , py::arg("window_class")
    , py::return_value_policy::automatic_reference);
    gui.def("get_window_dock_id", &ImGui::GetWindowDockID
    , py::return_value_policy::automatic_reference);
    gui.def("is_window_docked", &ImGui::IsWindowDocked
    , py::return_value_policy::automatic_reference);
    gui.def("log_to_tty", &ImGui::LogToTTY
    , py::arg("auto_open_depth") = -1
    , py::return_value_policy::automatic_reference);
    gui.def("log_to_file", &ImGui::LogToFile
    , py::arg("auto_open_depth") = -1
    , py::arg("filename") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("log_to_clipboard", &ImGui::LogToClipboard
    , py::arg("auto_open_depth") = -1
    , py::return_value_policy::automatic_reference);
    gui.def("log_finish", &ImGui::LogFinish
    , py::return_value_policy::automatic_reference);
    gui.def("log_buttons", &ImGui::LogButtons
    , py::return_value_policy::automatic_reference);
    gui.def("log_text", [](const char * fmt)
    {
        ImGui::LogText(fmt);
        return ;
    }
    , py::arg("fmt")
    , py::return_value_policy::automatic_reference);
    gui.def("begin_drag_drop_source", &ImGui::BeginDragDropSource
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_drag_drop_payload", &ImGui::SetDragDropPayload
    , py::arg("type")
    , py::arg("data")
    , py::arg("sz")
    , py::arg("cond") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("end_drag_drop_source", &ImGui::EndDragDropSource
    , py::return_value_policy::automatic_reference);
    gui.def("begin_drag_drop_target", &ImGui::BeginDragDropTarget
    , py::return_value_policy::automatic_reference);
    gui.def("accept_drag_drop_payload", &ImGui::AcceptDragDropPayload
    , py::arg("type")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("end_drag_drop_target", &ImGui::EndDragDropTarget
    , py::return_value_policy::automatic_reference);
    gui.def("get_drag_drop_payload", &ImGui::GetDragDropPayload
    , py::return_value_policy::automatic_reference);
    gui.def("begin_disabled", &ImGui::BeginDisabled
    , py::arg("disabled") = true
    , py::return_value_policy::automatic_reference);
    gui.def("end_disabled", &ImGui::EndDisabled
    , py::return_value_policy::automatic_reference);
    gui.def("push_clip_rect", &ImGui::PushClipRect
    , py::arg("clip_rect_min")
    , py::arg("clip_rect_max")
    , py::arg("intersect_with_current_clip_rect")
    , py::return_value_policy::automatic_reference);
    gui.def("pop_clip_rect", &ImGui::PopClipRect
    , py::return_value_policy::automatic_reference);
    gui.def("set_item_default_focus", &ImGui::SetItemDefaultFocus
    , py::return_value_policy::automatic_reference);
    gui.def("set_keyboard_focus_here", &ImGui::SetKeyboardFocusHere
    , py::arg("offset") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_item_allow_overlap", &ImGui::SetNextItemAllowOverlap
    , py::return_value_policy::automatic_reference);
    gui.def("is_item_hovered", &ImGui::IsItemHovered
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("is_item_active", &ImGui::IsItemActive
    , py::return_value_policy::automatic_reference);
    gui.def("is_item_focused", &ImGui::IsItemFocused
    , py::return_value_policy::automatic_reference);
    gui.def("is_item_clicked", &ImGui::IsItemClicked
    , py::arg("mouse_button") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("is_item_visible", &ImGui::IsItemVisible
    , py::return_value_policy::automatic_reference);
    gui.def("is_item_edited", &ImGui::IsItemEdited
    , py::return_value_policy::automatic_reference);
    gui.def("is_item_activated", &ImGui::IsItemActivated
    , py::return_value_policy::automatic_reference);
    gui.def("is_item_deactivated", &ImGui::IsItemDeactivated
    , py::return_value_policy::automatic_reference);
    gui.def("is_item_deactivated_after_edit", &ImGui::IsItemDeactivatedAfterEdit
    , py::return_value_policy::automatic_reference);
    gui.def("is_item_toggled_open", &ImGui::IsItemToggledOpen
    , py::return_value_policy::automatic_reference);
    gui.def("is_any_item_hovered", &ImGui::IsAnyItemHovered
    , py::return_value_policy::automatic_reference);
    gui.def("is_any_item_active", &ImGui::IsAnyItemActive
    , py::return_value_policy::automatic_reference);
    gui.def("is_any_item_focused", &ImGui::IsAnyItemFocused
    , py::return_value_policy::automatic_reference);
    gui.def("get_item_id", &ImGui::GetItemID
    , py::return_value_policy::automatic_reference);
    gui.def("get_item_rect_min", &ImGui::GetItemRectMin
    , py::return_value_policy::automatic_reference);
    gui.def("get_item_rect_max", &ImGui::GetItemRectMax
    , py::return_value_policy::automatic_reference);
    gui.def("get_item_rect_size", &ImGui::GetItemRectSize
    , py::return_value_policy::automatic_reference);
    gui.def("get_main_viewport", &ImGui::GetMainViewport
    , py::return_value_policy::automatic_reference);
    gui.def("get_background_draw_list", py::overload_cast<ImGuiViewport *>(&ImGui::GetBackgroundDrawList)
    , py::arg("viewport") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("get_foreground_draw_list", py::overload_cast<ImGuiViewport *>(&ImGui::GetForegroundDrawList)
    , py::arg("viewport") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("is_rect_visible", py::overload_cast<const ImVec2 &>(&ImGui::IsRectVisible)
    , py::arg("size")
    , py::return_value_policy::automatic_reference);
    gui.def("is_rect_visible", py::overload_cast<const ImVec2 &, const ImVec2 &>(&ImGui::IsRectVisible)
    , py::arg("rect_min")
    , py::arg("rect_max")
    , py::return_value_policy::automatic_reference);
    gui.def("get_time", &ImGui::GetTime
    , py::return_value_policy::automatic_reference);
    gui.def("get_frame_count", &ImGui::GetFrameCount
    , py::return_value_policy::automatic_reference);
    gui.def("get_draw_list_shared_data", &ImGui::GetDrawListSharedData
    , py::return_value_policy::automatic_reference);
    gui.def("get_style_color_name", &ImGui::GetStyleColorName
    , py::arg("idx")
    , py::return_value_policy::automatic_reference);
    gui.def("set_state_storage", &ImGui::SetStateStorage
    , py::arg("storage")
    , py::return_value_policy::automatic_reference);
    gui.def("get_state_storage", &ImGui::GetStateStorage
    , py::return_value_policy::automatic_reference);
    gui.def("calc_text_size", &ImGui::CalcTextSize
    , py::arg("text")
    , py::arg("text_end") = nullptr
    , py::arg("hide_text_after_double_hash") = false
    , py::arg("wrap_width") = -1.0f
    , py::return_value_policy::automatic_reference);
    gui.def("color_convert_u32_to_float4", &ImGui::ColorConvertU32ToFloat4
    , py::arg("in")
    , py::return_value_policy::automatic_reference);
    gui.def("color_convert_float4_to_u32", &ImGui::ColorConvertFloat4ToU32
    , py::arg("in")
    , py::return_value_policy::automatic_reference);
    gui.def("color_convert_rg_bto_hsv", [](float r, float g, float b, float & out_h, float & out_s, float & out_v)
    {
        ImGui::ColorConvertRGBtoHSV(r, g, b, out_h, out_s, out_v);
        return std::make_tuple(out_h, out_s, out_v);
    }
    , py::arg("r")
    , py::arg("g")
    , py::arg("b")
    , py::arg("out_h") = 0
    , py::arg("out_s") = 0
    , py::arg("out_v") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("color_convert_hs_vto_rgb", [](float h, float s, float v, float & out_r, float & out_g, float & out_b)
    {
        ImGui::ColorConvertHSVtoRGB(h, s, v, out_r, out_g, out_b);
        return std::make_tuple(out_r, out_g, out_b);
    }
    , py::arg("h")
    , py::arg("s")
    , py::arg("v")
    , py::arg("out_r") = 0
    , py::arg("out_g") = 0
    , py::arg("out_b")
    , py::return_value_policy::automatic_reference);
    gui.def("is_key_down", py::overload_cast<ImGuiKey>(&ImGui::IsKeyDown)
    , py::arg("key")
    , py::return_value_policy::automatic_reference);
    gui.def("is_key_pressed", py::overload_cast<ImGuiKey, bool>(&ImGui::IsKeyPressed)
    , py::arg("key")
    , py::arg("repeat") = true
    , py::return_value_policy::automatic_reference);
    gui.def("is_key_released", py::overload_cast<ImGuiKey>(&ImGui::IsKeyReleased)
    , py::arg("key")
    , py::return_value_policy::automatic_reference);
    gui.def("is_key_chord_pressed", py::overload_cast<ImGuiKeyChord>(&ImGui::IsKeyChordPressed)
    , py::arg("key_chord")
    , py::return_value_policy::automatic_reference);
    gui.def("get_key_pressed_amount", &ImGui::GetKeyPressedAmount
    , py::arg("key")
    , py::arg("repeat_delay")
    , py::arg("rate")
    , py::return_value_policy::automatic_reference);
    gui.def("get_key_name", &ImGui::GetKeyName
    , py::arg("key")
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_frame_want_capture_keyboard", &ImGui::SetNextFrameWantCaptureKeyboard
    , py::arg("want_capture_keyboard")
    , py::return_value_policy::automatic_reference);
    gui.def("shortcut", py::overload_cast<ImGuiKeyChord, ImGuiInputFlags>(&ImGui::Shortcut)
    , py::arg("key_chord")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_item_shortcut", &ImGui::SetNextItemShortcut
    , py::arg("key_chord")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("set_item_key_owner", py::overload_cast<ImGuiKey>(&ImGui::SetItemKeyOwner)
    , py::arg("key")
    , py::return_value_policy::automatic_reference);
    gui.def("is_mouse_down", py::overload_cast<ImGuiMouseButton>(&ImGui::IsMouseDown)
    , py::arg("button")
    , py::return_value_policy::automatic_reference);
    gui.def("is_mouse_clicked", py::overload_cast<ImGuiMouseButton, bool>(&ImGui::IsMouseClicked)
    , py::arg("button")
    , py::arg("repeat") = false
    , py::return_value_policy::automatic_reference);
    gui.def("is_mouse_released", py::overload_cast<ImGuiMouseButton>(&ImGui::IsMouseReleased)
    , py::arg("button")
    , py::return_value_policy::automatic_reference);
    gui.def("is_mouse_double_clicked", py::overload_cast<ImGuiMouseButton>(&ImGui::IsMouseDoubleClicked)
    , py::arg("button")
    , py::return_value_policy::automatic_reference);
    gui.def("get_mouse_clicked_count", &ImGui::GetMouseClickedCount
    , py::arg("button")
    , py::return_value_policy::automatic_reference);
    gui.def("is_mouse_hovering_rect", &ImGui::IsMouseHoveringRect
    , py::arg("r_min")
    , py::arg("r_max")
    , py::arg("clip") = true
    , py::return_value_policy::automatic_reference);
    gui.def("is_mouse_pos_valid", &ImGui::IsMousePosValid
    , py::arg("mouse_pos") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("is_any_mouse_down", &ImGui::IsAnyMouseDown
    , py::return_value_policy::automatic_reference);
    gui.def("get_mouse_pos", &ImGui::GetMousePos
    , py::return_value_policy::automatic_reference);
    gui.def("get_mouse_pos_on_opening_current_popup", &ImGui::GetMousePosOnOpeningCurrentPopup
    , py::return_value_policy::automatic_reference);
    gui.def("is_mouse_dragging", &ImGui::IsMouseDragging
    , py::arg("button")
    , py::arg("lock_threshold") = -1.0f
    , py::return_value_policy::automatic_reference);
    gui.def("get_mouse_drag_delta", &ImGui::GetMouseDragDelta
    , py::arg("button") = 0
    , py::arg("lock_threshold") = -1.0f
    , py::return_value_policy::automatic_reference);
    gui.def("reset_mouse_drag_delta", &ImGui::ResetMouseDragDelta
    , py::arg("button") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("get_mouse_cursor", &ImGui::GetMouseCursor
    , py::return_value_policy::automatic_reference);
    gui.def("set_mouse_cursor", &ImGui::SetMouseCursor
    , py::arg("cursor_type")
    , py::return_value_policy::automatic_reference);
    gui.def("set_next_frame_want_capture_mouse", &ImGui::SetNextFrameWantCaptureMouse
    , py::arg("want_capture_mouse")
    , py::return_value_policy::automatic_reference);
    gui.def("get_clipboard_text", &ImGui::GetClipboardText
    , py::return_value_policy::automatic_reference);
    gui.def("set_clipboard_text", &ImGui::SetClipboardText
    , py::arg("text")
    , py::return_value_policy::automatic_reference);
    gui.def("load_ini_settings_from_disk", &ImGui::LoadIniSettingsFromDisk
    , py::arg("ini_filename")
    , py::return_value_policy::automatic_reference);
    gui.def("load_ini_settings_from_memory", &ImGui::LoadIniSettingsFromMemory
    , py::arg("ini_data")
    , py::arg("ini_size") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("save_ini_settings_to_disk", &ImGui::SaveIniSettingsToDisk
    , py::arg("ini_filename")
    , py::return_value_policy::automatic_reference);
    gui.def("save_ini_settings_to_memory", [](size_t * out_ini_size)
    {
        auto ret = ImGui::SaveIniSettingsToMemory(out_ini_size);
        return std::make_tuple(ret, out_ini_size);
    }
    , py::arg("out_ini_size") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("debug_text_encoding", &ImGui::DebugTextEncoding
    , py::arg("text")
    , py::return_value_policy::automatic_reference);
    gui.def("debug_flash_style_color", &ImGui::DebugFlashStyleColor
    , py::arg("idx")
    , py::return_value_policy::automatic_reference);
    gui.def("debug_start_item_picker", &ImGui::DebugStartItemPicker
    , py::return_value_policy::automatic_reference);
    gui.def("debug_check_version_and_data_layout", &ImGui::DebugCheckVersionAndDataLayout
    , py::arg("version_str")
    , py::arg("sz_io")
    , py::arg("sz_style")
    , py::arg("sz_vec2")
    , py::arg("sz_vec4")
    , py::arg("sz_drawvert")
    , py::arg("sz_drawidx")
    , py::return_value_policy::automatic_reference);
    gui.def("debug_log", [](const char * fmt)
    {
        ImGui::DebugLog(fmt);
        return ;
    }
    , py::arg("fmt")
    , py::return_value_policy::automatic_reference);
    gui.def("update_platform_windows", &ImGui::UpdatePlatformWindows
    , py::return_value_policy::automatic_reference);
    gui.def("render_platform_windows_default", &ImGui::RenderPlatformWindowsDefault
    , py::arg("platform_render_arg") = nullptr
    , py::arg("renderer_render_arg") = nullptr
    , py::return_value_policy::automatic_reference);
    gui.def("destroy_platform_windows", &ImGui::DestroyPlatformWindows
    , py::return_value_policy::automatic_reference);
    gui.def("find_viewport_by_id", &ImGui::FindViewportByID
    , py::arg("id")
    , py::return_value_policy::automatic_reference);
    gui.def("find_viewport_by_platform_handle", &ImGui::FindViewportByPlatformHandle
    , py::arg("platform_handle")
    , py::return_value_policy::automatic_reference);
    py::enum_<ImGuiWindowFlags_>(gui, "WindowFlags", py::arithmetic())
        .value("NONE", ImGuiWindowFlags_None)
        .value("NO_TITLE_BAR", ImGuiWindowFlags_NoTitleBar)
        .value("NO_RESIZE", ImGuiWindowFlags_NoResize)
        .value("NO_MOVE", ImGuiWindowFlags_NoMove)
        .value("NO_SCROLLBAR", ImGuiWindowFlags_NoScrollbar)
        .value("NO_SCROLL_WITH_MOUSE", ImGuiWindowFlags_NoScrollWithMouse)
        .value("NO_COLLAPSE", ImGuiWindowFlags_NoCollapse)
        .value("ALWAYS_AUTO_RESIZE", ImGuiWindowFlags_AlwaysAutoResize)
        .value("NO_BACKGROUND", ImGuiWindowFlags_NoBackground)
        .value("NO_SAVED_SETTINGS", ImGuiWindowFlags_NoSavedSettings)
        .value("NO_MOUSE_INPUTS", ImGuiWindowFlags_NoMouseInputs)
        .value("MENU_BAR", ImGuiWindowFlags_MenuBar)
        .value("HORIZONTAL_SCROLLBAR", ImGuiWindowFlags_HorizontalScrollbar)
        .value("NO_FOCUS_ON_APPEARING", ImGuiWindowFlags_NoFocusOnAppearing)
        .value("NO_BRING_TO_FRONT_ON_FOCUS", ImGuiWindowFlags_NoBringToFrontOnFocus)
        .value("ALWAYS_VERTICAL_SCROLLBAR", ImGuiWindowFlags_AlwaysVerticalScrollbar)
        .value("ALWAYS_HORIZONTAL_SCROLLBAR", ImGuiWindowFlags_AlwaysHorizontalScrollbar)
        .value("NO_NAV_INPUTS", ImGuiWindowFlags_NoNavInputs)
        .value("NO_NAV_FOCUS", ImGuiWindowFlags_NoNavFocus)
        .value("UNSAVED_DOCUMENT", ImGuiWindowFlags_UnsavedDocument)
        .value("NO_DOCKING", ImGuiWindowFlags_NoDocking)
        .value("NO_NAV", ImGuiWindowFlags_NoNav)
        .value("NO_DECORATION", ImGuiWindowFlags_NoDecoration)
        .value("NO_INPUTS", ImGuiWindowFlags_NoInputs)
        .value("CHILD_WINDOW", ImGuiWindowFlags_ChildWindow)
        .value("TOOLTIP", ImGuiWindowFlags_Tooltip)
        .value("POPUP", ImGuiWindowFlags_Popup)
        .value("MODAL", ImGuiWindowFlags_Modal)
        .value("CHILD_MENU", ImGuiWindowFlags_ChildMenu)
        .value("DOCK_NODE_HOST", ImGuiWindowFlags_DockNodeHost)
        .export_values();

    py::enum_<ImGuiChildFlags_>(gui, "ChildFlags", py::arithmetic())
        .value("NONE", ImGuiChildFlags_None)
        .value("BORDERS", ImGuiChildFlags_Borders)
        .value("ALWAYS_USE_WINDOW_PADDING", ImGuiChildFlags_AlwaysUseWindowPadding)
        .value("RESIZE_X", ImGuiChildFlags_ResizeX)
        .value("RESIZE_Y", ImGuiChildFlags_ResizeY)
        .value("AUTO_RESIZE_X", ImGuiChildFlags_AutoResizeX)
        .value("AUTO_RESIZE_Y", ImGuiChildFlags_AutoResizeY)
        .value("ALWAYS_AUTO_RESIZE", ImGuiChildFlags_AlwaysAutoResize)
        .value("FRAME_STYLE", ImGuiChildFlags_FrameStyle)
        .value("NAV_FLATTENED", ImGuiChildFlags_NavFlattened)
        .export_values();

    py::enum_<ImGuiItemFlags_>(gui, "ItemFlags", py::arithmetic())
        .value("NONE", ImGuiItemFlags_None)
        .value("NO_TAB_STOP", ImGuiItemFlags_NoTabStop)
        .value("NO_NAV", ImGuiItemFlags_NoNav)
        .value("NO_NAV_DEFAULT_FOCUS", ImGuiItemFlags_NoNavDefaultFocus)
        .value("BUTTON_REPEAT", ImGuiItemFlags_ButtonRepeat)
        .value("AUTO_CLOSE_POPUPS", ImGuiItemFlags_AutoClosePopups)
        .value("ALLOW_DUPLICATE_ID", ImGuiItemFlags_AllowDuplicateId)
        .export_values();

    py::enum_<ImGuiInputTextFlags_>(gui, "InputTextFlags", py::arithmetic())
        .value("NONE", ImGuiInputTextFlags_None)
        .value("CHARS_DECIMAL", ImGuiInputTextFlags_CharsDecimal)
        .value("CHARS_HEXADECIMAL", ImGuiInputTextFlags_CharsHexadecimal)
        .value("CHARS_SCIENTIFIC", ImGuiInputTextFlags_CharsScientific)
        .value("CHARS_UPPERCASE", ImGuiInputTextFlags_CharsUppercase)
        .value("CHARS_NO_BLANK", ImGuiInputTextFlags_CharsNoBlank)
        .value("ALLOW_TAB_INPUT", ImGuiInputTextFlags_AllowTabInput)
        .value("ENTER_RETURNS_TRUE", ImGuiInputTextFlags_EnterReturnsTrue)
        .value("ESCAPE_CLEARS_ALL", ImGuiInputTextFlags_EscapeClearsAll)
        .value("CTRL_ENTER_FOR_NEW_LINE", ImGuiInputTextFlags_CtrlEnterForNewLine)
        .value("READ_ONLY", ImGuiInputTextFlags_ReadOnly)
        .value("PASSWORD", ImGuiInputTextFlags_Password)
        .value("ALWAYS_OVERWRITE", ImGuiInputTextFlags_AlwaysOverwrite)
        .value("AUTO_SELECT_ALL", ImGuiInputTextFlags_AutoSelectAll)
        .value("PARSE_EMPTY_REF_VAL", ImGuiInputTextFlags_ParseEmptyRefVal)
        .value("DISPLAY_EMPTY_REF_VAL", ImGuiInputTextFlags_DisplayEmptyRefVal)
        .value("NO_HORIZONTAL_SCROLL", ImGuiInputTextFlags_NoHorizontalScroll)
        .value("NO_UNDO_REDO", ImGuiInputTextFlags_NoUndoRedo)
        .value("CALLBACK_COMPLETION", ImGuiInputTextFlags_CallbackCompletion)
        .value("CALLBACK_HISTORY", ImGuiInputTextFlags_CallbackHistory)
        .value("CALLBACK_ALWAYS", ImGuiInputTextFlags_CallbackAlways)
        .value("CALLBACK_CHAR_FILTER", ImGuiInputTextFlags_CallbackCharFilter)
        .value("CALLBACK_RESIZE", ImGuiInputTextFlags_CallbackResize)
        .value("CALLBACK_EDIT", ImGuiInputTextFlags_CallbackEdit)
        .export_values();

    py::enum_<ImGuiTreeNodeFlags_>(gui, "TreeNodeFlags", py::arithmetic())
        .value("NONE", ImGuiTreeNodeFlags_None)
        .value("SELECTED", ImGuiTreeNodeFlags_Selected)
        .value("FRAMED", ImGuiTreeNodeFlags_Framed)
        .value("ALLOW_OVERLAP", ImGuiTreeNodeFlags_AllowOverlap)
        .value("NO_TREE_PUSH_ON_OPEN", ImGuiTreeNodeFlags_NoTreePushOnOpen)
        .value("NO_AUTO_OPEN_ON_LOG", ImGuiTreeNodeFlags_NoAutoOpenOnLog)
        .value("DEFAULT_OPEN", ImGuiTreeNodeFlags_DefaultOpen)
        .value("OPEN_ON_DOUBLE_CLICK", ImGuiTreeNodeFlags_OpenOnDoubleClick)
        .value("OPEN_ON_ARROW", ImGuiTreeNodeFlags_OpenOnArrow)
        .value("LEAF", ImGuiTreeNodeFlags_Leaf)
        .value("BULLET", ImGuiTreeNodeFlags_Bullet)
        .value("FRAME_PADDING", ImGuiTreeNodeFlags_FramePadding)
        .value("SPAN_AVAIL_WIDTH", ImGuiTreeNodeFlags_SpanAvailWidth)
        .value("SPAN_FULL_WIDTH", ImGuiTreeNodeFlags_SpanFullWidth)
        .value("SPAN_TEXT_WIDTH", ImGuiTreeNodeFlags_SpanTextWidth)
        .value("SPAN_ALL_COLUMNS", ImGuiTreeNodeFlags_SpanAllColumns)
        .value("NAV_LEFT_JUMPS_BACK_HERE", ImGuiTreeNodeFlags_NavLeftJumpsBackHere)
        .value("COLLAPSING_HEADER", ImGuiTreeNodeFlags_CollapsingHeader)
        .export_values();

    py::enum_<ImGuiPopupFlags_>(gui, "PopupFlags", py::arithmetic())
        .value("NONE", ImGuiPopupFlags_None)
        .value("MOUSE_BUTTON_LEFT", ImGuiPopupFlags_MouseButtonLeft)
        .value("MOUSE_BUTTON_RIGHT", ImGuiPopupFlags_MouseButtonRight)
        .value("MOUSE_BUTTON_MIDDLE", ImGuiPopupFlags_MouseButtonMiddle)
        .value("MOUSE_BUTTON_MASK", ImGuiPopupFlags_MouseButtonMask_)
        .value("MOUSE_BUTTON_DEFAULT", ImGuiPopupFlags_MouseButtonDefault_)
        .value("NO_REOPEN", ImGuiPopupFlags_NoReopen)
        .value("NO_OPEN_OVER_EXISTING_POPUP", ImGuiPopupFlags_NoOpenOverExistingPopup)
        .value("NO_OPEN_OVER_ITEMS", ImGuiPopupFlags_NoOpenOverItems)
        .value("ANY_POPUP_ID", ImGuiPopupFlags_AnyPopupId)
        .value("ANY_POPUP_LEVEL", ImGuiPopupFlags_AnyPopupLevel)
        .value("ANY_POPUP", ImGuiPopupFlags_AnyPopup)
        .export_values();

    py::enum_<ImGuiSelectableFlags_>(gui, "SelectableFlags", py::arithmetic())
        .value("NONE", ImGuiSelectableFlags_None)
        .value("NO_AUTO_CLOSE_POPUPS", ImGuiSelectableFlags_NoAutoClosePopups)
        .value("SPAN_ALL_COLUMNS", ImGuiSelectableFlags_SpanAllColumns)
        .value("ALLOW_DOUBLE_CLICK", ImGuiSelectableFlags_AllowDoubleClick)
        .value("DISABLED", ImGuiSelectableFlags_Disabled)
        .value("ALLOW_OVERLAP", ImGuiSelectableFlags_AllowOverlap)
        .value("HIGHLIGHT", ImGuiSelectableFlags_Highlight)
        .export_values();

    py::enum_<ImGuiComboFlags_>(gui, "ComboFlags", py::arithmetic())
        .value("NONE", ImGuiComboFlags_None)
        .value("POPUP_ALIGN_LEFT", ImGuiComboFlags_PopupAlignLeft)
        .value("HEIGHT_SMALL", ImGuiComboFlags_HeightSmall)
        .value("HEIGHT_REGULAR", ImGuiComboFlags_HeightRegular)
        .value("HEIGHT_LARGE", ImGuiComboFlags_HeightLarge)
        .value("HEIGHT_LARGEST", ImGuiComboFlags_HeightLargest)
        .value("NO_ARROW_BUTTON", ImGuiComboFlags_NoArrowButton)
        .value("NO_PREVIEW", ImGuiComboFlags_NoPreview)
        .value("WIDTH_FIT_PREVIEW", ImGuiComboFlags_WidthFitPreview)
        .value("HEIGHT_MASK", ImGuiComboFlags_HeightMask_)
        .export_values();

    py::enum_<ImGuiTabBarFlags_>(gui, "TabBarFlags", py::arithmetic())
        .value("NONE", ImGuiTabBarFlags_None)
        .value("REORDERABLE", ImGuiTabBarFlags_Reorderable)
        .value("AUTO_SELECT_NEW_TABS", ImGuiTabBarFlags_AutoSelectNewTabs)
        .value("TAB_LIST_POPUP_BUTTON", ImGuiTabBarFlags_TabListPopupButton)
        .value("NO_CLOSE_WITH_MIDDLE_MOUSE_BUTTON", ImGuiTabBarFlags_NoCloseWithMiddleMouseButton)
        .value("NO_TAB_LIST_SCROLLING_BUTTONS", ImGuiTabBarFlags_NoTabListScrollingButtons)
        .value("NO_TOOLTIP", ImGuiTabBarFlags_NoTooltip)
        .value("DRAW_SELECTED_OVERLINE", ImGuiTabBarFlags_DrawSelectedOverline)
        .value("FITTING_POLICY_RESIZE_DOWN", ImGuiTabBarFlags_FittingPolicyResizeDown)
        .value("FITTING_POLICY_SCROLL", ImGuiTabBarFlags_FittingPolicyScroll)
        .value("FITTING_POLICY_MASK", ImGuiTabBarFlags_FittingPolicyMask_)
        .value("FITTING_POLICY_DEFAULT", ImGuiTabBarFlags_FittingPolicyDefault_)
        .export_values();

    py::enum_<ImGuiTabItemFlags_>(gui, "TabItemFlags", py::arithmetic())
        .value("NONE", ImGuiTabItemFlags_None)
        .value("UNSAVED_DOCUMENT", ImGuiTabItemFlags_UnsavedDocument)
        .value("SET_SELECTED", ImGuiTabItemFlags_SetSelected)
        .value("NO_CLOSE_WITH_MIDDLE_MOUSE_BUTTON", ImGuiTabItemFlags_NoCloseWithMiddleMouseButton)
        .value("NO_PUSH_ID", ImGuiTabItemFlags_NoPushId)
        .value("NO_TOOLTIP", ImGuiTabItemFlags_NoTooltip)
        .value("NO_REORDER", ImGuiTabItemFlags_NoReorder)
        .value("LEADING", ImGuiTabItemFlags_Leading)
        .value("TRAILING", ImGuiTabItemFlags_Trailing)
        .value("NO_ASSUMED_CLOSURE", ImGuiTabItemFlags_NoAssumedClosure)
        .export_values();

    py::enum_<ImGuiFocusedFlags_>(gui, "FocusedFlags", py::arithmetic())
        .value("NONE", ImGuiFocusedFlags_None)
        .value("CHILD_WINDOWS", ImGuiFocusedFlags_ChildWindows)
        .value("ROOT_WINDOW", ImGuiFocusedFlags_RootWindow)
        .value("ANY_WINDOW", ImGuiFocusedFlags_AnyWindow)
        .value("NO_POPUP_HIERARCHY", ImGuiFocusedFlags_NoPopupHierarchy)
        .value("DOCK_HIERARCHY", ImGuiFocusedFlags_DockHierarchy)
        .value("ROOT_AND_CHILD_WINDOWS", ImGuiFocusedFlags_RootAndChildWindows)
        .export_values();

    py::enum_<ImGuiHoveredFlags_>(gui, "HoveredFlags", py::arithmetic())
        .value("NONE", ImGuiHoveredFlags_None)
        .value("CHILD_WINDOWS", ImGuiHoveredFlags_ChildWindows)
        .value("ROOT_WINDOW", ImGuiHoveredFlags_RootWindow)
        .value("ANY_WINDOW", ImGuiHoveredFlags_AnyWindow)
        .value("NO_POPUP_HIERARCHY", ImGuiHoveredFlags_NoPopupHierarchy)
        .value("DOCK_HIERARCHY", ImGuiHoveredFlags_DockHierarchy)
        .value("ALLOW_WHEN_BLOCKED_BY_POPUP", ImGuiHoveredFlags_AllowWhenBlockedByPopup)
        .value("ALLOW_WHEN_BLOCKED_BY_ACTIVE_ITEM", ImGuiHoveredFlags_AllowWhenBlockedByActiveItem)
        .value("ALLOW_WHEN_OVERLAPPED_BY_ITEM", ImGuiHoveredFlags_AllowWhenOverlappedByItem)
        .value("ALLOW_WHEN_OVERLAPPED_BY_WINDOW", ImGuiHoveredFlags_AllowWhenOverlappedByWindow)
        .value("ALLOW_WHEN_DISABLED", ImGuiHoveredFlags_AllowWhenDisabled)
        .value("NO_NAV_OVERRIDE", ImGuiHoveredFlags_NoNavOverride)
        .value("ALLOW_WHEN_OVERLAPPED", ImGuiHoveredFlags_AllowWhenOverlapped)
        .value("RECT_ONLY", ImGuiHoveredFlags_RectOnly)
        .value("ROOT_AND_CHILD_WINDOWS", ImGuiHoveredFlags_RootAndChildWindows)
        .value("FOR_TOOLTIP", ImGuiHoveredFlags_ForTooltip)
        .value("STATIONARY", ImGuiHoveredFlags_Stationary)
        .value("DELAY_NONE", ImGuiHoveredFlags_DelayNone)
        .value("DELAY_SHORT", ImGuiHoveredFlags_DelayShort)
        .value("DELAY_NORMAL", ImGuiHoveredFlags_DelayNormal)
        .value("NO_SHARED_DELAY", ImGuiHoveredFlags_NoSharedDelay)
        .export_values();

    py::enum_<ImGuiDockNodeFlags_>(gui, "DockNodeFlags", py::arithmetic())
        .value("NONE", ImGuiDockNodeFlags_None)
        .value("KEEP_ALIVE_ONLY", ImGuiDockNodeFlags_KeepAliveOnly)
        .value("NO_DOCKING_OVER_CENTRAL_NODE", ImGuiDockNodeFlags_NoDockingOverCentralNode)
        .value("PASSTHRU_CENTRAL_NODE", ImGuiDockNodeFlags_PassthruCentralNode)
        .value("NO_DOCKING_SPLIT", ImGuiDockNodeFlags_NoDockingSplit)
        .value("NO_RESIZE", ImGuiDockNodeFlags_NoResize)
        .value("AUTO_HIDE_TAB_BAR", ImGuiDockNodeFlags_AutoHideTabBar)
        .value("NO_UNDOCKING", ImGuiDockNodeFlags_NoUndocking)
        .export_values();

    py::enum_<ImGuiDragDropFlags_>(gui, "DragDropFlags", py::arithmetic())
        .value("NONE", ImGuiDragDropFlags_None)
        .value("SOURCE_NO_PREVIEW_TOOLTIP", ImGuiDragDropFlags_SourceNoPreviewTooltip)
        .value("SOURCE_NO_DISABLE_HOVER", ImGuiDragDropFlags_SourceNoDisableHover)
        .value("SOURCE_NO_HOLD_TO_OPEN_OTHERS", ImGuiDragDropFlags_SourceNoHoldToOpenOthers)
        .value("SOURCE_ALLOW_NULL_ID", ImGuiDragDropFlags_SourceAllowNullID)
        .value("SOURCE_EXTERN", ImGuiDragDropFlags_SourceExtern)
        .value("PAYLOAD_AUTO_EXPIRE", ImGuiDragDropFlags_PayloadAutoExpire)
        .value("PAYLOAD_NO_CROSS_CONTEXT", ImGuiDragDropFlags_PayloadNoCrossContext)
        .value("PAYLOAD_NO_CROSS_PROCESS", ImGuiDragDropFlags_PayloadNoCrossProcess)
        .value("ACCEPT_BEFORE_DELIVERY", ImGuiDragDropFlags_AcceptBeforeDelivery)
        .value("ACCEPT_NO_DRAW_DEFAULT_RECT", ImGuiDragDropFlags_AcceptNoDrawDefaultRect)
        .value("ACCEPT_NO_PREVIEW_TOOLTIP", ImGuiDragDropFlags_AcceptNoPreviewTooltip)
        .value("ACCEPT_PEEK_ONLY", ImGuiDragDropFlags_AcceptPeekOnly)
        .export_values();

    py::enum_<ImGuiDataType_>(gui, "DataType", py::arithmetic())
        .value("S8", ImGuiDataType_S8)
        .value("U8", ImGuiDataType_U8)
        .value("S16", ImGuiDataType_S16)
        .value("U16", ImGuiDataType_U16)
        .value("S32", ImGuiDataType_S32)
        .value("U32", ImGuiDataType_U32)
        .value("S64", ImGuiDataType_S64)
        .value("U64", ImGuiDataType_U64)
        .value("FLOAT", ImGuiDataType_Float)
        .value("DOUBLE", ImGuiDataType_Double)
        .value("BOOL", ImGuiDataType_Bool)
        .value("COUNT", ImGuiDataType_COUNT)
        .export_values();

    py::enum_<ImGuiDir>(gui, "Dir", py::arithmetic())
        .value("NONE", ImGuiDir_None)
        .value("LEFT", ImGuiDir_Left)
        .value("RIGHT", ImGuiDir_Right)
        .value("UP", ImGuiDir_Up)
        .value("DOWN", ImGuiDir_Down)
        .value("COUNT", ImGuiDir_COUNT)
        .export_values();

    py::enum_<ImGuiSortDirection>(gui, "SortDirection", py::arithmetic())
        .value("NONE", ImGuiSortDirection_None)
        .value("ASCENDING", ImGuiSortDirection_Ascending)
        .value("DESCENDING", ImGuiSortDirection_Descending)
        .export_values();

    py::enum_<ImGuiKey>(gui, "Key", py::arithmetic())
        .value("NONE", ImGuiKey_None)
        .value("TAB", ImGuiKey_Tab)
        .value("LEFT_ARROW", ImGuiKey_LeftArrow)
        .value("RIGHT_ARROW", ImGuiKey_RightArrow)
        .value("UP_ARROW", ImGuiKey_UpArrow)
        .value("DOWN_ARROW", ImGuiKey_DownArrow)
        .value("PAGE_UP", ImGuiKey_PageUp)
        .value("PAGE_DOWN", ImGuiKey_PageDown)
        .value("HOME", ImGuiKey_Home)
        .value("END", ImGuiKey_End)
        .value("INSERT", ImGuiKey_Insert)
        .value("DELETE", ImGuiKey_Delete)
        .value("BACKSPACE", ImGuiKey_Backspace)
        .value("SPACE", ImGuiKey_Space)
        .value("ENTER", ImGuiKey_Enter)
        .value("ESCAPE", ImGuiKey_Escape)
        .value("LEFT_CTRL", ImGuiKey_LeftCtrl)
        .value("LEFT_SHIFT", ImGuiKey_LeftShift)
        .value("LEFT_ALT", ImGuiKey_LeftAlt)
        .value("LEFT_SUPER", ImGuiKey_LeftSuper)
        .value("RIGHT_CTRL", ImGuiKey_RightCtrl)
        .value("RIGHT_SHIFT", ImGuiKey_RightShift)
        .value("RIGHT_ALT", ImGuiKey_RightAlt)
        .value("RIGHT_SUPER", ImGuiKey_RightSuper)
        .value("MENU", ImGuiKey_Menu)
        .value("0", ImGuiKey_0)
        .value("1", ImGuiKey_1)
        .value("2", ImGuiKey_2)
        .value("3", ImGuiKey_3)
        .value("4", ImGuiKey_4)
        .value("5", ImGuiKey_5)
        .value("6", ImGuiKey_6)
        .value("7", ImGuiKey_7)
        .value("8", ImGuiKey_8)
        .value("9", ImGuiKey_9)
        .value("A", ImGuiKey_A)
        .value("B", ImGuiKey_B)
        .value("C", ImGuiKey_C)
        .value("D", ImGuiKey_D)
        .value("E", ImGuiKey_E)
        .value("F", ImGuiKey_F)
        .value("G", ImGuiKey_G)
        .value("H", ImGuiKey_H)
        .value("I", ImGuiKey_I)
        .value("J", ImGuiKey_J)
        .value("K", ImGuiKey_K)
        .value("L", ImGuiKey_L)
        .value("M", ImGuiKey_M)
        .value("N", ImGuiKey_N)
        .value("O", ImGuiKey_O)
        .value("P", ImGuiKey_P)
        .value("Q", ImGuiKey_Q)
        .value("R", ImGuiKey_R)
        .value("S", ImGuiKey_S)
        .value("T", ImGuiKey_T)
        .value("U", ImGuiKey_U)
        .value("V", ImGuiKey_V)
        .value("W", ImGuiKey_W)
        .value("X", ImGuiKey_X)
        .value("Y", ImGuiKey_Y)
        .value("Z", ImGuiKey_Z)
        .value("F1", ImGuiKey_F1)
        .value("F2", ImGuiKey_F2)
        .value("F3", ImGuiKey_F3)
        .value("F4", ImGuiKey_F4)
        .value("F5", ImGuiKey_F5)
        .value("F6", ImGuiKey_F6)
        .value("F7", ImGuiKey_F7)
        .value("F8", ImGuiKey_F8)
        .value("F9", ImGuiKey_F9)
        .value("F10", ImGuiKey_F10)
        .value("F11", ImGuiKey_F11)
        .value("F12", ImGuiKey_F12)
        .value("F13", ImGuiKey_F13)
        .value("F14", ImGuiKey_F14)
        .value("F15", ImGuiKey_F15)
        .value("F16", ImGuiKey_F16)
        .value("F17", ImGuiKey_F17)
        .value("F18", ImGuiKey_F18)
        .value("F19", ImGuiKey_F19)
        .value("F20", ImGuiKey_F20)
        .value("F21", ImGuiKey_F21)
        .value("F22", ImGuiKey_F22)
        .value("F23", ImGuiKey_F23)
        .value("F24", ImGuiKey_F24)
        .value("APOSTROPHE", ImGuiKey_Apostrophe)
        .value("COMMA", ImGuiKey_Comma)
        .value("MINUS", ImGuiKey_Minus)
        .value("PERIOD", ImGuiKey_Period)
        .value("SLASH", ImGuiKey_Slash)
        .value("SEMICOLON", ImGuiKey_Semicolon)
        .value("EQUAL", ImGuiKey_Equal)
        .value("LEFT_BRACKET", ImGuiKey_LeftBracket)
        .value("BACKSLASH", ImGuiKey_Backslash)
        .value("RIGHT_BRACKET", ImGuiKey_RightBracket)
        .value("GRAVE_ACCENT", ImGuiKey_GraveAccent)
        .value("CAPS_LOCK", ImGuiKey_CapsLock)
        .value("SCROLL_LOCK", ImGuiKey_ScrollLock)
        .value("NUM_LOCK", ImGuiKey_NumLock)
        .value("PRINT_SCREEN", ImGuiKey_PrintScreen)
        .value("PAUSE", ImGuiKey_Pause)
        .value("KEYPAD0", ImGuiKey_Keypad0)
        .value("KEYPAD1", ImGuiKey_Keypad1)
        .value("KEYPAD2", ImGuiKey_Keypad2)
        .value("KEYPAD3", ImGuiKey_Keypad3)
        .value("KEYPAD4", ImGuiKey_Keypad4)
        .value("KEYPAD5", ImGuiKey_Keypad5)
        .value("KEYPAD6", ImGuiKey_Keypad6)
        .value("KEYPAD7", ImGuiKey_Keypad7)
        .value("KEYPAD8", ImGuiKey_Keypad8)
        .value("KEYPAD9", ImGuiKey_Keypad9)
        .value("KEYPAD_DECIMAL", ImGuiKey_KeypadDecimal)
        .value("KEYPAD_DIVIDE", ImGuiKey_KeypadDivide)
        .value("KEYPAD_MULTIPLY", ImGuiKey_KeypadMultiply)
        .value("KEYPAD_SUBTRACT", ImGuiKey_KeypadSubtract)
        .value("KEYPAD_ADD", ImGuiKey_KeypadAdd)
        .value("KEYPAD_ENTER", ImGuiKey_KeypadEnter)
        .value("KEYPAD_EQUAL", ImGuiKey_KeypadEqual)
        .value("APP_BACK", ImGuiKey_AppBack)
        .value("APP_FORWARD", ImGuiKey_AppForward)
        .value("GAMEPAD_START", ImGuiKey_GamepadStart)
        .value("GAMEPAD_BACK", ImGuiKey_GamepadBack)
        .value("GAMEPAD_FACE_LEFT", ImGuiKey_GamepadFaceLeft)
        .value("GAMEPAD_FACE_RIGHT", ImGuiKey_GamepadFaceRight)
        .value("GAMEPAD_FACE_UP", ImGuiKey_GamepadFaceUp)
        .value("GAMEPAD_FACE_DOWN", ImGuiKey_GamepadFaceDown)
        .value("GAMEPAD_DPAD_LEFT", ImGuiKey_GamepadDpadLeft)
        .value("GAMEPAD_DPAD_RIGHT", ImGuiKey_GamepadDpadRight)
        .value("GAMEPAD_DPAD_UP", ImGuiKey_GamepadDpadUp)
        .value("GAMEPAD_DPAD_DOWN", ImGuiKey_GamepadDpadDown)
        .value("GAMEPAD_L1", ImGuiKey_GamepadL1)
        .value("GAMEPAD_R1", ImGuiKey_GamepadR1)
        .value("GAMEPAD_L2", ImGuiKey_GamepadL2)
        .value("GAMEPAD_R2", ImGuiKey_GamepadR2)
        .value("GAMEPAD_L3", ImGuiKey_GamepadL3)
        .value("GAMEPAD_R3", ImGuiKey_GamepadR3)
        .value("GAMEPAD_L_STICK_LEFT", ImGuiKey_GamepadLStickLeft)
        .value("GAMEPAD_L_STICK_RIGHT", ImGuiKey_GamepadLStickRight)
        .value("GAMEPAD_L_STICK_UP", ImGuiKey_GamepadLStickUp)
        .value("GAMEPAD_L_STICK_DOWN", ImGuiKey_GamepadLStickDown)
        .value("GAMEPAD_R_STICK_LEFT", ImGuiKey_GamepadRStickLeft)
        .value("GAMEPAD_R_STICK_RIGHT", ImGuiKey_GamepadRStickRight)
        .value("GAMEPAD_R_STICK_UP", ImGuiKey_GamepadRStickUp)
        .value("GAMEPAD_R_STICK_DOWN", ImGuiKey_GamepadRStickDown)
        .value("MOUSE_LEFT", ImGuiKey_MouseLeft)
        .value("MOUSE_RIGHT", ImGuiKey_MouseRight)
        .value("MOUSE_MIDDLE", ImGuiKey_MouseMiddle)
        .value("MOUSE_X1", ImGuiKey_MouseX1)
        .value("MOUSE_X2", ImGuiKey_MouseX2)
        .value("MOUSE_WHEEL_X", ImGuiKey_MouseWheelX)
        .value("MOUSE_WHEEL_Y", ImGuiKey_MouseWheelY)
        .value("RESERVED_FOR_MOD_CTRL", ImGuiKey_ReservedForModCtrl)
        .value("RESERVED_FOR_MOD_SHIFT", ImGuiKey_ReservedForModShift)
        .value("RESERVED_FOR_MOD_ALT", ImGuiKey_ReservedForModAlt)
        .value("RESERVED_FOR_MOD_SUPER", ImGuiKey_ReservedForModSuper)
        .value("COUNT", ImGuiKey_COUNT)
        .value("MOD_NONE", ImGuiMod_None)
        .value("MOD_CTRL", ImGuiMod_Ctrl)
        .value("MOD_SHIFT", ImGuiMod_Shift)
        .value("MOD_ALT", ImGuiMod_Alt)
        .value("MOD_SUPER", ImGuiMod_Super)
        .value("MOD_MASK", ImGuiMod_Mask_)
        .value("NAMED_KEY_BEGIN", ImGuiKey_NamedKey_BEGIN)
        .value("NAMED_KEY_END", ImGuiKey_NamedKey_END)
        .value("NAMED_KEY_COUNT", ImGuiKey_NamedKey_COUNT)
        .value("KEYS_DATA_SIZE", ImGuiKey_KeysData_SIZE)
        .value("KEYS_DATA_OFFSET", ImGuiKey_KeysData_OFFSET)
        .export_values();

    py::enum_<ImGuiInputFlags_>(gui, "InputFlags", py::arithmetic())
        .value("NONE", ImGuiInputFlags_None)
        .value("REPEAT", ImGuiInputFlags_Repeat)
        .value("ROUTE_ACTIVE", ImGuiInputFlags_RouteActive)
        .value("ROUTE_FOCUSED", ImGuiInputFlags_RouteFocused)
        .value("ROUTE_GLOBAL", ImGuiInputFlags_RouteGlobal)
        .value("ROUTE_ALWAYS", ImGuiInputFlags_RouteAlways)
        .value("ROUTE_OVER_FOCUSED", ImGuiInputFlags_RouteOverFocused)
        .value("ROUTE_OVER_ACTIVE", ImGuiInputFlags_RouteOverActive)
        .value("ROUTE_UNLESS_BG_FOCUSED", ImGuiInputFlags_RouteUnlessBgFocused)
        .value("ROUTE_FROM_ROOT_WINDOW", ImGuiInputFlags_RouteFromRootWindow)
        .value("TOOLTIP", ImGuiInputFlags_Tooltip)
        .export_values();

    py::enum_<ImGuiConfigFlags_>(gui, "ConfigFlags", py::arithmetic())
        .value("NONE", ImGuiConfigFlags_None)
        .value("NAV_ENABLE_KEYBOARD", ImGuiConfigFlags_NavEnableKeyboard)
        .value("NAV_ENABLE_GAMEPAD", ImGuiConfigFlags_NavEnableGamepad)
        .value("NAV_ENABLE_SET_MOUSE_POS", ImGuiConfigFlags_NavEnableSetMousePos)
        .value("NAV_NO_CAPTURE_KEYBOARD", ImGuiConfigFlags_NavNoCaptureKeyboard)
        .value("NO_MOUSE", ImGuiConfigFlags_NoMouse)
        .value("NO_MOUSE_CURSOR_CHANGE", ImGuiConfigFlags_NoMouseCursorChange)
        .value("NO_KEYBOARD", ImGuiConfigFlags_NoKeyboard)
        .value("DOCKING_ENABLE", ImGuiConfigFlags_DockingEnable)
        .value("VIEWPORTS_ENABLE", ImGuiConfigFlags_ViewportsEnable)
        .value("DPI_ENABLE_SCALE_VIEWPORTS", ImGuiConfigFlags_DpiEnableScaleViewports)
        .value("DPI_ENABLE_SCALE_FONTS", ImGuiConfigFlags_DpiEnableScaleFonts)
        .value("IS_SRGB", ImGuiConfigFlags_IsSRGB)
        .value("IS_TOUCH_SCREEN", ImGuiConfigFlags_IsTouchScreen)
        .export_values();

    py::enum_<ImGuiBackendFlags_>(gui, "BackendFlags", py::arithmetic())
        .value("NONE", ImGuiBackendFlags_None)
        .value("HAS_GAMEPAD", ImGuiBackendFlags_HasGamepad)
        .value("HAS_MOUSE_CURSORS", ImGuiBackendFlags_HasMouseCursors)
        .value("HAS_SET_MOUSE_POS", ImGuiBackendFlags_HasSetMousePos)
        .value("RENDERER_HAS_VTX_OFFSET", ImGuiBackendFlags_RendererHasVtxOffset)
        .value("PLATFORM_HAS_VIEWPORTS", ImGuiBackendFlags_PlatformHasViewports)
        .value("HAS_MOUSE_HOVERED_VIEWPORT", ImGuiBackendFlags_HasMouseHoveredViewport)
        .value("RENDERER_HAS_VIEWPORTS", ImGuiBackendFlags_RendererHasViewports)
        .export_values();

    py::enum_<ImGuiCol_>(gui, "Col", py::arithmetic())
        .value("TEXT", ImGuiCol_Text)
        .value("TEXT_DISABLED", ImGuiCol_TextDisabled)
        .value("WINDOW_BG", ImGuiCol_WindowBg)
        .value("CHILD_BG", ImGuiCol_ChildBg)
        .value("POPUP_BG", ImGuiCol_PopupBg)
        .value("BORDER", ImGuiCol_Border)
        .value("BORDER_SHADOW", ImGuiCol_BorderShadow)
        .value("FRAME_BG", ImGuiCol_FrameBg)
        .value("FRAME_BG_HOVERED", ImGuiCol_FrameBgHovered)
        .value("FRAME_BG_ACTIVE", ImGuiCol_FrameBgActive)
        .value("TITLE_BG", ImGuiCol_TitleBg)
        .value("TITLE_BG_ACTIVE", ImGuiCol_TitleBgActive)
        .value("TITLE_BG_COLLAPSED", ImGuiCol_TitleBgCollapsed)
        .value("MENU_BAR_BG", ImGuiCol_MenuBarBg)
        .value("SCROLLBAR_BG", ImGuiCol_ScrollbarBg)
        .value("SCROLLBAR_GRAB", ImGuiCol_ScrollbarGrab)
        .value("SCROLLBAR_GRAB_HOVERED", ImGuiCol_ScrollbarGrabHovered)
        .value("SCROLLBAR_GRAB_ACTIVE", ImGuiCol_ScrollbarGrabActive)
        .value("CHECK_MARK", ImGuiCol_CheckMark)
        .value("SLIDER_GRAB", ImGuiCol_SliderGrab)
        .value("SLIDER_GRAB_ACTIVE", ImGuiCol_SliderGrabActive)
        .value("BUTTON", ImGuiCol_Button)
        .value("BUTTON_HOVERED", ImGuiCol_ButtonHovered)
        .value("BUTTON_ACTIVE", ImGuiCol_ButtonActive)
        .value("HEADER", ImGuiCol_Header)
        .value("HEADER_HOVERED", ImGuiCol_HeaderHovered)
        .value("HEADER_ACTIVE", ImGuiCol_HeaderActive)
        .value("SEPARATOR", ImGuiCol_Separator)
        .value("SEPARATOR_HOVERED", ImGuiCol_SeparatorHovered)
        .value("SEPARATOR_ACTIVE", ImGuiCol_SeparatorActive)
        .value("RESIZE_GRIP", ImGuiCol_ResizeGrip)
        .value("RESIZE_GRIP_HOVERED", ImGuiCol_ResizeGripHovered)
        .value("RESIZE_GRIP_ACTIVE", ImGuiCol_ResizeGripActive)
        .value("TAB_HOVERED", ImGuiCol_TabHovered)
        .value("TAB", ImGuiCol_Tab)
        .value("TAB_SELECTED", ImGuiCol_TabSelected)
        .value("TAB_SELECTED_OVERLINE", ImGuiCol_TabSelectedOverline)
        .value("TAB_DIMMED", ImGuiCol_TabDimmed)
        .value("TAB_DIMMED_SELECTED", ImGuiCol_TabDimmedSelected)
        .value("TAB_DIMMED_SELECTED_OVERLINE", ImGuiCol_TabDimmedSelectedOverline)
        .value("DOCKING_PREVIEW", ImGuiCol_DockingPreview)
        .value("DOCKING_EMPTY_BG", ImGuiCol_DockingEmptyBg)
        .value("PLOT_LINES", ImGuiCol_PlotLines)
        .value("PLOT_LINES_HOVERED", ImGuiCol_PlotLinesHovered)
        .value("PLOT_HISTOGRAM", ImGuiCol_PlotHistogram)
        .value("PLOT_HISTOGRAM_HOVERED", ImGuiCol_PlotHistogramHovered)
        .value("TABLE_HEADER_BG", ImGuiCol_TableHeaderBg)
        .value("TABLE_BORDER_STRONG", ImGuiCol_TableBorderStrong)
        .value("TABLE_BORDER_LIGHT", ImGuiCol_TableBorderLight)
        .value("TABLE_ROW_BG", ImGuiCol_TableRowBg)
        .value("TABLE_ROW_BG_ALT", ImGuiCol_TableRowBgAlt)
        .value("TEXT_LINK", ImGuiCol_TextLink)
        .value("TEXT_SELECTED_BG", ImGuiCol_TextSelectedBg)
        .value("DRAG_DROP_TARGET", ImGuiCol_DragDropTarget)
        .value("NAV_HIGHLIGHT", ImGuiCol_NavHighlight)
        .value("NAV_WINDOWING_HIGHLIGHT", ImGuiCol_NavWindowingHighlight)
        .value("NAV_WINDOWING_DIM_BG", ImGuiCol_NavWindowingDimBg)
        .value("MODAL_WINDOW_DIM_BG", ImGuiCol_ModalWindowDimBg)
        .value("COUNT", ImGuiCol_COUNT)
        .export_values();

    py::enum_<ImGuiStyleVar_>(gui, "StyleVar", py::arithmetic())
        .value("ALPHA", ImGuiStyleVar_Alpha)
        .value("DISABLED_ALPHA", ImGuiStyleVar_DisabledAlpha)
        .value("WINDOW_PADDING", ImGuiStyleVar_WindowPadding)
        .value("WINDOW_ROUNDING", ImGuiStyleVar_WindowRounding)
        .value("WINDOW_BORDER_SIZE", ImGuiStyleVar_WindowBorderSize)
        .value("WINDOW_MIN_SIZE", ImGuiStyleVar_WindowMinSize)
        .value("WINDOW_TITLE_ALIGN", ImGuiStyleVar_WindowTitleAlign)
        .value("CHILD_ROUNDING", ImGuiStyleVar_ChildRounding)
        .value("CHILD_BORDER_SIZE", ImGuiStyleVar_ChildBorderSize)
        .value("POPUP_ROUNDING", ImGuiStyleVar_PopupRounding)
        .value("POPUP_BORDER_SIZE", ImGuiStyleVar_PopupBorderSize)
        .value("FRAME_PADDING", ImGuiStyleVar_FramePadding)
        .value("FRAME_ROUNDING", ImGuiStyleVar_FrameRounding)
        .value("FRAME_BORDER_SIZE", ImGuiStyleVar_FrameBorderSize)
        .value("ITEM_SPACING", ImGuiStyleVar_ItemSpacing)
        .value("ITEM_INNER_SPACING", ImGuiStyleVar_ItemInnerSpacing)
        .value("INDENT_SPACING", ImGuiStyleVar_IndentSpacing)
        .value("CELL_PADDING", ImGuiStyleVar_CellPadding)
        .value("SCROLLBAR_SIZE", ImGuiStyleVar_ScrollbarSize)
        .value("SCROLLBAR_ROUNDING", ImGuiStyleVar_ScrollbarRounding)
        .value("GRAB_MIN_SIZE", ImGuiStyleVar_GrabMinSize)
        .value("GRAB_ROUNDING", ImGuiStyleVar_GrabRounding)
        .value("TAB_ROUNDING", ImGuiStyleVar_TabRounding)
        .value("TAB_BORDER_SIZE", ImGuiStyleVar_TabBorderSize)
        .value("TAB_BAR_BORDER_SIZE", ImGuiStyleVar_TabBarBorderSize)
        .value("TAB_BAR_OVERLINE_SIZE", ImGuiStyleVar_TabBarOverlineSize)
        .value("TABLE_ANGLED_HEADERS_ANGLE", ImGuiStyleVar_TableAngledHeadersAngle)
        .value("TABLE_ANGLED_HEADERS_TEXT_ALIGN", ImGuiStyleVar_TableAngledHeadersTextAlign)
        .value("BUTTON_TEXT_ALIGN", ImGuiStyleVar_ButtonTextAlign)
        .value("SELECTABLE_TEXT_ALIGN", ImGuiStyleVar_SelectableTextAlign)
        .value("SEPARATOR_TEXT_BORDER_SIZE", ImGuiStyleVar_SeparatorTextBorderSize)
        .value("SEPARATOR_TEXT_ALIGN", ImGuiStyleVar_SeparatorTextAlign)
        .value("SEPARATOR_TEXT_PADDING", ImGuiStyleVar_SeparatorTextPadding)
        .value("DOCKING_SEPARATOR_SIZE", ImGuiStyleVar_DockingSeparatorSize)
        .value("COUNT", ImGuiStyleVar_COUNT)
        .export_values();

    py::enum_<ImGuiButtonFlags_>(gui, "ButtonFlags", py::arithmetic())
        .value("NONE", ImGuiButtonFlags_None)
        .value("MOUSE_BUTTON_LEFT", ImGuiButtonFlags_MouseButtonLeft)
        .value("MOUSE_BUTTON_RIGHT", ImGuiButtonFlags_MouseButtonRight)
        .value("MOUSE_BUTTON_MIDDLE", ImGuiButtonFlags_MouseButtonMiddle)
        .value("MOUSE_BUTTON_MASK", ImGuiButtonFlags_MouseButtonMask_)
        .export_values();

    py::enum_<ImGuiColorEditFlags_>(gui, "ColorEditFlags", py::arithmetic())
        .value("NONE", ImGuiColorEditFlags_None)
        .value("NO_ALPHA", ImGuiColorEditFlags_NoAlpha)
        .value("NO_PICKER", ImGuiColorEditFlags_NoPicker)
        .value("NO_OPTIONS", ImGuiColorEditFlags_NoOptions)
        .value("NO_SMALL_PREVIEW", ImGuiColorEditFlags_NoSmallPreview)
        .value("NO_INPUTS", ImGuiColorEditFlags_NoInputs)
        .value("NO_TOOLTIP", ImGuiColorEditFlags_NoTooltip)
        .value("NO_LABEL", ImGuiColorEditFlags_NoLabel)
        .value("NO_SIDE_PREVIEW", ImGuiColorEditFlags_NoSidePreview)
        .value("NO_DRAG_DROP", ImGuiColorEditFlags_NoDragDrop)
        .value("NO_BORDER", ImGuiColorEditFlags_NoBorder)
        .value("ALPHA_BAR", ImGuiColorEditFlags_AlphaBar)
        .value("ALPHA_PREVIEW", ImGuiColorEditFlags_AlphaPreview)
        .value("ALPHA_PREVIEW_HALF", ImGuiColorEditFlags_AlphaPreviewHalf)
        .value("HDR", ImGuiColorEditFlags_HDR)
        .value("DISPLAY_RGB", ImGuiColorEditFlags_DisplayRGB)
        .value("DISPLAY_HSV", ImGuiColorEditFlags_DisplayHSV)
        .value("DISPLAY_HEX", ImGuiColorEditFlags_DisplayHex)
        .value("UINT8", ImGuiColorEditFlags_Uint8)
        .value("FLOAT", ImGuiColorEditFlags_Float)
        .value("PICKER_HUE_BAR", ImGuiColorEditFlags_PickerHueBar)
        .value("PICKER_HUE_WHEEL", ImGuiColorEditFlags_PickerHueWheel)
        .value("INPUT_RGB", ImGuiColorEditFlags_InputRGB)
        .value("INPUT_HSV", ImGuiColorEditFlags_InputHSV)
        .value("DEFAULT_OPTIONS", ImGuiColorEditFlags_DefaultOptions_)
        .value("DISPLAY_MASK", ImGuiColorEditFlags_DisplayMask_)
        .value("DATA_TYPE_MASK", ImGuiColorEditFlags_DataTypeMask_)
        .value("PICKER_MASK", ImGuiColorEditFlags_PickerMask_)
        .value("INPUT_MASK", ImGuiColorEditFlags_InputMask_)
        .export_values();

    py::enum_<ImGuiSliderFlags_>(gui, "SliderFlags", py::arithmetic())
        .value("NONE", ImGuiSliderFlags_None)
        .value("LOGARITHMIC", ImGuiSliderFlags_Logarithmic)
        .value("NO_ROUND_TO_FORMAT", ImGuiSliderFlags_NoRoundToFormat)
        .value("NO_INPUT", ImGuiSliderFlags_NoInput)
        .value("WRAP_AROUND", ImGuiSliderFlags_WrapAround)
        .value("CLAMP_ON_INPUT", ImGuiSliderFlags_ClampOnInput)
        .value("CLAMP_ZERO_RANGE", ImGuiSliderFlags_ClampZeroRange)
        .value("ALWAYS_CLAMP", ImGuiSliderFlags_AlwaysClamp)
        .value("INVALID_MASK", ImGuiSliderFlags_InvalidMask_)
        .export_values();

    py::enum_<ImGuiMouseButton_>(gui, "MouseButton", py::arithmetic())
        .value("LEFT", ImGuiMouseButton_Left)
        .value("RIGHT", ImGuiMouseButton_Right)
        .value("MIDDLE", ImGuiMouseButton_Middle)
        .value("COUNT", ImGuiMouseButton_COUNT)
        .export_values();

    py::enum_<ImGuiMouseCursor_>(gui, "MouseCursor", py::arithmetic())
        .value("NONE", ImGuiMouseCursor_None)
        .value("ARROW", ImGuiMouseCursor_Arrow)
        .value("TEXT_INPUT", ImGuiMouseCursor_TextInput)
        .value("RESIZE_ALL", ImGuiMouseCursor_ResizeAll)
        .value("RESIZE_NS", ImGuiMouseCursor_ResizeNS)
        .value("RESIZE_EW", ImGuiMouseCursor_ResizeEW)
        .value("RESIZE_NESW", ImGuiMouseCursor_ResizeNESW)
        .value("RESIZE_NWSE", ImGuiMouseCursor_ResizeNWSE)
        .value("HAND", ImGuiMouseCursor_Hand)
        .value("NOT_ALLOWED", ImGuiMouseCursor_NotAllowed)
        .value("COUNT", ImGuiMouseCursor_COUNT)
        .export_values();

    py::enum_<ImGuiMouseSource>(gui, "MouseSource", py::arithmetic())
        .value("MOUSE", ImGuiMouseSource_Mouse)
        .value("TOUCH_SCREEN", ImGuiMouseSource_TouchScreen)
        .value("PEN", ImGuiMouseSource_Pen)
        .value("COUNT", ImGuiMouseSource_COUNT)
        .export_values();

    py::enum_<ImGuiCond_>(gui, "Cond", py::arithmetic())
        .value("NONE", ImGuiCond_None)
        .value("ALWAYS", ImGuiCond_Always)
        .value("ONCE", ImGuiCond_Once)
        .value("FIRST_USE_EVER", ImGuiCond_FirstUseEver)
        .value("APPEARING", ImGuiCond_Appearing)
        .export_values();

    py::enum_<ImGuiTableFlags_>(gui, "TableFlags", py::arithmetic())
        .value("NONE", ImGuiTableFlags_None)
        .value("RESIZABLE", ImGuiTableFlags_Resizable)
        .value("REORDERABLE", ImGuiTableFlags_Reorderable)
        .value("HIDEABLE", ImGuiTableFlags_Hideable)
        .value("SORTABLE", ImGuiTableFlags_Sortable)
        .value("NO_SAVED_SETTINGS", ImGuiTableFlags_NoSavedSettings)
        .value("CONTEXT_MENU_IN_BODY", ImGuiTableFlags_ContextMenuInBody)
        .value("ROW_BG", ImGuiTableFlags_RowBg)
        .value("BORDERS_INNER_H", ImGuiTableFlags_BordersInnerH)
        .value("BORDERS_OUTER_H", ImGuiTableFlags_BordersOuterH)
        .value("BORDERS_INNER_V", ImGuiTableFlags_BordersInnerV)
        .value("BORDERS_OUTER_V", ImGuiTableFlags_BordersOuterV)
        .value("BORDERS_H", ImGuiTableFlags_BordersH)
        .value("BORDERS_V", ImGuiTableFlags_BordersV)
        .value("BORDERS_INNER", ImGuiTableFlags_BordersInner)
        .value("BORDERS_OUTER", ImGuiTableFlags_BordersOuter)
        .value("BORDERS", ImGuiTableFlags_Borders)
        .value("NO_BORDERS_IN_BODY", ImGuiTableFlags_NoBordersInBody)
        .value("NO_BORDERS_IN_BODY_UNTIL_RESIZE", ImGuiTableFlags_NoBordersInBodyUntilResize)
        .value("SIZING_FIXED_FIT", ImGuiTableFlags_SizingFixedFit)
        .value("SIZING_FIXED_SAME", ImGuiTableFlags_SizingFixedSame)
        .value("SIZING_STRETCH_PROP", ImGuiTableFlags_SizingStretchProp)
        .value("SIZING_STRETCH_SAME", ImGuiTableFlags_SizingStretchSame)
        .value("NO_HOST_EXTEND_X", ImGuiTableFlags_NoHostExtendX)
        .value("NO_HOST_EXTEND_Y", ImGuiTableFlags_NoHostExtendY)
        .value("NO_KEEP_COLUMNS_VISIBLE", ImGuiTableFlags_NoKeepColumnsVisible)
        .value("PRECISE_WIDTHS", ImGuiTableFlags_PreciseWidths)
        .value("NO_CLIP", ImGuiTableFlags_NoClip)
        .value("PAD_OUTER_X", ImGuiTableFlags_PadOuterX)
        .value("NO_PAD_OUTER_X", ImGuiTableFlags_NoPadOuterX)
        .value("NO_PAD_INNER_X", ImGuiTableFlags_NoPadInnerX)
        .value("SCROLL_X", ImGuiTableFlags_ScrollX)
        .value("SCROLL_Y", ImGuiTableFlags_ScrollY)
        .value("SORT_MULTI", ImGuiTableFlags_SortMulti)
        .value("SORT_TRISTATE", ImGuiTableFlags_SortTristate)
        .value("HIGHLIGHT_HOVERED_COLUMN", ImGuiTableFlags_HighlightHoveredColumn)
        .value("SIZING_MASK", ImGuiTableFlags_SizingMask_)
        .export_values();

    py::enum_<ImGuiTableColumnFlags_>(gui, "TableColumnFlags", py::arithmetic())
        .value("NONE", ImGuiTableColumnFlags_None)
        .value("DISABLED", ImGuiTableColumnFlags_Disabled)
        .value("DEFAULT_HIDE", ImGuiTableColumnFlags_DefaultHide)
        .value("DEFAULT_SORT", ImGuiTableColumnFlags_DefaultSort)
        .value("WIDTH_STRETCH", ImGuiTableColumnFlags_WidthStretch)
        .value("WIDTH_FIXED", ImGuiTableColumnFlags_WidthFixed)
        .value("NO_RESIZE", ImGuiTableColumnFlags_NoResize)
        .value("NO_REORDER", ImGuiTableColumnFlags_NoReorder)
        .value("NO_HIDE", ImGuiTableColumnFlags_NoHide)
        .value("NO_CLIP", ImGuiTableColumnFlags_NoClip)
        .value("NO_SORT", ImGuiTableColumnFlags_NoSort)
        .value("NO_SORT_ASCENDING", ImGuiTableColumnFlags_NoSortAscending)
        .value("NO_SORT_DESCENDING", ImGuiTableColumnFlags_NoSortDescending)
        .value("NO_HEADER_LABEL", ImGuiTableColumnFlags_NoHeaderLabel)
        .value("NO_HEADER_WIDTH", ImGuiTableColumnFlags_NoHeaderWidth)
        .value("PREFER_SORT_ASCENDING", ImGuiTableColumnFlags_PreferSortAscending)
        .value("PREFER_SORT_DESCENDING", ImGuiTableColumnFlags_PreferSortDescending)
        .value("INDENT_ENABLE", ImGuiTableColumnFlags_IndentEnable)
        .value("INDENT_DISABLE", ImGuiTableColumnFlags_IndentDisable)
        .value("ANGLED_HEADER", ImGuiTableColumnFlags_AngledHeader)
        .value("IS_ENABLED", ImGuiTableColumnFlags_IsEnabled)
        .value("IS_VISIBLE", ImGuiTableColumnFlags_IsVisible)
        .value("IS_SORTED", ImGuiTableColumnFlags_IsSorted)
        .value("IS_HOVERED", ImGuiTableColumnFlags_IsHovered)
        .value("WIDTH_MASK", ImGuiTableColumnFlags_WidthMask_)
        .value("INDENT_MASK", ImGuiTableColumnFlags_IndentMask_)
        .value("STATUS_MASK", ImGuiTableColumnFlags_StatusMask_)
        .value("NO_DIRECT_RESIZE", ImGuiTableColumnFlags_NoDirectResize_)
        .export_values();

    py::enum_<ImGuiTableRowFlags_>(gui, "TableRowFlags", py::arithmetic())
        .value("NONE", ImGuiTableRowFlags_None)
        .value("HEADERS", ImGuiTableRowFlags_Headers)
        .export_values();

    py::enum_<ImGuiTableBgTarget_>(gui, "TableBgTarget", py::arithmetic())
        .value("NONE", ImGuiTableBgTarget_None)
        .value("ROW_BG0", ImGuiTableBgTarget_RowBg0)
        .value("ROW_BG1", ImGuiTableBgTarget_RowBg1)
        .value("CELL_BG", ImGuiTableBgTarget_CellBg)
        .export_values();

    py::class_<ImGuiTableSortSpecs> TableSortSpecs(gui, "TableSortSpecs");
    TableSortSpecs.def_readwrite("specs", &ImGuiTableSortSpecs::Specs);
    TableSortSpecs.def_readwrite("specs_count", &ImGuiTableSortSpecs::SpecsCount);
    TableSortSpecs.def_readwrite("specs_dirty", &ImGuiTableSortSpecs::SpecsDirty);
    TableSortSpecs.def(py::init<>());
    py::class_<ImGuiTableColumnSortSpecs> TableColumnSortSpecs(gui, "TableColumnSortSpecs");
    TableColumnSortSpecs.def_readwrite("column_user_id", &ImGuiTableColumnSortSpecs::ColumnUserID);
    TableColumnSortSpecs.def_readwrite("column_index", &ImGuiTableColumnSortSpecs::ColumnIndex);
    TableColumnSortSpecs.def_readwrite("sort_order", &ImGuiTableColumnSortSpecs::SortOrder);
    TableColumnSortSpecs.def_readwrite("sort_direction", &ImGuiTableColumnSortSpecs::SortDirection);
    TableColumnSortSpecs.def(py::init<>());
    py::class_<ImNewWrapper> NewWrapper(gui, "NewWrapper");
    py::class_<ImGuiStyle> Style(gui, "Style");
    Style.def_readwrite("alpha", &ImGuiStyle::Alpha);
    Style.def_readwrite("disabled_alpha", &ImGuiStyle::DisabledAlpha);
    Style.def_readwrite("window_padding", &ImGuiStyle::WindowPadding);
    Style.def_readwrite("window_rounding", &ImGuiStyle::WindowRounding);
    Style.def_readwrite("window_border_size", &ImGuiStyle::WindowBorderSize);
    Style.def_readwrite("window_min_size", &ImGuiStyle::WindowMinSize);
    Style.def_readwrite("window_title_align", &ImGuiStyle::WindowTitleAlign);
    Style.def_readwrite("window_menu_button_position", &ImGuiStyle::WindowMenuButtonPosition);
    Style.def_readwrite("child_rounding", &ImGuiStyle::ChildRounding);
    Style.def_readwrite("child_border_size", &ImGuiStyle::ChildBorderSize);
    Style.def_readwrite("popup_rounding", &ImGuiStyle::PopupRounding);
    Style.def_readwrite("popup_border_size", &ImGuiStyle::PopupBorderSize);
    Style.def_readwrite("frame_padding", &ImGuiStyle::FramePadding);
    Style.def_readwrite("frame_rounding", &ImGuiStyle::FrameRounding);
    Style.def_readwrite("frame_border_size", &ImGuiStyle::FrameBorderSize);
    Style.def_readwrite("item_spacing", &ImGuiStyle::ItemSpacing);
    Style.def_readwrite("item_inner_spacing", &ImGuiStyle::ItemInnerSpacing);
    Style.def_readwrite("cell_padding", &ImGuiStyle::CellPadding);
    Style.def_readwrite("touch_extra_padding", &ImGuiStyle::TouchExtraPadding);
    Style.def_readwrite("indent_spacing", &ImGuiStyle::IndentSpacing);
    Style.def_readwrite("columns_min_spacing", &ImGuiStyle::ColumnsMinSpacing);
    Style.def_readwrite("scrollbar_size", &ImGuiStyle::ScrollbarSize);
    Style.def_readwrite("scrollbar_rounding", &ImGuiStyle::ScrollbarRounding);
    Style.def_readwrite("grab_min_size", &ImGuiStyle::GrabMinSize);
    Style.def_readwrite("grab_rounding", &ImGuiStyle::GrabRounding);
    Style.def_readwrite("log_slider_deadzone", &ImGuiStyle::LogSliderDeadzone);
    Style.def_readwrite("tab_rounding", &ImGuiStyle::TabRounding);
    Style.def_readwrite("tab_border_size", &ImGuiStyle::TabBorderSize);
    Style.def_readwrite("tab_min_width_for_close_button", &ImGuiStyle::TabMinWidthForCloseButton);
    Style.def_readwrite("tab_bar_border_size", &ImGuiStyle::TabBarBorderSize);
    Style.def_readwrite("tab_bar_overline_size", &ImGuiStyle::TabBarOverlineSize);
    Style.def_readwrite("table_angled_headers_angle", &ImGuiStyle::TableAngledHeadersAngle);
    Style.def_readwrite("table_angled_headers_text_align", &ImGuiStyle::TableAngledHeadersTextAlign);
    Style.def_readwrite("color_button_position", &ImGuiStyle::ColorButtonPosition);
    Style.def_readwrite("button_text_align", &ImGuiStyle::ButtonTextAlign);
    Style.def_readwrite("selectable_text_align", &ImGuiStyle::SelectableTextAlign);
    Style.def_readwrite("separator_text_border_size", &ImGuiStyle::SeparatorTextBorderSize);
    Style.def_readwrite("separator_text_align", &ImGuiStyle::SeparatorTextAlign);
    Style.def_readwrite("separator_text_padding", &ImGuiStyle::SeparatorTextPadding);
    Style.def_readwrite("display_window_padding", &ImGuiStyle::DisplayWindowPadding);
    Style.def_readwrite("display_safe_area_padding", &ImGuiStyle::DisplaySafeAreaPadding);
    Style.def_readwrite("docking_separator_size", &ImGuiStyle::DockingSeparatorSize);
    Style.def_readwrite("mouse_cursor_scale", &ImGuiStyle::MouseCursorScale);
    Style.def_readwrite("anti_aliased_lines", &ImGuiStyle::AntiAliasedLines);
    Style.def_readwrite("anti_aliased_lines_use_tex", &ImGuiStyle::AntiAliasedLinesUseTex);
    Style.def_readwrite("anti_aliased_fill", &ImGuiStyle::AntiAliasedFill);
    Style.def_readwrite("curve_tessellation_tol", &ImGuiStyle::CurveTessellationTol);
    Style.def_readwrite("circle_tessellation_max_error", &ImGuiStyle::CircleTessellationMaxError);
    Style.def_readonly("colors", &ImGuiStyle::Colors);
    Style.def_readwrite("hover_stationary_delay", &ImGuiStyle::HoverStationaryDelay);
    Style.def_readwrite("hover_delay_short", &ImGuiStyle::HoverDelayShort);
    Style.def_readwrite("hover_delay_normal", &ImGuiStyle::HoverDelayNormal);
    Style.def_readwrite("hover_flags_for_tooltip_mouse", &ImGuiStyle::HoverFlagsForTooltipMouse);
    Style.def_readwrite("hover_flags_for_tooltip_nav", &ImGuiStyle::HoverFlagsForTooltipNav);
    Style.def(py::init<>());
    Style.def("scale_all_sizes", &ImGuiStyle::ScaleAllSizes
    , py::arg("scale_factor")
    , py::return_value_policy::automatic_reference);
    py::class_<ImGuiKeyData> KeyData(gui, "KeyData");
    KeyData.def_readwrite("down", &ImGuiKeyData::Down);
    KeyData.def_readwrite("down_duration", &ImGuiKeyData::DownDuration);
    KeyData.def_readwrite("down_duration_prev", &ImGuiKeyData::DownDurationPrev);
    KeyData.def_readwrite("analog_value", &ImGuiKeyData::AnalogValue);
    py::class_<ImGuiIO> IO(gui, "IO");
    IO.def_readwrite("config_flags", &ImGuiIO::ConfigFlags);
    IO.def_readwrite("backend_flags", &ImGuiIO::BackendFlags);
    IO.def_readwrite("display_size", &ImGuiIO::DisplaySize);
    IO.def_readwrite("delta_time", &ImGuiIO::DeltaTime);
    IO.def_readwrite("ini_saving_rate", &ImGuiIO::IniSavingRate);
    IO.def_readwrite("ini_filename", &ImGuiIO::IniFilename);
    IO.def_readwrite("log_filename", &ImGuiIO::LogFilename);
    IO.def_readwrite("user_data", &ImGuiIO::UserData);
    IO.def_readwrite("fonts", &ImGuiIO::Fonts);
    IO.def_readwrite("font_global_scale", &ImGuiIO::FontGlobalScale);
    IO.def_readwrite("font_allow_user_scaling", &ImGuiIO::FontAllowUserScaling);
    IO.def_readwrite("font_default", &ImGuiIO::FontDefault);
    IO.def_readwrite("display_framebuffer_scale", &ImGuiIO::DisplayFramebufferScale);
    IO.def_readwrite("config_docking_no_split", &ImGuiIO::ConfigDockingNoSplit);
    IO.def_readwrite("config_docking_with_shift", &ImGuiIO::ConfigDockingWithShift);
    IO.def_readwrite("config_docking_always_tab_bar", &ImGuiIO::ConfigDockingAlwaysTabBar);
    IO.def_readwrite("config_docking_transparent_payload", &ImGuiIO::ConfigDockingTransparentPayload);
    IO.def_readwrite("config_viewports_no_auto_merge", &ImGuiIO::ConfigViewportsNoAutoMerge);
    IO.def_readwrite("config_viewports_no_task_bar_icon", &ImGuiIO::ConfigViewportsNoTaskBarIcon);
    IO.def_readwrite("config_viewports_no_decoration", &ImGuiIO::ConfigViewportsNoDecoration);
    IO.def_readwrite("config_viewports_no_default_parent", &ImGuiIO::ConfigViewportsNoDefaultParent);
    IO.def_readwrite("mouse_draw_cursor", &ImGuiIO::MouseDrawCursor);
    IO.def_readwrite("config_mac_osx_behaviors", &ImGuiIO::ConfigMacOSXBehaviors);
    IO.def_readwrite("config_nav_swap_gamepad_buttons", &ImGuiIO::ConfigNavSwapGamepadButtons);
    IO.def_readwrite("config_input_trickle_event_queue", &ImGuiIO::ConfigInputTrickleEventQueue);
    IO.def_readwrite("config_input_text_cursor_blink", &ImGuiIO::ConfigInputTextCursorBlink);
    IO.def_readwrite("config_input_text_enter_keep_active", &ImGuiIO::ConfigInputTextEnterKeepActive);
    IO.def_readwrite("config_drag_click_to_input_text", &ImGuiIO::ConfigDragClickToInputText);
    IO.def_readwrite("config_windows_resize_from_edges", &ImGuiIO::ConfigWindowsResizeFromEdges);
    IO.def_readwrite("config_windows_move_from_title_bar_only", &ImGuiIO::ConfigWindowsMoveFromTitleBarOnly);
    IO.def_readwrite("config_scrollbar_scroll_by_page", &ImGuiIO::ConfigScrollbarScrollByPage);
    IO.def_readwrite("config_memory_compact_timer", &ImGuiIO::ConfigMemoryCompactTimer);
    IO.def_readwrite("mouse_double_click_time", &ImGuiIO::MouseDoubleClickTime);
    IO.def_readwrite("mouse_double_click_max_dist", &ImGuiIO::MouseDoubleClickMaxDist);
    IO.def_readwrite("mouse_drag_threshold", &ImGuiIO::MouseDragThreshold);
    IO.def_readwrite("key_repeat_delay", &ImGuiIO::KeyRepeatDelay);
    IO.def_readwrite("key_repeat_rate", &ImGuiIO::KeyRepeatRate);
    IO.def_readwrite("config_error_recovery", &ImGuiIO::ConfigErrorRecovery);
    IO.def_readwrite("config_error_recovery_enable_assert", &ImGuiIO::ConfigErrorRecoveryEnableAssert);
    IO.def_readwrite("config_error_recovery_enable_debug_log", &ImGuiIO::ConfigErrorRecoveryEnableDebugLog);
    IO.def_readwrite("config_error_recovery_enable_tooltip", &ImGuiIO::ConfigErrorRecoveryEnableTooltip);
    IO.def_readwrite("config_debug_is_debugger_present", &ImGuiIO::ConfigDebugIsDebuggerPresent);
    IO.def_readwrite("config_debug_highlight_id_conflicts", &ImGuiIO::ConfigDebugHighlightIdConflicts);
    IO.def_readwrite("config_debug_begin_return_value_once", &ImGuiIO::ConfigDebugBeginReturnValueOnce);
    IO.def_readwrite("config_debug_begin_return_value_loop", &ImGuiIO::ConfigDebugBeginReturnValueLoop);
    IO.def_readwrite("config_debug_ignore_focus_loss", &ImGuiIO::ConfigDebugIgnoreFocusLoss);
    IO.def_readwrite("config_debug_ini_settings", &ImGuiIO::ConfigDebugIniSettings);
    IO.def_readwrite("backend_platform_name", &ImGuiIO::BackendPlatformName);
    IO.def_readwrite("backend_renderer_name", &ImGuiIO::BackendRendererName);
    IO.def_readwrite("backend_platform_user_data", &ImGuiIO::BackendPlatformUserData);
    IO.def_readwrite("backend_renderer_user_data", &ImGuiIO::BackendRendererUserData);
    IO.def_readwrite("backend_language_user_data", &ImGuiIO::BackendLanguageUserData);
    IO.def("add_key_event", &ImGuiIO::AddKeyEvent
    , py::arg("key")
    , py::arg("down")
    , py::return_value_policy::automatic_reference);
    IO.def("add_key_analog_event", &ImGuiIO::AddKeyAnalogEvent
    , py::arg("key")
    , py::arg("down")
    , py::arg("v")
    , py::return_value_policy::automatic_reference);
    IO.def("add_mouse_pos_event", &ImGuiIO::AddMousePosEvent
    , py::arg("x")
    , py::arg("y")
    , py::return_value_policy::automatic_reference);
    IO.def("add_mouse_button_event", &ImGuiIO::AddMouseButtonEvent
    , py::arg("button")
    , py::arg("down")
    , py::return_value_policy::automatic_reference);
    IO.def("add_mouse_wheel_event", &ImGuiIO::AddMouseWheelEvent
    , py::arg("wheel_x")
    , py::arg("wheel_y")
    , py::return_value_policy::automatic_reference);
    IO.def("add_mouse_source_event", &ImGuiIO::AddMouseSourceEvent
    , py::arg("source")
    , py::return_value_policy::automatic_reference);
    IO.def("add_mouse_viewport_event", &ImGuiIO::AddMouseViewportEvent
    , py::arg("id")
    , py::return_value_policy::automatic_reference);
    IO.def("add_focus_event", &ImGuiIO::AddFocusEvent
    , py::arg("focused")
    , py::return_value_policy::automatic_reference);
    IO.def("add_input_character", &ImGuiIO::AddInputCharacter
    , py::arg("c")
    , py::return_value_policy::automatic_reference);
    IO.def("add_input_character_utf16", &ImGuiIO::AddInputCharacterUTF16
    , py::arg("c")
    , py::return_value_policy::automatic_reference);
    IO.def("add_input_characters_utf8", &ImGuiIO::AddInputCharactersUTF8
    , py::arg("str")
    , py::return_value_policy::automatic_reference);
    IO.def("set_key_event_native_data", &ImGuiIO::SetKeyEventNativeData
    , py::arg("key")
    , py::arg("native_keycode")
    , py::arg("native_scancode")
    , py::arg("native_legacy_index") = -1
    , py::return_value_policy::automatic_reference);
    IO.def("set_app_accepting_events", &ImGuiIO::SetAppAcceptingEvents
    , py::arg("accepting_events")
    , py::return_value_policy::automatic_reference);
    IO.def("clear_events_queue", &ImGuiIO::ClearEventsQueue
    , py::return_value_policy::automatic_reference);
    IO.def("clear_input_keys", &ImGuiIO::ClearInputKeys
    , py::return_value_policy::automatic_reference);
    IO.def("clear_input_mouse", &ImGuiIO::ClearInputMouse
    , py::return_value_policy::automatic_reference);
    IO.def_readwrite("want_capture_mouse", &ImGuiIO::WantCaptureMouse);
    IO.def_readwrite("want_capture_keyboard", &ImGuiIO::WantCaptureKeyboard);
    IO.def_readwrite("want_text_input", &ImGuiIO::WantTextInput);
    IO.def_readwrite("want_set_mouse_pos", &ImGuiIO::WantSetMousePos);
    IO.def_readwrite("want_save_ini_settings", &ImGuiIO::WantSaveIniSettings);
    IO.def_readwrite("nav_active", &ImGuiIO::NavActive);
    IO.def_readwrite("nav_visible", &ImGuiIO::NavVisible);
    IO.def_readwrite("framerate", &ImGuiIO::Framerate);
    IO.def_readwrite("metrics_render_vertices", &ImGuiIO::MetricsRenderVertices);
    IO.def_readwrite("metrics_render_indices", &ImGuiIO::MetricsRenderIndices);
    IO.def_readwrite("metrics_render_windows", &ImGuiIO::MetricsRenderWindows);
    IO.def_readwrite("metrics_active_windows", &ImGuiIO::MetricsActiveWindows);
    IO.def_readwrite("mouse_delta", &ImGuiIO::MouseDelta);
    IO.def_readwrite("mouse_pos", &ImGuiIO::MousePos);
    IO.def_readwrite("mouse_wheel", &ImGuiIO::MouseWheel);
    IO.def_readwrite("mouse_wheel_h", &ImGuiIO::MouseWheelH);
    IO.def_readwrite("mouse_source", &ImGuiIO::MouseSource);
    IO.def_readwrite("mouse_hovered_viewport", &ImGuiIO::MouseHoveredViewport);
    IO.def_readwrite("key_ctrl", &ImGuiIO::KeyCtrl);
    IO.def_readwrite("key_shift", &ImGuiIO::KeyShift);
    IO.def_readwrite("key_alt", &ImGuiIO::KeyAlt);
    IO.def_readwrite("key_super", &ImGuiIO::KeySuper);
    IO.def_readwrite("key_mods", &ImGuiIO::KeyMods);
    IO.def_readonly("keys_data", &ImGuiIO::KeysData);
    IO.def_readwrite("want_capture_mouse_unless_popup_close", &ImGuiIO::WantCaptureMouseUnlessPopupClose);
    IO.def_readwrite("mouse_pos_prev", &ImGuiIO::MousePosPrev);
    IO.def_readonly("mouse_clicked_pos", &ImGuiIO::MouseClickedPos);
    IO.def_readonly("mouse_clicked_time", &ImGuiIO::MouseClickedTime);
    IO.def_readonly("mouse_clicked", &ImGuiIO::MouseClicked);
    IO.def_readonly("mouse_double_clicked", &ImGuiIO::MouseDoubleClicked);
    IO.def_readonly("mouse_clicked_count", &ImGuiIO::MouseClickedCount);
    IO.def_readonly("mouse_clicked_last_count", &ImGuiIO::MouseClickedLastCount);
    IO.def_readonly("mouse_released", &ImGuiIO::MouseReleased);
    IO.def_readonly("mouse_down_owned", &ImGuiIO::MouseDownOwned);
    IO.def_readonly("mouse_down_owned_unless_popup_close", &ImGuiIO::MouseDownOwnedUnlessPopupClose);
    IO.def_readwrite("mouse_wheel_request_axis_swap", &ImGuiIO::MouseWheelRequestAxisSwap);
    IO.def_readwrite("mouse_ctrl_left_as_right_click", &ImGuiIO::MouseCtrlLeftAsRightClick);
    IO.def_readonly("mouse_down_duration", &ImGuiIO::MouseDownDuration);
    IO.def_readonly("mouse_down_duration_prev", &ImGuiIO::MouseDownDurationPrev);
    IO.def_readonly("mouse_drag_max_distance_abs", &ImGuiIO::MouseDragMaxDistanceAbs);
    IO.def_readonly("mouse_drag_max_distance_sqr", &ImGuiIO::MouseDragMaxDistanceSqr);
    IO.def_readwrite("pen_pressure", &ImGuiIO::PenPressure);
    IO.def_readwrite("app_focus_lost", &ImGuiIO::AppFocusLost);
    IO.def_readwrite("app_accepting_events", &ImGuiIO::AppAcceptingEvents);
    IO.def_readwrite("backend_using_legacy_key_arrays", &ImGuiIO::BackendUsingLegacyKeyArrays);
    IO.def_readwrite("backend_using_legacy_nav_input_array", &ImGuiIO::BackendUsingLegacyNavInputArray);
    IO.def_readwrite("input_queue_surrogate", &ImGuiIO::InputQueueSurrogate);
    IO.def_readwrite("input_queue_characters", &ImGuiIO::InputQueueCharacters);
    IO.def(py::init<>());
    py::class_<ImGuiInputTextCallbackData> InputTextCallbackData(gui, "InputTextCallbackData");
    InputTextCallbackData.def_readwrite("event_flag", &ImGuiInputTextCallbackData::EventFlag);
    InputTextCallbackData.def_readwrite("flags", &ImGuiInputTextCallbackData::Flags);
    InputTextCallbackData.def_readwrite("user_data", &ImGuiInputTextCallbackData::UserData);
    InputTextCallbackData.def_readwrite("event_char", &ImGuiInputTextCallbackData::EventChar);
    InputTextCallbackData.def_readwrite("event_key", &ImGuiInputTextCallbackData::EventKey);
    InputTextCallbackData.def_readwrite("buf", &ImGuiInputTextCallbackData::Buf);
    InputTextCallbackData.def_readwrite("buf_text_len", &ImGuiInputTextCallbackData::BufTextLen);
    InputTextCallbackData.def_readwrite("buf_size", &ImGuiInputTextCallbackData::BufSize);
    InputTextCallbackData.def_readwrite("buf_dirty", &ImGuiInputTextCallbackData::BufDirty);
    InputTextCallbackData.def_readwrite("cursor_pos", &ImGuiInputTextCallbackData::CursorPos);
    InputTextCallbackData.def_readwrite("selection_start", &ImGuiInputTextCallbackData::SelectionStart);
    InputTextCallbackData.def_readwrite("selection_end", &ImGuiInputTextCallbackData::SelectionEnd);
    InputTextCallbackData.def(py::init<>());
    InputTextCallbackData.def("delete_chars", &ImGuiInputTextCallbackData::DeleteChars
    , py::arg("pos")
    , py::arg("bytes_count")
    , py::return_value_policy::automatic_reference);
    InputTextCallbackData.def("insert_chars", &ImGuiInputTextCallbackData::InsertChars
    , py::arg("pos")
    , py::arg("text")
    , py::arg("text_end") = nullptr
    , py::return_value_policy::automatic_reference);
    InputTextCallbackData.def("select_all", &ImGuiInputTextCallbackData::SelectAll
    , py::return_value_policy::automatic_reference);
    InputTextCallbackData.def("clear_selection", &ImGuiInputTextCallbackData::ClearSelection
    , py::return_value_policy::automatic_reference);
    InputTextCallbackData.def("has_selection", &ImGuiInputTextCallbackData::HasSelection
    , py::return_value_policy::automatic_reference);
    py::class_<ImGuiSizeCallbackData> SizeCallbackData(gui, "SizeCallbackData");
    SizeCallbackData.def_readwrite("user_data", &ImGuiSizeCallbackData::UserData);
    SizeCallbackData.def_readwrite("pos", &ImGuiSizeCallbackData::Pos);
    SizeCallbackData.def_readwrite("current_size", &ImGuiSizeCallbackData::CurrentSize);
    SizeCallbackData.def_readwrite("desired_size", &ImGuiSizeCallbackData::DesiredSize);
    py::class_<ImGuiWindowClass> WindowClass(gui, "WindowClass");
    WindowClass.def_readwrite("class_id", &ImGuiWindowClass::ClassId);
    WindowClass.def_readwrite("parent_viewport_id", &ImGuiWindowClass::ParentViewportId);
    WindowClass.def_readwrite("focus_route_parent_window_id", &ImGuiWindowClass::FocusRouteParentWindowId);
    WindowClass.def_readwrite("viewport_flags_override_set", &ImGuiWindowClass::ViewportFlagsOverrideSet);
    WindowClass.def_readwrite("viewport_flags_override_clear", &ImGuiWindowClass::ViewportFlagsOverrideClear);
    WindowClass.def_readwrite("tab_item_flags_override_set", &ImGuiWindowClass::TabItemFlagsOverrideSet);
    WindowClass.def_readwrite("dock_node_flags_override_set", &ImGuiWindowClass::DockNodeFlagsOverrideSet);
    WindowClass.def_readwrite("docking_always_tab_bar", &ImGuiWindowClass::DockingAlwaysTabBar);
    WindowClass.def_readwrite("docking_allow_unclassed", &ImGuiWindowClass::DockingAllowUnclassed);
    WindowClass.def(py::init<>());
    py::class_<ImGuiPayload> Payload(gui, "Payload");
    Payload.def_readwrite("data", &ImGuiPayload::Data);
    Payload.def_readwrite("data_size", &ImGuiPayload::DataSize);
    Payload.def_readwrite("source_id", &ImGuiPayload::SourceId);
    Payload.def_readwrite("source_parent_id", &ImGuiPayload::SourceParentId);
    Payload.def_readwrite("data_frame_count", &ImGuiPayload::DataFrameCount);
    Payload.def_readonly("data_type", &ImGuiPayload::DataType);
    Payload.def_readwrite("preview", &ImGuiPayload::Preview);
    Payload.def_readwrite("delivery", &ImGuiPayload::Delivery);
    Payload.def(py::init<>());
    Payload.def("clear", &ImGuiPayload::Clear
    , py::return_value_policy::automatic_reference);
    Payload.def("is_data_type", &ImGuiPayload::IsDataType
    , py::arg("type")
    , py::return_value_policy::automatic_reference);
    Payload.def("is_preview", &ImGuiPayload::IsPreview
    , py::return_value_policy::automatic_reference);
    Payload.def("is_delivery", &ImGuiPayload::IsDelivery
    , py::return_value_policy::automatic_reference);
    py::class_<ImGuiOnceUponAFrame> OnceUponAFrame(gui, "OnceUponAFrame");
    OnceUponAFrame.def(py::init<>());
    OnceUponAFrame.def_readwrite("ref_frame", &ImGuiOnceUponAFrame::RefFrame);
    py::class_<ImGuiTextFilter> TextFilter(gui, "TextFilter");
    TextFilter.def(py::init<const char *>()
    , py::arg("default_filter") = nullptr
    );
    TextFilter.def("draw", &ImGuiTextFilter::Draw
    , py::arg("label") = nullptr
    , py::arg("width") = 0.0f
    , py::return_value_policy::automatic_reference);
    TextFilter.def("pass_filter", &ImGuiTextFilter::PassFilter
    , py::arg("text")
    , py::arg("text_end") = nullptr
    , py::return_value_policy::automatic_reference);
    TextFilter.def("build", &ImGuiTextFilter::Build
    , py::return_value_policy::automatic_reference);
    TextFilter.def("clear", &ImGuiTextFilter::Clear
    , py::return_value_policy::automatic_reference);
    TextFilter.def("is_active", &ImGuiTextFilter::IsActive
    , py::return_value_policy::automatic_reference);
    TextFilter.def_readonly("input_buf", &ImGuiTextFilter::InputBuf);
    TextFilter.def_readwrite("count_grep", &ImGuiTextFilter::CountGrep);
    py::class_<ImGuiStoragePair> StoragePair(gui, "StoragePair");
    StoragePair.def_readwrite("key", &ImGuiStoragePair::key);
    StoragePair.def(py::init<ImGuiID, int>()
    , py::arg("_key")
    , py::arg("_val")
    );
    StoragePair.def(py::init<ImGuiID, float>()
    , py::arg("_key")
    , py::arg("_val")
    );
    StoragePair.def(py::init<ImGuiID, void *>()
    , py::arg("_key")
    , py::arg("_val")
    );
    py::class_<ImGuiStorage> Storage(gui, "Storage");
    Storage.def("clear", &ImGuiStorage::Clear
    , py::return_value_policy::automatic_reference);
    Storage.def("get_int", &ImGuiStorage::GetInt
    , py::arg("key")
    , py::arg("default_val") = 0
    , py::return_value_policy::automatic_reference);
    Storage.def("set_int", &ImGuiStorage::SetInt
    , py::arg("key")
    , py::arg("val")
    , py::return_value_policy::automatic_reference);
    Storage.def("get_bool", &ImGuiStorage::GetBool
    , py::arg("key")
    , py::arg("default_val") = false
    , py::return_value_policy::automatic_reference);
    Storage.def("set_bool", &ImGuiStorage::SetBool
    , py::arg("key")
    , py::arg("val")
    , py::return_value_policy::automatic_reference);
    Storage.def("get_float", &ImGuiStorage::GetFloat
    , py::arg("key")
    , py::arg("default_val") = 0.0f
    , py::return_value_policy::automatic_reference);
    Storage.def("set_float", &ImGuiStorage::SetFloat
    , py::arg("key")
    , py::arg("val")
    , py::return_value_policy::automatic_reference);
    Storage.def("get_void_ptr", &ImGuiStorage::GetVoidPtr
    , py::arg("key")
    , py::return_value_policy::automatic_reference);
    Storage.def("set_void_ptr", &ImGuiStorage::SetVoidPtr
    , py::arg("key")
    , py::arg("val")
    , py::return_value_policy::automatic_reference);
    Storage.def("get_int_ref", &ImGuiStorage::GetIntRef
    , py::arg("key")
    , py::arg("default_val") = 0
    , py::return_value_policy::automatic_reference);
    Storage.def("get_bool_ref", &ImGuiStorage::GetBoolRef
    , py::arg("key")
    , py::arg("default_val") = false
    , py::return_value_policy::automatic_reference);
    Storage.def("get_float_ref", &ImGuiStorage::GetFloatRef
    , py::arg("key")
    , py::arg("default_val") = 0.0f
    , py::return_value_policy::automatic_reference);
    Storage.def("get_void_ptr_ref", &ImGuiStorage::GetVoidPtrRef
    , py::arg("key")
    , py::arg("default_val") = nullptr
    , py::return_value_policy::automatic_reference);
    Storage.def("build_sort_by_key", &ImGuiStorage::BuildSortByKey
    , py::return_value_policy::automatic_reference);
    Storage.def("set_all_int", &ImGuiStorage::SetAllInt
    , py::arg("val")
    , py::return_value_policy::automatic_reference);
    py::class_<ImGuiListClipper> ListClipper(gui, "ListClipper");
    ListClipper.def_readwrite("display_start", &ImGuiListClipper::DisplayStart);
    ListClipper.def_readwrite("display_end", &ImGuiListClipper::DisplayEnd);
    ListClipper.def_readwrite("items_count", &ImGuiListClipper::ItemsCount);
    ListClipper.def_readwrite("items_height", &ImGuiListClipper::ItemsHeight);
    ListClipper.def_readwrite("start_pos_y", &ImGuiListClipper::StartPosY);
    ListClipper.def_readwrite("start_seek_offset_y", &ImGuiListClipper::StartSeekOffsetY);
    ListClipper.def_readwrite("temp_data", &ImGuiListClipper::TempData);
    ListClipper.def(py::init<>());
    ListClipper.def("begin", &ImGuiListClipper::Begin
    , py::arg("items_count")
    , py::arg("items_height") = -1.0f
    , py::return_value_policy::automatic_reference);
    ListClipper.def("end", &ImGuiListClipper::End
    , py::return_value_policy::automatic_reference);
    ListClipper.def("step", &ImGuiListClipper::Step
    , py::return_value_policy::automatic_reference);
    ListClipper.def("include_item_by_index", &ImGuiListClipper::IncludeItemByIndex
    , py::arg("item_index")
    , py::return_value_policy::automatic_reference);
    ListClipper.def("include_items_by_index", &ImGuiListClipper::IncludeItemsByIndex
    , py::arg("item_begin")
    , py::arg("item_end")
    , py::return_value_policy::automatic_reference);
    ListClipper.def("seek_cursor_for_item", &ImGuiListClipper::SeekCursorForItem
    , py::arg("item_index")
    , py::return_value_policy::automatic_reference);
    py::class_<ImColor> Color(gui, "Color");
    Color.def_readwrite("value", &ImColor::Value);
    Color.def(py::init<>());
    Color.def(py::init<float, float, float, float>()
    , py::arg("r")
    , py::arg("g")
    , py::arg("b")
    , py::arg("a") = 1.0f
    );
    Color.def(py::init<const ImVec4 &>()
    , py::arg("col")
    );
    Color.def(py::init<int, int, int, int>()
    , py::arg("r")
    , py::arg("g")
    , py::arg("b")
    , py::arg("a") = 255
    );
    Color.def(py::init<ImU32>()
    , py::arg("rgba")
    );
    Color.def("set_hsv", &ImColor::SetHSV
    , py::arg("h")
    , py::arg("s")
    , py::arg("v")
    , py::arg("a") = 1.0f
    , py::return_value_policy::automatic_reference);
    py::enum_<ImGuiMultiSelectFlags_>(gui, "MultiSelectFlags", py::arithmetic())
        .value("NONE", ImGuiMultiSelectFlags_None)
        .value("SINGLE_SELECT", ImGuiMultiSelectFlags_SingleSelect)
        .value("NO_SELECT_ALL", ImGuiMultiSelectFlags_NoSelectAll)
        .value("NO_RANGE_SELECT", ImGuiMultiSelectFlags_NoRangeSelect)
        .value("NO_AUTO_SELECT", ImGuiMultiSelectFlags_NoAutoSelect)
        .value("NO_AUTO_CLEAR", ImGuiMultiSelectFlags_NoAutoClear)
        .value("NO_AUTO_CLEAR_ON_RESELECT", ImGuiMultiSelectFlags_NoAutoClearOnReselect)
        .value("BOX_SELECT1D", ImGuiMultiSelectFlags_BoxSelect1d)
        .value("BOX_SELECT2D", ImGuiMultiSelectFlags_BoxSelect2d)
        .value("BOX_SELECT_NO_SCROLL", ImGuiMultiSelectFlags_BoxSelectNoScroll)
        .value("CLEAR_ON_ESCAPE", ImGuiMultiSelectFlags_ClearOnEscape)
        .value("CLEAR_ON_CLICK_VOID", ImGuiMultiSelectFlags_ClearOnClickVoid)
        .value("SCOPE_WINDOW", ImGuiMultiSelectFlags_ScopeWindow)
        .value("SCOPE_RECT", ImGuiMultiSelectFlags_ScopeRect)
        .value("SELECT_ON_CLICK", ImGuiMultiSelectFlags_SelectOnClick)
        .value("SELECT_ON_CLICK_RELEASE", ImGuiMultiSelectFlags_SelectOnClickRelease)
        .value("NAV_WRAP_X", ImGuiMultiSelectFlags_NavWrapX)
        .export_values();

    py::class_<ImGuiMultiSelectIO> MultiSelectIO(gui, "MultiSelectIO");
    MultiSelectIO.def_readwrite("requests", &ImGuiMultiSelectIO::Requests);
    MultiSelectIO.def_readwrite("range_src_item", &ImGuiMultiSelectIO::RangeSrcItem);
    MultiSelectIO.def_readwrite("nav_id_item", &ImGuiMultiSelectIO::NavIdItem);
    MultiSelectIO.def_readwrite("nav_id_selected", &ImGuiMultiSelectIO::NavIdSelected);
    MultiSelectIO.def_readwrite("range_src_reset", &ImGuiMultiSelectIO::RangeSrcReset);
    MultiSelectIO.def_readwrite("items_count", &ImGuiMultiSelectIO::ItemsCount);
    py::enum_<ImGuiSelectionRequestType>(gui, "SelectionRequestType", py::arithmetic())
        .value("NONE", ImGuiSelectionRequestType_None)
        .value("SET_ALL", ImGuiSelectionRequestType_SetAll)
        .value("SET_RANGE", ImGuiSelectionRequestType_SetRange)
        .export_values();

    py::class_<ImGuiSelectionRequest> SelectionRequest(gui, "SelectionRequest");
    SelectionRequest.def_readwrite("type", &ImGuiSelectionRequest::Type);
    SelectionRequest.def_readwrite("selected", &ImGuiSelectionRequest::Selected);
    SelectionRequest.def_readwrite("range_direction", &ImGuiSelectionRequest::RangeDirection);
    SelectionRequest.def_readwrite("range_first_item", &ImGuiSelectionRequest::RangeFirstItem);
    SelectionRequest.def_readwrite("range_last_item", &ImGuiSelectionRequest::RangeLastItem);
    py::class_<ImDrawCmd> DrawCmd(gui, "DrawCmd");
    DrawCmd.def_readwrite("clip_rect", &ImDrawCmd::ClipRect);
    DrawCmd.def_readwrite("texture_id", &ImDrawCmd::TextureId);
    DrawCmd.def_readwrite("vtx_offset", &ImDrawCmd::VtxOffset);
    DrawCmd.def_readwrite("idx_offset", &ImDrawCmd::IdxOffset);
    DrawCmd.def_readwrite("elem_count", &ImDrawCmd::ElemCount);
    DrawCmd.def_readwrite("user_callback_data", &ImDrawCmd::UserCallbackData);
    DrawCmd.def(py::init<>());
    DrawCmd.def("get_tex_id", &ImDrawCmd::GetTexID
    , py::return_value_policy::automatic_reference);
    py::class_<ImDrawVert> DrawVert(gui, "DrawVert");
    DrawVert.def_readwrite("pos", &ImDrawVert::pos);
    DrawVert.def_readwrite("uv", &ImDrawVert::uv);
    DrawVert.def_readwrite("col", &ImDrawVert::col);
    py::class_<ImDrawCmdHeader> DrawCmdHeader(gui, "DrawCmdHeader");
    DrawCmdHeader.def_readwrite("clip_rect", &ImDrawCmdHeader::ClipRect);
    DrawCmdHeader.def_readwrite("texture_id", &ImDrawCmdHeader::TextureId);
    DrawCmdHeader.def_readwrite("vtx_offset", &ImDrawCmdHeader::VtxOffset);
    py::class_<ImDrawChannel> DrawChannel(gui, "DrawChannel");
    py::class_<ImDrawListSplitter> DrawListSplitter(gui, "DrawListSplitter");
    DrawListSplitter.def(py::init<>());
    DrawListSplitter.def("clear", &ImDrawListSplitter::Clear
    , py::return_value_policy::automatic_reference);
    DrawListSplitter.def("clear_free_memory", &ImDrawListSplitter::ClearFreeMemory
    , py::return_value_policy::automatic_reference);
    DrawListSplitter.def("split", &ImDrawListSplitter::Split
    , py::arg("draw_list")
    , py::arg("count")
    , py::return_value_policy::automatic_reference);
    DrawListSplitter.def("merge", &ImDrawListSplitter::Merge
    , py::arg("draw_list")
    , py::return_value_policy::automatic_reference);
    DrawListSplitter.def("set_current_channel", &ImDrawListSplitter::SetCurrentChannel
    , py::arg("draw_list")
    , py::arg("channel_idx")
    , py::return_value_policy::automatic_reference);
    py::enum_<ImDrawFlags_>(gui, "DrawFlags", py::arithmetic())
        .value("NONE", ImDrawFlags_None)
        .value("CLOSED", ImDrawFlags_Closed)
        .value("ROUND_CORNERS_TOP_LEFT", ImDrawFlags_RoundCornersTopLeft)
        .value("ROUND_CORNERS_TOP_RIGHT", ImDrawFlags_RoundCornersTopRight)
        .value("ROUND_CORNERS_BOTTOM_LEFT", ImDrawFlags_RoundCornersBottomLeft)
        .value("ROUND_CORNERS_BOTTOM_RIGHT", ImDrawFlags_RoundCornersBottomRight)
        .value("ROUND_CORNERS_NONE", ImDrawFlags_RoundCornersNone)
        .value("ROUND_CORNERS_TOP", ImDrawFlags_RoundCornersTop)
        .value("ROUND_CORNERS_BOTTOM", ImDrawFlags_RoundCornersBottom)
        .value("ROUND_CORNERS_LEFT", ImDrawFlags_RoundCornersLeft)
        .value("ROUND_CORNERS_RIGHT", ImDrawFlags_RoundCornersRight)
        .value("ROUND_CORNERS_ALL", ImDrawFlags_RoundCornersAll)
        .value("ROUND_CORNERS_DEFAULT", ImDrawFlags_RoundCornersDefault_)
        .value("ROUND_CORNERS_MASK", ImDrawFlags_RoundCornersMask_)
        .export_values();

    py::enum_<ImDrawListFlags_>(gui, "DrawListFlags", py::arithmetic())
        .value("NONE", ImDrawListFlags_None)
        .value("ANTI_ALIASED_LINES", ImDrawListFlags_AntiAliasedLines)
        .value("ANTI_ALIASED_LINES_USE_TEX", ImDrawListFlags_AntiAliasedLinesUseTex)
        .value("ANTI_ALIASED_FILL", ImDrawListFlags_AntiAliasedFill)
        .value("ALLOW_VTX_OFFSET", ImDrawListFlags_AllowVtxOffset)
        .export_values();

    py::class_<ImDrawList> DrawList(gui, "DrawList");
    DrawList.def_readwrite("cmd_buffer", &ImDrawList::CmdBuffer);
    DrawList.def_readwrite("idx_buffer", &ImDrawList::IdxBuffer);
    DrawList.def_readwrite("vtx_buffer", &ImDrawList::VtxBuffer);
    DrawList.def_readwrite("flags", &ImDrawList::Flags);
    DrawList.def(py::init<ImDrawListSharedData *>()
    , py::arg("shared_data")
    );
    DrawList.def("push_clip_rect", &ImDrawList::PushClipRect
    , py::arg("clip_rect_min")
    , py::arg("clip_rect_max")
    , py::arg("intersect_with_current_clip_rect") = false
    , py::return_value_policy::automatic_reference);
    DrawList.def("push_clip_rect_full_screen", &ImDrawList::PushClipRectFullScreen
    , py::return_value_policy::automatic_reference);
    DrawList.def("pop_clip_rect", &ImDrawList::PopClipRect
    , py::return_value_policy::automatic_reference);
    DrawList.def("push_texture_id", &ImDrawList::PushTextureID
    , py::arg("texture_id")
    , py::return_value_policy::automatic_reference);
    DrawList.def("pop_texture_id", &ImDrawList::PopTextureID
    , py::return_value_policy::automatic_reference);
    DrawList.def("get_clip_rect_min", &ImDrawList::GetClipRectMin
    , py::return_value_policy::automatic_reference);
    DrawList.def("get_clip_rect_max", &ImDrawList::GetClipRectMax
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_line", &ImDrawList::AddLine
    , py::arg("p1")
    , py::arg("p2")
    , py::arg("col")
    , py::arg("thickness") = 1.0f
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_rect", &ImDrawList::AddRect
    , py::arg("p_min")
    , py::arg("p_max")
    , py::arg("col")
    , py::arg("rounding") = 0.0f
    , py::arg("flags") = 0
    , py::arg("thickness") = 1.0f
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_rect_filled", &ImDrawList::AddRectFilled
    , py::arg("p_min")
    , py::arg("p_max")
    , py::arg("col")
    , py::arg("rounding") = 0.0f
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_rect_filled_multi_color", &ImDrawList::AddRectFilledMultiColor
    , py::arg("p_min")
    , py::arg("p_max")
    , py::arg("col_upr_left")
    , py::arg("col_upr_right")
    , py::arg("col_bot_right")
    , py::arg("col_bot_left")
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_quad", &ImDrawList::AddQuad
    , py::arg("p1")
    , py::arg("p2")
    , py::arg("p3")
    , py::arg("p4")
    , py::arg("col")
    , py::arg("thickness") = 1.0f
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_quad_filled", &ImDrawList::AddQuadFilled
    , py::arg("p1")
    , py::arg("p2")
    , py::arg("p3")
    , py::arg("p4")
    , py::arg("col")
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_triangle", &ImDrawList::AddTriangle
    , py::arg("p1")
    , py::arg("p2")
    , py::arg("p3")
    , py::arg("col")
    , py::arg("thickness") = 1.0f
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_triangle_filled", &ImDrawList::AddTriangleFilled
    , py::arg("p1")
    , py::arg("p2")
    , py::arg("p3")
    , py::arg("col")
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_circle", &ImDrawList::AddCircle
    , py::arg("center")
    , py::arg("radius")
    , py::arg("col")
    , py::arg("num_segments") = 0
    , py::arg("thickness") = 1.0f
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_circle_filled", &ImDrawList::AddCircleFilled
    , py::arg("center")
    , py::arg("radius")
    , py::arg("col")
    , py::arg("num_segments") = 0
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_ngon", &ImDrawList::AddNgon
    , py::arg("center")
    , py::arg("radius")
    , py::arg("col")
    , py::arg("num_segments")
    , py::arg("thickness") = 1.0f
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_ngon_filled", &ImDrawList::AddNgonFilled
    , py::arg("center")
    , py::arg("radius")
    , py::arg("col")
    , py::arg("num_segments")
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_ellipse", &ImDrawList::AddEllipse
    , py::arg("center")
    , py::arg("radius")
    , py::arg("col")
    , py::arg("rot") = 0.0f
    , py::arg("num_segments") = 0
    , py::arg("thickness") = 1.0f
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_ellipse_filled", &ImDrawList::AddEllipseFilled
    , py::arg("center")
    , py::arg("radius")
    , py::arg("col")
    , py::arg("rot") = 0.0f
    , py::arg("num_segments") = 0
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_text", py::overload_cast<const ImVec2 &, ImU32, const char *, const char *>(&ImDrawList::AddText)
    , py::arg("pos")
    , py::arg("col")
    , py::arg("text_begin")
    , py::arg("text_end") = nullptr
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_text", py::overload_cast<const ImFont *, float, const ImVec2 &, ImU32, const char *, const char *, float, const ImVec4 *>(&ImDrawList::AddText)
    , py::arg("font")
    , py::arg("font_size")
    , py::arg("pos")
    , py::arg("col")
    , py::arg("text_begin")
    , py::arg("text_end") = nullptr
    , py::arg("wrap_width") = 0.0f
    , py::arg("cpu_fine_clip_rect") = nullptr
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_bezier_cubic", &ImDrawList::AddBezierCubic
    , py::arg("p1")
    , py::arg("p2")
    , py::arg("p3")
    , py::arg("p4")
    , py::arg("col")
    , py::arg("thickness")
    , py::arg("num_segments") = 0
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_bezier_quadratic", &ImDrawList::AddBezierQuadratic
    , py::arg("p1")
    , py::arg("p2")
    , py::arg("p3")
    , py::arg("col")
    , py::arg("thickness")
    , py::arg("num_segments") = 0
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_polyline", &ImDrawList::AddPolyline
    , py::arg("points")
    , py::arg("num_points")
    , py::arg("col")
    , py::arg("flags")
    , py::arg("thickness")
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_convex_poly_filled", &ImDrawList::AddConvexPolyFilled
    , py::arg("points")
    , py::arg("num_points")
    , py::arg("col")
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_concave_poly_filled", &ImDrawList::AddConcavePolyFilled
    , py::arg("points")
    , py::arg("num_points")
    , py::arg("col")
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_image", &ImDrawList::AddImage
    , py::arg("user_texture_id")
    , py::arg("p_min")
    , py::arg("p_max")
    , py::arg("uv_min") = ImVec2(0,0)
    , py::arg("uv_max") = ImVec2(1,1)
    , py::arg("col") = IM_COL32_WHITE
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_image_quad", &ImDrawList::AddImageQuad
    , py::arg("user_texture_id")
    , py::arg("p1")
    , py::arg("p2")
    , py::arg("p3")
    , py::arg("p4")
    , py::arg("uv1") = ImVec2(0,0)
    , py::arg("uv2") = ImVec2(1,0)
    , py::arg("uv3") = ImVec2(1,1)
    , py::arg("uv4") = ImVec2(0,1)
    , py::arg("col") = IM_COL32_WHITE
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_image_rounded", &ImDrawList::AddImageRounded
    , py::arg("user_texture_id")
    , py::arg("p_min")
    , py::arg("p_max")
    , py::arg("uv_min")
    , py::arg("uv_max")
    , py::arg("col")
    , py::arg("rounding")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    DrawList.def("path_clear", &ImDrawList::PathClear
    , py::return_value_policy::automatic_reference);
    DrawList.def("path_line_to", &ImDrawList::PathLineTo
    , py::arg("pos")
    , py::return_value_policy::automatic_reference);
    DrawList.def("path_line_to_merge_duplicate", &ImDrawList::PathLineToMergeDuplicate
    , py::arg("pos")
    , py::return_value_policy::automatic_reference);
    DrawList.def("path_fill_convex", &ImDrawList::PathFillConvex
    , py::arg("col")
    , py::return_value_policy::automatic_reference);
    DrawList.def("path_fill_concave", &ImDrawList::PathFillConcave
    , py::arg("col")
    , py::return_value_policy::automatic_reference);
    DrawList.def("path_stroke", &ImDrawList::PathStroke
    , py::arg("col")
    , py::arg("flags") = 0
    , py::arg("thickness") = 1.0f
    , py::return_value_policy::automatic_reference);
    DrawList.def("path_arc_to", &ImDrawList::PathArcTo
    , py::arg("center")
    , py::arg("radius")
    , py::arg("a_min")
    , py::arg("a_max")
    , py::arg("num_segments") = 0
    , py::return_value_policy::automatic_reference);
    DrawList.def("path_arc_to_fast", &ImDrawList::PathArcToFast
    , py::arg("center")
    , py::arg("radius")
    , py::arg("a_min_of_12")
    , py::arg("a_max_of_12")
    , py::return_value_policy::automatic_reference);
    DrawList.def("path_elliptical_arc_to", &ImDrawList::PathEllipticalArcTo
    , py::arg("center")
    , py::arg("radius")
    , py::arg("rot")
    , py::arg("a_min")
    , py::arg("a_max")
    , py::arg("num_segments") = 0
    , py::return_value_policy::automatic_reference);
    DrawList.def("path_bezier_cubic_curve_to", &ImDrawList::PathBezierCubicCurveTo
    , py::arg("p2")
    , py::arg("p3")
    , py::arg("p4")
    , py::arg("num_segments") = 0
    , py::return_value_policy::automatic_reference);
    DrawList.def("path_bezier_quadratic_curve_to", &ImDrawList::PathBezierQuadraticCurveTo
    , py::arg("p2")
    , py::arg("p3")
    , py::arg("num_segments") = 0
    , py::return_value_policy::automatic_reference);
    DrawList.def("path_rect", &ImDrawList::PathRect
    , py::arg("rect_min")
    , py::arg("rect_max")
    , py::arg("rounding") = 0.0f
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    DrawList.def("add_draw_cmd", &ImDrawList::AddDrawCmd
    , py::return_value_policy::automatic_reference);
    DrawList.def("clone_output", &ImDrawList::CloneOutput
    , py::return_value_policy::automatic_reference);
    DrawList.def("channels_split", &ImDrawList::ChannelsSplit
    , py::arg("count")
    , py::return_value_policy::automatic_reference);
    DrawList.def("channels_merge", &ImDrawList::ChannelsMerge
    , py::return_value_policy::automatic_reference);
    DrawList.def("channels_set_current", &ImDrawList::ChannelsSetCurrent
    , py::arg("n")
    , py::return_value_policy::automatic_reference);
    DrawList.def("prim_reserve", &ImDrawList::PrimReserve
    , py::arg("idx_count")
    , py::arg("vtx_count")
    , py::return_value_policy::automatic_reference);
    DrawList.def("prim_unreserve", &ImDrawList::PrimUnreserve
    , py::arg("idx_count")
    , py::arg("vtx_count")
    , py::return_value_policy::automatic_reference);
    DrawList.def("prim_rect", &ImDrawList::PrimRect
    , py::arg("a")
    , py::arg("b")
    , py::arg("col")
    , py::return_value_policy::automatic_reference);
    DrawList.def("prim_rect_uv", &ImDrawList::PrimRectUV
    , py::arg("a")
    , py::arg("b")
    , py::arg("uv_a")
    , py::arg("uv_b")
    , py::arg("col")
    , py::return_value_policy::automatic_reference);
    DrawList.def("prim_quad_uv", &ImDrawList::PrimQuadUV
    , py::arg("a")
    , py::arg("b")
    , py::arg("c")
    , py::arg("d")
    , py::arg("uv_a")
    , py::arg("uv_b")
    , py::arg("uv_c")
    , py::arg("uv_d")
    , py::arg("col")
    , py::return_value_policy::automatic_reference);
    DrawList.def("prim_write_vtx", &ImDrawList::PrimWriteVtx
    , py::arg("pos")
    , py::arg("uv")
    , py::arg("col")
    , py::return_value_policy::automatic_reference);
    DrawList.def("prim_write_idx", &ImDrawList::PrimWriteIdx
    , py::arg("idx")
    , py::return_value_policy::automatic_reference);
    DrawList.def("prim_vtx", &ImDrawList::PrimVtx
    , py::arg("pos")
    , py::arg("uv")
    , py::arg("col")
    , py::return_value_policy::automatic_reference);
    py::class_<ImDrawData> DrawData(gui, "DrawData");
    DrawData.def_readwrite("valid", &ImDrawData::Valid);
    DrawData.def_readwrite("cmd_lists_count", &ImDrawData::CmdListsCount);
    DrawData.def_readwrite("total_idx_count", &ImDrawData::TotalIdxCount);
    DrawData.def_readwrite("total_vtx_count", &ImDrawData::TotalVtxCount);
    DrawData.def_readwrite("display_pos", &ImDrawData::DisplayPos);
    DrawData.def_readwrite("display_size", &ImDrawData::DisplaySize);
    DrawData.def_readwrite("framebuffer_scale", &ImDrawData::FramebufferScale);
    DrawData.def_readwrite("owner_viewport", &ImDrawData::OwnerViewport);
    DrawData.def(py::init<>());
    DrawData.def("clear", &ImDrawData::Clear
    , py::return_value_policy::automatic_reference);
    DrawData.def("add_draw_list", &ImDrawData::AddDrawList
    , py::arg("draw_list")
    , py::return_value_policy::automatic_reference);
    DrawData.def("de_index_all_buffers", &ImDrawData::DeIndexAllBuffers
    , py::return_value_policy::automatic_reference);
    DrawData.def("scale_clip_rects", &ImDrawData::ScaleClipRects
    , py::arg("fb_scale")
    , py::return_value_policy::automatic_reference);
    py::class_<ImFontConfig> FontConfig(gui, "FontConfig");
    FontConfig.def_readwrite("font_data", &ImFontConfig::FontData);
    FontConfig.def_readwrite("font_data_size", &ImFontConfig::FontDataSize);
    FontConfig.def_readwrite("font_data_owned_by_atlas", &ImFontConfig::FontDataOwnedByAtlas);
    FontConfig.def_readwrite("font_no", &ImFontConfig::FontNo);
    FontConfig.def_readwrite("size_pixels", &ImFontConfig::SizePixels);
    FontConfig.def_readwrite("oversample_h", &ImFontConfig::OversampleH);
    FontConfig.def_readwrite("oversample_v", &ImFontConfig::OversampleV);
    FontConfig.def_readwrite("pixel_snap_h", &ImFontConfig::PixelSnapH);
    FontConfig.def_readwrite("glyph_extra_spacing", &ImFontConfig::GlyphExtraSpacing);
    FontConfig.def_readwrite("glyph_offset", &ImFontConfig::GlyphOffset);
    FontConfig.def_readwrite("glyph_ranges", &ImFontConfig::GlyphRanges);
    FontConfig.def_readwrite("glyph_min_advance_x", &ImFontConfig::GlyphMinAdvanceX);
    FontConfig.def_readwrite("glyph_max_advance_x", &ImFontConfig::GlyphMaxAdvanceX);
    FontConfig.def_readwrite("merge_mode", &ImFontConfig::MergeMode);
    FontConfig.def_readwrite("font_builder_flags", &ImFontConfig::FontBuilderFlags);
    FontConfig.def_readwrite("rasterizer_multiply", &ImFontConfig::RasterizerMultiply);
    FontConfig.def_readwrite("rasterizer_density", &ImFontConfig::RasterizerDensity);
    FontConfig.def_readwrite("ellipsis_char", &ImFontConfig::EllipsisChar);
    FontConfig.def_readonly("name", &ImFontConfig::Name);
    FontConfig.def_readwrite("dst_font", &ImFontConfig::DstFont);
    FontConfig.def(py::init<>());
    py::class_<ImFontGlyph> FontGlyph(gui, "FontGlyph");
    FontGlyph.def_property("colored",
        [](ImFontGlyph *self) { return self->Colored; },
        [](ImFontGlyph *self, int value) { self->Colored = value; }
    );
    FontGlyph.def_property("visible",
        [](ImFontGlyph *self) { return self->Visible; },
        [](ImFontGlyph *self, int value) { self->Visible = value; }
    );
    FontGlyph.def_property("codepoint",
        [](ImFontGlyph *self) { return self->Codepoint; },
        [](ImFontGlyph *self, int value) { self->Codepoint = value; }
    );
    FontGlyph.def_readwrite("advance_x", &ImFontGlyph::AdvanceX);
    FontGlyph.def_readwrite("x0", &ImFontGlyph::X0);
    FontGlyph.def_readwrite("y0", &ImFontGlyph::Y0);
    FontGlyph.def_readwrite("x1", &ImFontGlyph::X1);
    FontGlyph.def_readwrite("y1", &ImFontGlyph::Y1);
    FontGlyph.def_readwrite("u0", &ImFontGlyph::U0);
    FontGlyph.def_readwrite("v0", &ImFontGlyph::V0);
    FontGlyph.def_readwrite("u1", &ImFontGlyph::U1);
    FontGlyph.def_readwrite("v1", &ImFontGlyph::V1);
    py::class_<ImFontGlyphRangesBuilder> FontGlyphRangesBuilder(gui, "FontGlyphRangesBuilder");
    FontGlyphRangesBuilder.def_readwrite("used_chars", &ImFontGlyphRangesBuilder::UsedChars);
    FontGlyphRangesBuilder.def(py::init<>());
    FontGlyphRangesBuilder.def("clear", &ImFontGlyphRangesBuilder::Clear
    , py::return_value_policy::automatic_reference);
    FontGlyphRangesBuilder.def("get_bit", &ImFontGlyphRangesBuilder::GetBit
    , py::arg("n")
    , py::return_value_policy::automatic_reference);
    FontGlyphRangesBuilder.def("set_bit", &ImFontGlyphRangesBuilder::SetBit
    , py::arg("n")
    , py::return_value_policy::automatic_reference);
    FontGlyphRangesBuilder.def("add_char", &ImFontGlyphRangesBuilder::AddChar
    , py::arg("c")
    , py::return_value_policy::automatic_reference);
    FontGlyphRangesBuilder.def("add_text", &ImFontGlyphRangesBuilder::AddText
    , py::arg("text")
    , py::arg("text_end") = nullptr
    , py::return_value_policy::automatic_reference);
    /***
     * Missing symbols
    FontGlyphRangesBuilder.def("add_ranges", &ImFontGlyphRangesBuilder::AddRanges
    , py::arg("ranges")
    , py::return_value_policy::automatic_reference);
    FontGlyphRangesBuilder.def("build_ranges", &ImFontGlyphRangesBuilder::BuildRanges
    , py::arg("out_ranges")
    , py::return_value_policy::automatic_reference);
    ***/
    py::class_<ImFontAtlasCustomRect> FontAtlasCustomRect(gui, "FontAtlasCustomRect");
    FontAtlasCustomRect.def_readwrite("width", &ImFontAtlasCustomRect::Width);
    FontAtlasCustomRect.def_readwrite("height", &ImFontAtlasCustomRect::Height);
    FontAtlasCustomRect.def_readwrite("x", &ImFontAtlasCustomRect::X);
    FontAtlasCustomRect.def_readwrite("y", &ImFontAtlasCustomRect::Y);
    FontAtlasCustomRect.def_readwrite("glyph_id", &ImFontAtlasCustomRect::GlyphID);
    FontAtlasCustomRect.def_readwrite("glyph_advance_x", &ImFontAtlasCustomRect::GlyphAdvanceX);
    FontAtlasCustomRect.def_readwrite("glyph_offset", &ImFontAtlasCustomRect::GlyphOffset);
    FontAtlasCustomRect.def_readwrite("font", &ImFontAtlasCustomRect::Font);
    FontAtlasCustomRect.def(py::init<>());
    FontAtlasCustomRect.def("is_packed", &ImFontAtlasCustomRect::IsPacked
    , py::return_value_policy::automatic_reference);
    py::enum_<ImFontAtlasFlags_>(gui, "FontAtlasFlags", py::arithmetic())
        .value("NONE", ImFontAtlasFlags_None)
        .value("NO_POWER_OF_TWO_HEIGHT", ImFontAtlasFlags_NoPowerOfTwoHeight)
        .value("NO_MOUSE_CURSORS", ImFontAtlasFlags_NoMouseCursors)
        .value("NO_BAKED_LINES", ImFontAtlasFlags_NoBakedLines)
        .export_values();

    py::class_<ImFontAtlas> FontAtlas(gui, "FontAtlas");
    FontAtlas.def(py::init<>());
    FontAtlas.def("add_font", &ImFontAtlas::AddFont
    , py::arg("font_cfg")
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("add_font_default", &ImFontAtlas::AddFontDefault
    , py::arg("font_cfg") = nullptr
    , py::return_value_policy::automatic_reference);
    /***
     * Missing symbols
    FontAtlas.def("add_font_from_file_ttf", &ImFontAtlas::AddFontFromFileTTF
    , py::arg("filename")
    , py::arg("size_pixels")
    , py::arg("font_cfg") = nullptr
    , py::arg("glyph_ranges") = nullptr
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("add_font_from_memory_ttf", &ImFontAtlas::AddFontFromMemoryTTF
    , py::arg("font_data")
    , py::arg("font_data_size")
    , py::arg("size_pixels")
    , py::arg("font_cfg") = nullptr
    , py::arg("glyph_ranges") = nullptr
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("add_font_from_memory_compressed_ttf", &ImFontAtlas::AddFontFromMemoryCompressedTTF
    , py::arg("compressed_font_data")
    , py::arg("compressed_font_data_size")
    , py::arg("size_pixels")
    , py::arg("font_cfg") = nullptr
    , py::arg("glyph_ranges") = nullptr
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("add_font_from_memory_compressed_base85_ttf", &ImFontAtlas::AddFontFromMemoryCompressedBase85TTF
    , py::arg("compressed_font_data_base85")
    , py::arg("size_pixels")
    , py::arg("font_cfg") = nullptr
    , py::arg("glyph_ranges") = nullptr
    , py::return_value_policy::automatic_reference);
    ***/
    FontAtlas.def("clear_input_data", &ImFontAtlas::ClearInputData
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("clear_tex_data", &ImFontAtlas::ClearTexData
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("clear_fonts", &ImFontAtlas::ClearFonts
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("clear", &ImFontAtlas::Clear
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("build", &ImFontAtlas::Build
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("is_built", &ImFontAtlas::IsBuilt
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("set_tex_id", &ImFontAtlas::SetTexID
    , py::arg("id")
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("get_glyph_ranges_default", &ImFontAtlas::GetGlyphRangesDefault
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("get_glyph_ranges_greek", &ImFontAtlas::GetGlyphRangesGreek
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("get_glyph_ranges_korean", &ImFontAtlas::GetGlyphRangesKorean
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("get_glyph_ranges_japanese", &ImFontAtlas::GetGlyphRangesJapanese
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("get_glyph_ranges_chinese_full", &ImFontAtlas::GetGlyphRangesChineseFull
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("get_glyph_ranges_chinese_simplified_common", &ImFontAtlas::GetGlyphRangesChineseSimplifiedCommon
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("get_glyph_ranges_cyrillic", &ImFontAtlas::GetGlyphRangesCyrillic
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("get_glyph_ranges_thai", &ImFontAtlas::GetGlyphRangesThai
    , py::return_value_policy::automatic_reference);
    FontAtlas.def("get_glyph_ranges_vietnamese", &ImFontAtlas::GetGlyphRangesVietnamese
    , py::return_value_policy::automatic_reference);
    FontAtlas.def_readwrite("flags", &ImFontAtlas::Flags);
    FontAtlas.def_readwrite("tex_id", &ImFontAtlas::TexID);
    FontAtlas.def_readwrite("tex_desired_width", &ImFontAtlas::TexDesiredWidth);
    FontAtlas.def_readwrite("tex_glyph_padding", &ImFontAtlas::TexGlyphPadding);
    FontAtlas.def_readwrite("locked", &ImFontAtlas::Locked);
    FontAtlas.def_readwrite("user_data", &ImFontAtlas::UserData);
    FontAtlas.def_readwrite("tex_ready", &ImFontAtlas::TexReady);
    FontAtlas.def_readwrite("tex_pixels_use_colors", &ImFontAtlas::TexPixelsUseColors);
    FontAtlas.def_readonly("tex_uv_lines", &ImFontAtlas::TexUvLines);
    FontAtlas.def_readwrite("font_builder_io", &ImFontAtlas::FontBuilderIO);
    FontAtlas.def_readwrite("font_builder_flags", &ImFontAtlas::FontBuilderFlags);
    FontAtlas.def_readwrite("pack_id_mouse_cursors", &ImFontAtlas::PackIdMouseCursors);
    FontAtlas.def_readwrite("pack_id_lines", &ImFontAtlas::PackIdLines);
    py::class_<ImFont> Font(gui, "Font");
    Font.def_readwrite("index_advance_x", &ImFont::IndexAdvanceX);
    Font.def_readwrite("fallback_advance_x", &ImFont::FallbackAdvanceX);
    Font.def_readwrite("font_size", &ImFont::FontSize);
    Font.def_readwrite("index_lookup", &ImFont::IndexLookup);
    Font.def_readwrite("glyphs", &ImFont::Glyphs);
    Font.def_readwrite("fallback_glyph", &ImFont::FallbackGlyph);
    Font.def_readwrite("container_atlas", &ImFont::ContainerAtlas);
    Font.def_readwrite("config_data", &ImFont::ConfigData);
    Font.def_readwrite("config_data_count", &ImFont::ConfigDataCount);
    Font.def_readwrite("fallback_char", &ImFont::FallbackChar);
    Font.def_readwrite("ellipsis_char", &ImFont::EllipsisChar);
    Font.def_readwrite("ellipsis_char_count", &ImFont::EllipsisCharCount);
    Font.def_readwrite("ellipsis_width", &ImFont::EllipsisWidth);
    Font.def_readwrite("ellipsis_char_step", &ImFont::EllipsisCharStep);
    Font.def_readwrite("dirty_lookup_tables", &ImFont::DirtyLookupTables);
    Font.def_readwrite("scale", &ImFont::Scale);
    Font.def_readwrite("ascent", &ImFont::Ascent);
    Font.def_readwrite("descent", &ImFont::Descent);
    Font.def_readwrite("metrics_total_surface", &ImFont::MetricsTotalSurface);
    Font.def_readonly("used4k_pages_map", &ImFont::Used4kPagesMap);
    Font.def(py::init<>());
    /***
     * Missing symbols
    Font.def("find_glyph", &ImFont::FindGlyph
    , py::arg("c")
    , py::return_value_policy::automatic_reference);
    Font.def("find_glyph_no_fallback", &ImFont::FindGlyphNoFallback
    , py::arg("c")
    , py::return_value_policy::automatic_reference);
    ***/
    Font.def("get_char_advance", &ImFont::GetCharAdvance
    , py::arg("c")
    , py::return_value_policy::automatic_reference);
    Font.def("is_loaded", &ImFont::IsLoaded
    , py::return_value_policy::automatic_reference);
    Font.def("get_debug_name", &ImFont::GetDebugName
    , py::return_value_policy::automatic_reference);
    Font.def("calc_word_wrap_position_a", &ImFont::CalcWordWrapPositionA
    , py::arg("scale")
    , py::arg("text")
    , py::arg("text_end")
    , py::arg("wrap_width")
    , py::return_value_policy::automatic_reference);
    /***
     * Missing symbols
    Font.def("render_char", &ImFont::RenderChar
    , py::arg("draw_list")
    , py::arg("size")
    , py::arg("pos")
    , py::arg("col")
    , py::arg("c")
    , py::return_value_policy::automatic_reference);
    ***/
    Font.def("render_text", &ImFont::RenderText
    , py::arg("draw_list")
    , py::arg("size")
    , py::arg("pos")
    , py::arg("col")
    , py::arg("clip_rect")
    , py::arg("text_begin")
    , py::arg("text_end")
    , py::arg("wrap_width") = 0.0f
    , py::arg("cpu_fine_clip") = false
    , py::return_value_policy::automatic_reference);
    Font.def("build_lookup_table", &ImFont::BuildLookupTable
    , py::return_value_policy::automatic_reference);
    Font.def("clear_output_data", &ImFont::ClearOutputData
    , py::return_value_policy::automatic_reference);
    Font.def("grow_index", &ImFont::GrowIndex
    , py::arg("new_size")
    , py::return_value_policy::automatic_reference);
    /***
     * Missing symbols
    Font.def("add_glyph", &ImFont::AddGlyph
    , py::arg("src_cfg")
    , py::arg("c")
    , py::arg("x0")
    , py::arg("y0")
    , py::arg("x1")
    , py::arg("y1")
    , py::arg("u0")
    , py::arg("v0")
    , py::arg("u1")
    , py::arg("v1")
    , py::arg("advance_x")
    , py::return_value_policy::automatic_reference);
    Font.def("add_remap_char", &ImFont::AddRemapChar
    , py::arg("dst")
    , py::arg("src")
    , py::arg("overwrite_dst") = true
    , py::return_value_policy::automatic_reference);
    Font.def("set_glyph_visible", &ImFont::SetGlyphVisible
    , py::arg("c")
    , py::arg("visible")
    , py::return_value_policy::automatic_reference);
    ***/
    Font.def("is_glyph_range_unused", &ImFont::IsGlyphRangeUnused
    , py::arg("c_begin")
    , py::arg("c_last")
    , py::return_value_policy::automatic_reference);
    py::enum_<ImGuiViewportFlags_>(gui, "ViewportFlags", py::arithmetic())
        .value("NONE", ImGuiViewportFlags_None)
        .value("IS_PLATFORM_WINDOW", ImGuiViewportFlags_IsPlatformWindow)
        .value("IS_PLATFORM_MONITOR", ImGuiViewportFlags_IsPlatformMonitor)
        .value("OWNED_BY_APP", ImGuiViewportFlags_OwnedByApp)
        .value("NO_DECORATION", ImGuiViewportFlags_NoDecoration)
        .value("NO_TASK_BAR_ICON", ImGuiViewportFlags_NoTaskBarIcon)
        .value("NO_FOCUS_ON_APPEARING", ImGuiViewportFlags_NoFocusOnAppearing)
        .value("NO_FOCUS_ON_CLICK", ImGuiViewportFlags_NoFocusOnClick)
        .value("NO_INPUTS", ImGuiViewportFlags_NoInputs)
        .value("NO_RENDERER_CLEAR", ImGuiViewportFlags_NoRendererClear)
        .value("NO_AUTO_MERGE", ImGuiViewportFlags_NoAutoMerge)
        .value("TOP_MOST", ImGuiViewportFlags_TopMost)
        .value("CAN_HOST_OTHER_WINDOWS", ImGuiViewportFlags_CanHostOtherWindows)
        .value("IS_MINIMIZED", ImGuiViewportFlags_IsMinimized)
        .value("IS_FOCUSED", ImGuiViewportFlags_IsFocused)
        .export_values();

    py::class_<ImGuiViewport> Viewport(gui, "Viewport");
    Viewport.def_readwrite("id", &ImGuiViewport::ID);
    Viewport.def_readwrite("flags", &ImGuiViewport::Flags);
    Viewport.def_readwrite("pos", &ImGuiViewport::Pos);
    Viewport.def_readwrite("size", &ImGuiViewport::Size);
    Viewport.def_readwrite("work_pos", &ImGuiViewport::WorkPos);
    Viewport.def_readwrite("work_size", &ImGuiViewport::WorkSize);
    Viewport.def_readwrite("dpi_scale", &ImGuiViewport::DpiScale);
    Viewport.def_readwrite("parent_viewport_id", &ImGuiViewport::ParentViewportId);
    Viewport.def_readwrite("draw_data", &ImGuiViewport::DrawData);
    Viewport.def_readwrite("renderer_user_data", &ImGuiViewport::RendererUserData);
    Viewport.def_readwrite("platform_user_data", &ImGuiViewport::PlatformUserData);
    Viewport.def_readwrite("platform_handle", &ImGuiViewport::PlatformHandle);
    Viewport.def_readwrite("platform_handle_raw", &ImGuiViewport::PlatformHandleRaw);
    Viewport.def_readwrite("platform_window_created", &ImGuiViewport::PlatformWindowCreated);
    Viewport.def_readwrite("platform_request_move", &ImGuiViewport::PlatformRequestMove);
    Viewport.def_readwrite("platform_request_resize", &ImGuiViewport::PlatformRequestResize);
    Viewport.def_readwrite("platform_request_close", &ImGuiViewport::PlatformRequestClose);
    Viewport.def(py::init<>());
    Viewport.def("get_center", &ImGuiViewport::GetCenter
    , py::return_value_policy::automatic_reference);
    Viewport.def("get_work_center", &ImGuiViewport::GetWorkCenter
    , py::return_value_policy::automatic_reference);
    py::class_<ImGuiPlatformMonitor> PlatformMonitor(gui, "PlatformMonitor");
    PlatformMonitor.def_readwrite("main_pos", &ImGuiPlatformMonitor::MainPos);
    PlatformMonitor.def_readwrite("main_size", &ImGuiPlatformMonitor::MainSize);
    PlatformMonitor.def_readwrite("work_pos", &ImGuiPlatformMonitor::WorkPos);
    PlatformMonitor.def_readwrite("work_size", &ImGuiPlatformMonitor::WorkSize);
    PlatformMonitor.def_readwrite("dpi_scale", &ImGuiPlatformMonitor::DpiScale);
    PlatformMonitor.def_readwrite("platform_handle", &ImGuiPlatformMonitor::PlatformHandle);
    PlatformMonitor.def(py::init<>());
    py::class_<ImGuiPlatformImeData> PlatformeData(gui, "PlatformeData");
    PlatformeData.def_readwrite("want_visible", &ImGuiPlatformImeData::WantVisible);
    PlatformeData.def_readwrite("input_pos", &ImGuiPlatformImeData::InputPos);
    PlatformeData.def_readwrite("input_line_height", &ImGuiPlatformImeData::InputLineHeight);
    PlatformeData.def(py::init<>());

    Style.def("set_color", [](ImGuiStyle& self, int item, ImVec4 color)
    {
        if (item < 0) throw py::index_error();
        if (item >= IM_ARRAYSIZE(self.Colors)) throw py::index_error();
        self.Colors[item] = color;
    }, py::arg("item"), py::arg("color"));
    IO.def("set_mouse_down", [](ImGuiIO& self, int button, bool down)
    {
        if (button < 0) throw py::index_error();
        if (button >= IM_ARRAYSIZE(self.MouseDown)) throw py::index_error();
        self.MouseDown[button] = down;
    }, py::arg("button"), py::arg("down"));
    IO.def("set_key_down", [](ImGuiIO& self, int key, bool down)
    {
        if (key < 0) throw py::index_error();
        if (key >= IM_ARRAYSIZE(self.KeysDown)) throw py::index_error();
        self.KeysDown[key] = down;
    }, py::arg("key"), py::arg("down"));
    IO.def("set_key_map", [](ImGuiIO& self, int key, int value)
    {
        if (key < 0) throw py::index_error();
        if (key >= IM_ARRAYSIZE(self.KeyMap)) throw py::index_error();
        self.KeyMap[key] = value;
    }, py::arg("key"), py::arg("value"));
    DrawData.def_property_readonly("cmd_lists", [](const ImDrawData& self)
    {
        py::list ret;
        for(int i = 0; i < self.CmdListsCount; i++)
        {
            ret.append(self.CmdLists[i]);
        }
        return ret;
    });
    DrawVert.def_property_readonly_static("pos_offset", [](py::object)
    {
        return IM_OFFSETOF(ImDrawVert, pos);
    });
    DrawVert.def_property_readonly_static("uv_offset", [](py::object)
    {
        return IM_OFFSETOF(ImDrawVert, uv);
    });
    DrawVert.def_property_readonly_static("col_offset", [](py::object)
    {
        return IM_OFFSETOF(ImDrawVert, col);
    });
    FontAtlas.def("get_tex_data_as_alpha8", [](ImFontAtlas& atlas)
    {
        unsigned char* pixels;
        int width, height, bytes_per_pixel;
        atlas.GetTexDataAsAlpha8(&pixels, &width, &height, &bytes_per_pixel);
        std::string data((char*)pixels, width * height * bytes_per_pixel);
        return std::make_tuple(width, height, py::bytes(data));
    });
    FontAtlas.def("get_tex_data_as_rgba32", [](ImFontAtlas& atlas)
    {
        unsigned char* pixels;
        int width, height, bytes_per_pixel;
        atlas.GetTexDataAsRGBA32(&pixels, &width, &height, &bytes_per_pixel);
        std::string data((char*)pixels, width * height * bytes_per_pixel);
        return std::make_tuple(width, height, py::bytes(data));
    });
    gui.def("init", []()
    {
        ImGui::CreateContext();
        ImGuiIO& io = ImGui::GetIO();
        io.DisplaySize = ImVec2(100.0, 100.0);
        unsigned char* pixels;
        int w, h;
        io.Fonts->GetTexDataAsAlpha8(&pixels, &w, &h, nullptr);
    });
    gui.def("input_text", [](const char* label, char* data, size_t max_size, ImGuiInputTextFlags flags)
    {
        max_size++;
        char* text = (char*)malloc(max_size * sizeof(char));
        strncpy(text, data, max_size);
        auto ret = ImGui::InputText(label, text, max_size, flags, nullptr, NULL);
        std::string output(text);
        free(text);
        return std::make_tuple(ret, output);
    }
    , py::arg("label")
    , py::arg("data")
    , py::arg("max_size")
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("input_text_multiline", [](const char* label, char* data, size_t max_size, const ImVec2& size, ImGuiInputTextFlags flags)
    {
        max_size++;
        char* text = (char*)malloc(max_size * sizeof(char));
        strncpy(text, data, max_size);
        auto ret = ImGui::InputTextMultiline(label, text, max_size, size, flags, nullptr, NULL);
        std::string output(text);
        free(text);
        return std::make_tuple(ret, output);
    }
    , py::arg("label")
    , py::arg("data")
    , py::arg("max_size")
    , py::arg("size") = ImVec2(0,0)
    , py::arg("flags") = 0
    , py::return_value_policy::automatic_reference);
    gui.def("combo", [](const char* label, int * current_item, std::vector<std::string> items, int popup_max_height_in_items)
    {
        std::vector<const char*> ptrs;
        for (const std::string& s : items)
        {
            ptrs.push_back(s.c_str());
        }
        auto ret = ImGui::Combo(label, current_item, ptrs.data(), ptrs.size(), popup_max_height_in_items);
        return std::make_tuple(ret, current_item);
    }
    , py::arg("label")
    , py::arg("current_item")
    , py::arg("items")
    , py::arg("popup_max_height_in_items") = -1
    , py::return_value_policy::automatic_reference);
    gui.def("list_box", [](const char* label, int * current_item, std::vector<std::string> items, int height_in_items)
    {
        std::vector<const char*> ptrs;
        for (const std::string& s : items)
        {
            ptrs.push_back(s.c_str());
        }
        auto ret = ImGui::ListBox(label, current_item, ptrs.data(), ptrs.size(), height_in_items);
        return std::make_tuple(ret, current_item);
    }
    , py::arg("label")
    , py::arg("current_item")
    , py::arg("items")
    , py::arg("height_in_items") = -1
    , py::return_value_policy::automatic_reference);
    gui.def("plot_lines", [](const char* label, std::vector<float> values, int values_offset, const char* overlay_text, float scale_min, float scale_max, ImVec2 graph_size)
    {
        ImGui::PlotLines(label, values.data(), values.size(), values_offset, overlay_text, scale_min, scale_max, graph_size, sizeof(float));
    }
    , py::arg("label")
    , py::arg("values")
    , py::arg("values_offset") = 0
    , py::arg("overlay_text") = nullptr
    , py::arg("scale_min") = FLT_MAX
    , py::arg("scale_max") = FLT_MAX
    , py::arg("graph_size") = ImVec2(0,0)
    );
    gui.def("plot_histogram", [](const char* label, std::vector<float> values, int values_offset, const char* overlay_text, float scale_min, float scale_max, ImVec2 graph_size)
    {
        ImGui::PlotHistogram(label, values.data(), values.size(), values_offset, overlay_text, scale_min, scale_max, graph_size, sizeof(float));
    }
    , py::arg("label")
    , py::arg("values")
    , py::arg("values_offset") = 0
    , py::arg("overlay_text") = nullptr
    , py::arg("scale_min") = FLT_MAX
    , py::arg("scale_max") = FLT_MAX
    , py::arg("graph_size") = ImVec2(0,0)
    );
}

