#include <stdarg.h>
#include <stdlib.h>
#include "imwidget/debug_console.h"
#include "util/logging.h"

DebugConsole::DebugConsole(const char* name)
  : ImWindowBase(false, false),
    name_(name) {
    ClearLog();
    memset(inputbuf_, 0, sizeof(inputbuf_));
    history_pos_ = -1;
    AddLog("Welcome to %s!", name_);
    shorthelp_.insert(std::make_pair("clear", "Clear the window"));
    shorthelp_.insert(std::make_pair("history", "Show command history"));
    shorthelp_.insert(std::make_pair("help", "Show command help"));
}

DebugConsole::~DebugConsole() {
    ClearLog();
    for (int i = 0; i < history_.Size; i++)
        free(history_[i]);
}

void DebugConsole::PushLineCallback(std::function<
            void(DebugConsole* console, const char* line)> line_cb) {
    line_cb_.push_back(line_cb);
}

void DebugConsole::PopLineCallback() {
    line_cb_.pop_back();
}

void  DebugConsole::ClearLog() {
    for (int i = 0; i < items_.Size; i++)
        free(items_[i]);
    items_.clear();
    scroll_to_bottom_ = true;
}

void DebugConsole::RegisterCommand(const char* command, const char* shorthelp,
        std::function<void(DebugConsole*, int, char**)> fn) {
    shorthelp_.insert(std::make_pair(command, shorthelp));
    commands_.insert(std::make_pair(command, fn));
}

void  DebugConsole::AddLog(const char* fmt, ...) {
    char buf[1024];
    va_list args;
    va_start(args, fmt);
    vsnprintf(buf, sizeof(buf), fmt, args);
    buf[sizeof(buf)-1] = 0;
    va_end(args);
    items_.push_back(strdup(buf));
    scroll_to_bottom_ = true;

    char *log = buf;
    if (log[0] == '#' && buf[1] == '{')
        log += 6;

    char *end = log + strlen(log) - 1;
    while(end > log && (*end == '\r' || *end == '\n')) {
        *end-- = '\0';
    }
    LOG(INFO, "$$ ", log);
}

