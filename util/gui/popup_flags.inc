ImGuiPopupFlags GetPopupFlags(const proto::PopupFlags& flags) {
    ImGuiPopupFlags f = 0;
    if (flags.mouse_button_left()) f |= ImGuiPopupFlags_MouseButtonLeft;
    if (flags.mouse_button_right()) f |= ImGuiPopupFlags_MouseButtonRight;
    if (flags.mouse_button_middle()) f |= ImGuiPopupFlags_MouseButtonMiddle;
    if (flags.no_open_over_existing_popup()) f |= ImGuiPopupFlags_NoOpenOverExistingPopup;
    if (flags.no_open_over_items()) f |= ImGuiPopupFlags_NoOpenOverItems;
    if (flags.any_popup_id()) f |= ImGuiPopupFlags_AnyPopupId;
    if (flags.any_popup_level()) f |= ImGuiPopupFlags_AnyPopupLevel;
    if (flags.any_popup()) f |= ImGuiPopupFlags_AnyPopup;
    return f;
}
