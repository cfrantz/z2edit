ImGuiInputTextFlags GetInputTextFlags(const proto::InputTextFlags& flags) {
    ImGuiInputTextFlags f = 0;
    if (flags.chars_decimal()) f |= ImGuiInputTextFlags_CharsDecimal;
    if (flags.chars_hexadecimal()) f |= ImGuiInputTextFlags_CharsHexadecimal;
    if (flags.chars_uppercase()) f |= ImGuiInputTextFlags_CharsUppercase;
    if (flags.chars_no_blank()) f |= ImGuiInputTextFlags_CharsNoBlank;
    if (flags.auto_select_all()) f |= ImGuiInputTextFlags_AutoSelectAll;
    if (flags.enter_returns_true()) f |= ImGuiInputTextFlags_EnterReturnsTrue;
    if (flags.callback_completion()) f |= ImGuiInputTextFlags_CallbackCompletion;
    if (flags.callback_history()) f |= ImGuiInputTextFlags_CallbackHistory;
    if (flags.callback_always()) f |= ImGuiInputTextFlags_CallbackAlways;
    if (flags.callback_char_filter()) f |= ImGuiInputTextFlags_CallbackCharFilter;
    if (flags.allow_tab_input()) f |= ImGuiInputTextFlags_AllowTabInput;
    if (flags.ctrl_enter_for_new_line()) f |= ImGuiInputTextFlags_CtrlEnterForNewLine;
    if (flags.no_horizontal_scroll()) f |= ImGuiInputTextFlags_NoHorizontalScroll;
    if (flags.always_overwrite()) f |= ImGuiInputTextFlags_AlwaysOverwrite;
    if (flags.read_only()) f |= ImGuiInputTextFlags_ReadOnly;
    if (flags.password()) f |= ImGuiInputTextFlags_Password;
    if (flags.no_undo_redo()) f |= ImGuiInputTextFlags_NoUndoRedo;
    if (flags.chars_scientific()) f |= ImGuiInputTextFlags_CharsScientific;
    if (flags.callback_resize()) f |= ImGuiInputTextFlags_CallbackResize;
    if (flags.callback_edit()) f |= ImGuiInputTextFlags_CallbackEdit;
    if (flags.escape_clears_all()) f |= ImGuiInputTextFlags_EscapeClearsAll;
    return f;
}