bool DebugConsole::Draw() {
    if (!visible_)
        return false;

    ImGui::SetNextWindowSize(ImVec2(520,600), ImGuiSetCond_FirstUseEver);
    if (!ImGui::Begin(name_, &visible_)) {
        ImGui::End();
        return false;
    }

    ImGui::TextWrapped("Enter 'HELP' for help, press TAB to use text completion.");

    // TODO: display items starting from the bottom
    if (ImGui::SmallButton("Clear")) ClearLog();
    ImGui::SameLine();
    if (ImGui::SmallButton("Scroll to bottom")) scroll_to_bottom_ = true;
    //static float t = 0.0f; if (ImGui::GetTime() - t > 0.02f) { t = ImGui::GetTime(); AddLog("Spam %f", t); }

    ImGui::Separator();

    ImGui::PushStyleVar(ImGuiStyleVar_FramePadding, ImVec2(0,0));
    static ImGuiTextFilter filter;
    filter.Draw("Filter (\"incl,-excl\") (\"error\")", 180);
    ImGui::PopStyleVar();
    ImGui::Separator();

    ImGui::BeginChild("ScrollingRegion", ImVec2(0,
                      -ImGui::GetItemsLineHeightWithSpacing()), false,
                      ImGuiWindowFlags_HorizontalScrollbar);
    if (ImGui::BeginPopupContextWindow()) {
        if (ImGui::Selectable("Clear")) ClearLog();
        ImGui::EndPopup();
    }

    // Display every line as a separate entry so we can change their color or add custom widgets. If you only want raw text you can use ImGui::TextUnformatted(log.begin(), log.end());
    // NB- if you have thousands of entries this approach may be too inefficient and may require user-side clipping to only process visible items.
    // You can seek and display only the lines that are visible using the ImGuiListClipper helper, if your elements are evenly spaced and you have cheap random access to the elements.
    // To use the clipper we could replace the 'for (int i = 0; i < items_.Size; i++)' loop with:
    //     ImGuiListClipper clipper(items_.Size);
    //     while (clipper.Step())
    //         for (int i = clipper.DisplayStart; i < clipper.DisplayEnd; i++)
    // However take note that you can not use this code as is if a filter is active because it breaks the 'cheap random-access' property. We would need random-access on the post-filtered list.
    // A typical application wanting coarse clipping and filtering may want to pre-compute an array of indices that passed the filtering test, recomputing this array when user changes the filter,
    // and appending newly elements as they are inserted. This is left as a task to the user until we can manage to improve this example code!
    // If your items are of variable size you may want to implement code similar to what ImGuiListClipper does. Or split your data into fixed height items to allow random-seeking into your list.
    ImGui::PushStyleVar(ImGuiStyleVar_ItemSpacing, ImVec2(4,1)); // Tighten spacing
    for (int i = 0; i < items_.Size; i++) {
        const char* item = items_[i];
        if (!filter.PassFilter(item))
            continue;
        ImVec4 col = ImVec4(1.0f,1.0f,1.0f,
                            1.0f); // A better implementation may store a type per-item. For the sample let's just parse the text.
        if (strstr(item, "[error]")) col = ImColor(1.0f,0.4f,0.4f,1.0f);
        else if (strncmp(item, "# ", 2) == 0) col = ImColor(1.0f,0.78f,0.58f,1.0f);
        else if (strncmp(item, "#{", 2) == 0) { // && item[5] == '}') {
            unsigned cval = strtoul(item+2, 0, 16);
            col = ImColor(float(cval & 0xF00) / float(0xF00),
                          float(cval & 0x0F0) / float(0x0F0),
                          float(cval & 0x00F) / float(0x00F),
                          1.0f);
            item += 6;
        }
        ImGui::PushStyleColor(ImGuiCol_Text, col);
        ImGui::TextUnformatted(item);
        ImGui::PopStyleColor();
    }
    if (scroll_to_bottom_)
        ImGui::SetScrollHere();
    scroll_to_bottom_ = false;
    ImGui::PopStyleVar();
    ImGui::EndChild();
    ImGui::Separator();

    // Command-line
    if (ImGui::InputText("Input", inputbuf_, sizeof(inputbuf_),
                         ImGuiInputTextFlags_EnterReturnsTrue|ImGuiInputTextFlags_CallbackCompletion|ImGuiInputTextFlags_CallbackHistory,
                         &TextEditCallbackStub, (void*)this)) {
        char* input_end = inputbuf_+strlen(inputbuf_);
        while (input_end > inputbuf_ && input_end[-1] == ' ') input_end--;
        *input_end = 0;
        if (inputbuf_[0])
            ExecCommand(inputbuf_);
        strcpy(inputbuf_, "");
    }

    // Demonstrate keeping auto focus on the input box
    if (ImGui::IsItemHovered() || (ImGui::IsRootWindowOrAnyChildFocused()
                                   && !ImGui::IsAnyItemActive() && !ImGui::IsMouseClicked(0)))
        ImGui::SetKeyboardFocusHere(-1); // Auto focus previous widget

    ImGui::End();
    return false;
}

void  DebugConsole::ExecCommand(const char* command_line) {
    if (!line_cb_.empty()) {
        line_cb_.back()(this, command_line);
        return;
    }
    AddLog("#{fc8}%s\n", command_line);

    // Insert into history. First find match and delete it so it can be pushed to the back. This isn't trying to be smart or optimal.
    history_pos_ = -1;
    for (int i = history_.Size-1; i >= 0; i--)
        if (strcasecmp(history_[i], command_line) == 0) {
            free(history_[i]);
            history_.erase(history_.begin() + i);
            break;
        }
    history_.push_back(strdup(command_line));

    // Process command
    if (strcasecmp(command_line, "CLEAR") == 0) {
        ClearLog();
    } else if (strcasecmp(command_line, "HELP") == 0) {
        AddLog("Commands:");
        for(const auto& c : shorthelp_) {
            AddLog("- %s: %s", c.first, c.second);
        }
    } else if (strcasecmp(command_line, "HISTORY") == 0) {
        for (int i = history_.Size >= 10 ? history_.Size - 10 : 0; i < history_.Size;
                i++)
            AddLog("%3d: %s\n", i, history_[i]);
    } else {
        int argc = 0;
        char *argv[128];
        char *command = strdup(command_line);
        char *orig = command;

        argv[argc] = nullptr;
        while(*command) {
            while(isspace(*command))
                *command++ = '\0';

            if (*command == '#') {
                *command++ = '\0';
                break;
            }

            if (*command) {
                bool quote = *command == '"';
                if (quote) {
                    argv[argc++] = ++command;
                    while(*command && *command != '"') {
                        command +=  (*command == '\\' && command[1]) ? 2 : 1;
                    }
                    if (*command == '"') {
                        *command++ = '\0';
                    } else {
                        AddLog("[error] Unterminated string.");
                        argc = 0;
                        break;
                    }
                } else {
                    argv[argc++] = command;
                    while(*command && !isspace(*command)) {
                        command++;
                    }
                }
            } else {
                break;
            }
        }
        if (argc && strlen(argv[0])) {
            for(const auto& c : commands_) {
                if (!strcasecmp(argv[0], c.first)) {
                    c.second(this, argc, argv);
                    free(orig);
                    return;
                }
            }
            AddLog("Unknown command: '%s'\n", argv[0]);
        }
        free(orig);
    }
}

