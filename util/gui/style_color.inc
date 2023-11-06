int PushStyleColor(const proto::Col& v) {
    int pushes = 0;
    if (v.text() != 0) {
        ImGui::PushStyleColor(ImGuiCol_Text, v.text());
        pushes +=1;
    }
    if (v.text_disabled() != 0) {
        ImGui::PushStyleColor(ImGuiCol_TextDisabled, v.text_disabled());
        pushes +=1;
    }
    if (v.window_bg() != 0) {
        ImGui::PushStyleColor(ImGuiCol_WindowBg, v.window_bg());
        pushes +=1;
    }
    if (v.child_bg() != 0) {
        ImGui::PushStyleColor(ImGuiCol_ChildBg, v.child_bg());
        pushes +=1;
    }
    if (v.popup_bg() != 0) {
        ImGui::PushStyleColor(ImGuiCol_PopupBg, v.popup_bg());
        pushes +=1;
    }
    if (v.border() != 0) {
        ImGui::PushStyleColor(ImGuiCol_Border, v.border());
        pushes +=1;
    }
    if (v.border_shadow() != 0) {
        ImGui::PushStyleColor(ImGuiCol_BorderShadow, v.border_shadow());
        pushes +=1;
    }
    if (v.frame_bg() != 0) {
        ImGui::PushStyleColor(ImGuiCol_FrameBg, v.frame_bg());
        pushes +=1;
    }
    if (v.frame_bg_hovered() != 0) {
        ImGui::PushStyleColor(ImGuiCol_FrameBgHovered, v.frame_bg_hovered());
        pushes +=1;
    }
    if (v.frame_bg_active() != 0) {
        ImGui::PushStyleColor(ImGuiCol_FrameBgActive, v.frame_bg_active());
        pushes +=1;
    }
    if (v.title_bg() != 0) {
        ImGui::PushStyleColor(ImGuiCol_TitleBg, v.title_bg());
        pushes +=1;
    }
    if (v.title_bg_active() != 0) {
        ImGui::PushStyleColor(ImGuiCol_TitleBgActive, v.title_bg_active());
        pushes +=1;
    }
    if (v.title_bg_collapsed() != 0) {
        ImGui::PushStyleColor(ImGuiCol_TitleBgCollapsed, v.title_bg_collapsed());
        pushes +=1;
    }
    if (v.menu_bar_bg() != 0) {
        ImGui::PushStyleColor(ImGuiCol_MenuBarBg, v.menu_bar_bg());
        pushes +=1;
    }
    if (v.scrollbar_bg() != 0) {
        ImGui::PushStyleColor(ImGuiCol_ScrollbarBg, v.scrollbar_bg());
        pushes +=1;
    }
    if (v.scrollbar_grab() != 0) {
        ImGui::PushStyleColor(ImGuiCol_ScrollbarGrab, v.scrollbar_grab());
        pushes +=1;
    }
    if (v.scrollbar_grab_hovered() != 0) {
        ImGui::PushStyleColor(ImGuiCol_ScrollbarGrabHovered, v.scrollbar_grab_hovered());
        pushes +=1;
    }
    if (v.scrollbar_grab_active() != 0) {
        ImGui::PushStyleColor(ImGuiCol_ScrollbarGrabActive, v.scrollbar_grab_active());
        pushes +=1;
    }
    if (v.check_mark() != 0) {
        ImGui::PushStyleColor(ImGuiCol_CheckMark, v.check_mark());
        pushes +=1;
    }
    if (v.slider_grab() != 0) {
        ImGui::PushStyleColor(ImGuiCol_SliderGrab, v.slider_grab());
        pushes +=1;
    }
    if (v.slider_grab_active() != 0) {
        ImGui::PushStyleColor(ImGuiCol_SliderGrabActive, v.slider_grab_active());
        pushes +=1;
    }
    if (v.button() != 0) {
        ImGui::PushStyleColor(ImGuiCol_Button, v.button());
        pushes +=1;
    }
    if (v.button_hovered() != 0) {
        ImGui::PushStyleColor(ImGuiCol_ButtonHovered, v.button_hovered());
        pushes +=1;
    }
    if (v.button_active() != 0) {
        ImGui::PushStyleColor(ImGuiCol_ButtonActive, v.button_active());
        pushes +=1;
    }
    if (v.header() != 0) {
        ImGui::PushStyleColor(ImGuiCol_Header, v.header());
        pushes +=1;
    }
    if (v.header_hovered() != 0) {
        ImGui::PushStyleColor(ImGuiCol_HeaderHovered, v.header_hovered());
        pushes +=1;
    }
    if (v.header_active() != 0) {
        ImGui::PushStyleColor(ImGuiCol_HeaderActive, v.header_active());
        pushes +=1;
    }
    if (v.separator() != 0) {
        ImGui::PushStyleColor(ImGuiCol_Separator, v.separator());
        pushes +=1;
    }
    if (v.separator_hovered() != 0) {
        ImGui::PushStyleColor(ImGuiCol_SeparatorHovered, v.separator_hovered());
        pushes +=1;
    }
    if (v.separator_active() != 0) {
        ImGui::PushStyleColor(ImGuiCol_SeparatorActive, v.separator_active());
        pushes +=1;
    }
    if (v.resize_grip() != 0) {
        ImGui::PushStyleColor(ImGuiCol_ResizeGrip, v.resize_grip());
        pushes +=1;
    }
    if (v.resize_grip_hovered() != 0) {
        ImGui::PushStyleColor(ImGuiCol_ResizeGripHovered, v.resize_grip_hovered());
        pushes +=1;
    }
    if (v.resize_grip_active() != 0) {
        ImGui::PushStyleColor(ImGuiCol_ResizeGripActive, v.resize_grip_active());
        pushes +=1;
    }
    if (v.tab() != 0) {
        ImGui::PushStyleColor(ImGuiCol_Tab, v.tab());
        pushes +=1;
    }
    if (v.tab_hovered() != 0) {
        ImGui::PushStyleColor(ImGuiCol_TabHovered, v.tab_hovered());
        pushes +=1;
    }
    if (v.tab_active() != 0) {
        ImGui::PushStyleColor(ImGuiCol_TabActive, v.tab_active());
        pushes +=1;
    }
    if (v.tab_unfocused() != 0) {
        ImGui::PushStyleColor(ImGuiCol_TabUnfocused, v.tab_unfocused());
        pushes +=1;
    }
    if (v.tab_unfocused_active() != 0) {
        ImGui::PushStyleColor(ImGuiCol_TabUnfocusedActive, v.tab_unfocused_active());
        pushes +=1;
    }
    if (v.plot_lines() != 0) {
        ImGui::PushStyleColor(ImGuiCol_PlotLines, v.plot_lines());
        pushes +=1;
    }
    if (v.plot_lines_hovered() != 0) {
        ImGui::PushStyleColor(ImGuiCol_PlotLinesHovered, v.plot_lines_hovered());
        pushes +=1;
    }
    if (v.plot_histogram() != 0) {
        ImGui::PushStyleColor(ImGuiCol_PlotHistogram, v.plot_histogram());
        pushes +=1;
    }
    if (v.plot_histogram_hovered() != 0) {
        ImGui::PushStyleColor(ImGuiCol_PlotHistogramHovered, v.plot_histogram_hovered());
        pushes +=1;
    }
    if (v.table_header_bg() != 0) {
        ImGui::PushStyleColor(ImGuiCol_TableHeaderBg, v.table_header_bg());
        pushes +=1;
    }
    if (v.table_border_strong() != 0) {
        ImGui::PushStyleColor(ImGuiCol_TableBorderStrong, v.table_border_strong());
        pushes +=1;
    }
    if (v.table_border_light() != 0) {
        ImGui::PushStyleColor(ImGuiCol_TableBorderLight, v.table_border_light());
        pushes +=1;
    }
    if (v.table_row_bg() != 0) {
        ImGui::PushStyleColor(ImGuiCol_TableRowBg, v.table_row_bg());
        pushes +=1;
    }
    if (v.table_row_bg_alt() != 0) {
        ImGui::PushStyleColor(ImGuiCol_TableRowBgAlt, v.table_row_bg_alt());
        pushes +=1;
    }
    if (v.text_selected_bg() != 0) {
        ImGui::PushStyleColor(ImGuiCol_TextSelectedBg, v.text_selected_bg());
        pushes +=1;
    }
    if (v.drag_drop_target() != 0) {
        ImGui::PushStyleColor(ImGuiCol_DragDropTarget, v.drag_drop_target());
        pushes +=1;
    }
    if (v.nav_highlight() != 0) {
        ImGui::PushStyleColor(ImGuiCol_NavHighlight, v.nav_highlight());
        pushes +=1;
    }
    if (v.nav_windowing_highlight() != 0) {
        ImGui::PushStyleColor(ImGuiCol_NavWindowingHighlight, v.nav_windowing_highlight());
        pushes +=1;
    }
    if (v.nav_windowing_dim_bg() != 0) {
        ImGui::PushStyleColor(ImGuiCol_NavWindowingDimBg, v.nav_windowing_dim_bg());
        pushes +=1;
    }
    if (v.modal_window_dim_bg() != 0) {
        ImGui::PushStyleColor(ImGuiCol_ModalWindowDimBg, v.modal_window_dim_bg());
        pushes +=1;
    }
    return pushes;
}
