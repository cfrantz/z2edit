ImGuiColorEditFlags GetColorEditFlags(const proto::ColorEditFlags& flags) {
    ImGuiColorEditFlags f = 0;
    if (flags.no_alpha()) f |= ImGuiColorEditFlags_NoAlpha;
    if (flags.no_picker()) f |= ImGuiColorEditFlags_NoPicker;
    if (flags.no_options()) f |= ImGuiColorEditFlags_NoOptions;
    if (flags.no_small_preview()) f |= ImGuiColorEditFlags_NoSmallPreview;
    if (flags.no_inputs()) f |= ImGuiColorEditFlags_NoInputs;
    if (flags.no_tooltip()) f |= ImGuiColorEditFlags_NoTooltip;
    if (flags.no_label()) f |= ImGuiColorEditFlags_NoLabel;
    if (flags.no_side_preview()) f |= ImGuiColorEditFlags_NoSidePreview;
    if (flags.no_drag_drop()) f |= ImGuiColorEditFlags_NoDragDrop;
    if (flags.no_border()) f |= ImGuiColorEditFlags_NoBorder;
    if (flags.alpha_bar()) f |= ImGuiColorEditFlags_AlphaBar;
    if (flags.alpha_preview()) f |= ImGuiColorEditFlags_AlphaPreview;
    if (flags.alpha_preview_half()) f |= ImGuiColorEditFlags_AlphaPreviewHalf;
    if (flags.hdr()) f |= ImGuiColorEditFlags_HDR;
    if (flags.display_rgb()) f |= ImGuiColorEditFlags_DisplayRGB;
    if (flags.display_hsv()) f |= ImGuiColorEditFlags_DisplayHSV;
    if (flags.display_hex()) f |= ImGuiColorEditFlags_DisplayHex;
    if (flags.uint8()) f |= ImGuiColorEditFlags_Uint8;
    if (flags.float_()) f |= ImGuiColorEditFlags_Float;
    if (flags.picker_hue_bar()) f |= ImGuiColorEditFlags_PickerHueBar;
    if (flags.picker_hue_wheel()) f |= ImGuiColorEditFlags_PickerHueWheel;
    if (flags.input_rgb()) f |= ImGuiColorEditFlags_InputRGB;
    if (flags.input_hsv()) f |= ImGuiColorEditFlags_InputHSV;
    return f;
}