int DebugConsole::TextEditCallbackStub(ImGuiTextEditCallbackData* data) {
    // In C++11 you are better off using lambdas for this sort of forwarding callbacks
    DebugConsole* console = (DebugConsole*)data->UserData;
    return console->TextEditCallback(data);
}

int  DebugConsole::TextEditCallback(ImGuiTextEditCallbackData* data) {
    //AddLog("cursor: %d, selection: %d-%d", data->CursorPos, data->SelectionStart, data->SelectionEnd);
    switch (data->EventFlag) {
    case ImGuiInputTextFlags_CallbackCompletion: {
        // Example of TEXT COMPLETION

        // Locate beginning of current word
        const char* word_end = data->Buf + data->CursorPos;
        const char* word_start = word_end;
        while (word_start > data->Buf) {
            const char c = word_start[-1];
            if (c == ' ' || c == '\t' || c == ',' || c == ';')
                break;
            word_start--;
        }

        // Build a list of candidates
        ImVector<const char*> candidates;
        for(const auto& c : commands_) {
            if (strncasecmp(c.first, word_start, (int)(word_end-word_start)) == 0)
                candidates.push_back(c.first);
        }

        if (candidates.Size == 0) {
            // No match
            AddLog("No match for \"%.*s\"!\n", (int)(word_end-word_start), word_start);
        } else if (candidates.Size == 1) {
            // Single match. Delete the beginning of the word and replace it entirely so we've got nice casing
            data->DeleteChars((int)(word_start-data->Buf), (int)(word_end-word_start));
            data->InsertChars(data->CursorPos, candidates[0]);
            data->InsertChars(data->CursorPos, " ");
        } else {
            // Multiple matches. Complete as much as we can, so inputing "C" will complete to "CL" and display "CLEAR" and "CLASSIFY"
            int match_len = (int)(word_end - word_start);
            for (;;) {
                int c = 0;
                bool all_candidates_matches = true;
                for (int i = 0; i < candidates.Size && all_candidates_matches; i++)
                    if (i == 0)
                        c = toupper(candidates[i][match_len]);
                    else if (c != toupper(candidates[i][match_len]))
                        all_candidates_matches = false;
                if (!all_candidates_matches)
                    break;
                match_len++;
            }

            if (match_len > 0) {
                data->DeleteChars((int)(word_start - data->Buf), (int)(word_end-word_start));
                data->InsertChars(data->CursorPos, candidates[0], candidates[0] + match_len);
            }

            // List matches
            AddLog("Possible matches:\n");
            for (int i = 0; i < candidates.Size; i++)
                AddLog("- %s\n", candidates[i]);
        }

        break;
    }
    case ImGuiInputTextFlags_CallbackHistory: {
        // Example of HISTORY
        const int prev_history_pos = history_pos_;
        if (data->EventKey == ImGuiKey_UpArrow) {
            if (history_pos_ == -1)
                history_pos_ = history_.Size - 1;
            else if (history_pos_ > 0)
                history_pos_--;
        } else if (data->EventKey == ImGuiKey_DownArrow) {
            if (history_pos_ != -1)
                if (++history_pos_ >= history_.Size)
                    history_pos_ = -1;
        }

        // A better implementation would preserve the data on the current input line along with cursor position.
        if (prev_history_pos != history_pos_) {
            data->CursorPos = data->SelectionStart = data->SelectionEnd = data->BufTextLen
                              = (int)snprintf(data->Buf, (size_t)data->BufSize, "%s",
                                              (history_pos_ >= 0) ? history_[history_pos_] : "");
            data->BufDirty = true;
        }
    }
    }
    return 0;
}
