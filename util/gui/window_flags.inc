ImGuiWindowFlags GetWindowFlags(const proto::WindowFlags& flags) {
    ImGuiWindowFlags f = 0;
    if (flags.no_title_bar()) f |= ImGuiWindowFlags_NoTitleBar;
    if (flags.no_resize()) f |= ImGuiWindowFlags_NoResize;
    if (flags.no_move()) f |= ImGuiWindowFlags_NoMove;
    if (flags.no_scrollbar()) f |= ImGuiWindowFlags_NoScrollbar;
    if (flags.no_scroll_with_mouse()) f |= ImGuiWindowFlags_NoScrollWithMouse;
    if (flags.no_collapse()) f |= ImGuiWindowFlags_NoCollapse;
    if (flags.always_auto_resize()) f |= ImGuiWindowFlags_AlwaysAutoResize;
    if (flags.no_background()) f |= ImGuiWindowFlags_NoBackground;
    if (flags.no_saved_settings()) f |= ImGuiWindowFlags_NoSavedSettings;
    if (flags.no_mouse_inputs()) f |= ImGuiWindowFlags_NoMouseInputs;
    if (flags.menu_bar()) f |= ImGuiWindowFlags_MenuBar;
    if (flags.horizontal_scrollbar()) f |= ImGuiWindowFlags_HorizontalScrollbar;
    if (flags.no_focus_on_appearing()) f |= ImGuiWindowFlags_NoFocusOnAppearing;
    if (flags.no_bring_to_front_on_focus()) f |= ImGuiWindowFlags_NoBringToFrontOnFocus;
    if (flags.always_vertical_scrollbar()) f |= ImGuiWindowFlags_AlwaysVerticalScrollbar;
    if (flags.always_horizontal_scrollbar()) f |= ImGuiWindowFlags_AlwaysHorizontalScrollbar;
    if (flags.always_use_window_padding()) f |= ImGuiWindowFlags_AlwaysUseWindowPadding;
    if (flags.no_nav_inputs()) f |= ImGuiWindowFlags_NoNavInputs;
    if (flags.no_nav_focus()) f |= ImGuiWindowFlags_NoNavFocus;
    if (flags.unsaved_document()) f |= ImGuiWindowFlags_UnsavedDocument;
    return f;
}
