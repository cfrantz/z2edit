#include <cstdio>

#include "app.h"
#include "imgui.h"
#include "absl/log/log.h"
#include "absl/strings/match.h"
#include "imwidget/error_dialog.h"
#include "util/browser.h"
#include "util/os.h"

#include "version.h"

#ifdef HAVE_NFD
#include "nfd.h"
#endif


namespace project {

void App::Init() {
}

void App::ProcessEvent(SDL_Event* event) {
}

void App::ProcessMessage(const std::string& msg, const void* extra) {
}

void App::Draw() {
    ImGui::SetNextWindowSize(ImVec2(500,300), ImGuiCond_FirstUseEver);
    if (ImGui::BeginMainMenuBar()) {
        if (ImGui::BeginMenu("File")) {
            if (ImGui::MenuItem("Open", "Ctrl+O")) {
#ifdef HAVE_NFD
                char *filename = nullptr;
                auto result = NFD_OpenDialog("z2prj;nes", nullptr, &filename);
                if (result == NFD_OKAY) {
                    // DOSTUFF
                    save_filename_.assign(filename);
                }
                free(filename);
#else
            LOG(ERROR) << "File|Open";
#endif
            }

            if (ImGui::MenuItem("Save", "Ctrl+S")) {
                if (save_filename_.empty())
                    goto save_as;
                // DOSTUFF
            }
            if (ImGui::MenuItem("Save As")) {
save_as:
#ifdef HAVE_NFD
                char *filename = nullptr;
                auto result = NFD_SaveDialog("z2prj", nullptr, &filename);
                if (result == NFD_OKAY) {
                    std::string savefile = filename;
                    if (absl::EndsWith(savefile, ".z2prj")) {
                        save_filename_.assign(savefile);
                        // DOSTUFF
                    } else {
                        ErrorDialog::Spawn("Bad File Extension",
                            ErrorDialog::OK | ErrorDialog::CANCEL,
                            "Project files should have the extension .z2prj\n"
                            "If you want to save a .nes file, use File | Export\n\n"
                            "Press 'OK' to save anyway.\n")->set_result_cb(
                                [=](int result) {
                                    if (result == ErrorDialog::OK) {
                                        save_filename_.assign(savefile);
                                        // DOSTUFF
                                    }
                                });
                    }
                }
                free(filename);
#else
            LOG(ERROR) << "File|SaveAs";
#endif
            }
            ImGui::EndMenu();
        }
        if (ImGui::BeginMenu("Edit")) {
            ImGui::MenuItem("Debug Console", nullptr,
                            &console_.visible());
            ImGui::EndMenu();
        }
        if (ImGui::BeginMenu("View")) {
            ImGui::EndMenu();
        }
        if (ImGui::BeginMenu("Help")) {
            if (ImGui::MenuItem("Online Help")) {
                Help("root");
            }
            if (ImGui::MenuItem("About")) {
                ErrorDialog::Spawn("About App",
                    "Empty Project\n\n",
#ifdef BUILD_GIT_VERSION
                    "Version: ", BUILD_GIT_VERSION, "-", BUILD_SCM_STATUS
#else
                    "Version: Unknown"
#warning "Built without version stamp"
#endif
                    );

            }
            ImGui::EndMenu();
        }
        ImGui::EndMainMenuBar();
    }

#if 0
    if (!loaded_) {
        char *filename = nullptr;
        auto result = NFD_OpenDialog("z2prj,nes", nullptr, &filename);
        if (result == NFD_OKAY) {
            project_.Load(filename, false);
            if (ends_with(filename, ".z2prj")) {
                save_filename_.assign(filename);
            }
        }
        free(filename);
    }
#endif
}

void App::Help(const std::string& topickey) {
}

}  // namespace project
