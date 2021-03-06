#include "imwidget/project.h"

#include "google/protobuf/text_format.h"
#include "imwidget/imapp.h"
#include "imwidget/error_dialog.h"
#include "ips/ips.h"
#include "nes/cartridge.h"
#include "util/compress.h"
#include "util/config.h"
#include "util/file.h"
#include "util/logging.h"
#include "util/os.h"
#include "util/status.h"
#include "util/statusor.h"

using google::protobuf::TextFormat;
namespace z2util {

void Project::Init() {}
bool Project::Draw() {
    if (!visible_)
        return changed_;

    ImGui::Begin("Project", &visible_);
    if (ImGui::Button("Manual Commit")) {
        Commit("Manual Commit");
    }
    ImApp::Get()->HelpButton("project", true);

    char name[100];
    strncpy(name, project_.name().c_str(), 99);
    if (ImGui::InputText("Project Name", name, sizeof(name))) {
        project_.set_name(name);
    }

    char items_text[project_.history_size()][100];
    const char *items[project_.history_size()];
    char descr[100];
    int n = 0;
    for(const auto& h : project_.history()) {
        if (n == selection_) {
            strncpy(descr, h.description().c_str(), 99);
            descr[99] = 0;
        }
        items[n] = items_text[n];
        snprintf(items_text[n++], 100, "%s: %s",
                os::CTime(h.create_time()).c_str(), h.description().c_str());
    }
    ImGui::ListBox("Commit History", &selection_, items, n, 15);

    if (ImGui::Button("Make Selection Current")) {
        auto rom = ZLib::Uncompress(project_.history(selection_).rom());
        if (rom.ok()) {
            cartridge_->LoadRom(rom.ValueOrDie());
            ImApp::Get()->ProcessMessage("loadpostprocess",
                    reinterpret_cast<void*>(0));
        }
    }
    ImGui::SameLine();
    auto* history = project_.mutable_history();
    if (ImGui::Button("Delete Selection")) {
        history->erase(history->begin() + selection_);
    }
    if (ImGui::InputText("Description", descr, sizeof(descr))) {
        history->Mutable(selection_)->set_description(descr);
    }
    ImGui::End();
    return false;
}

bool Project::Load(const std::string& filename, bool with_warnings) {
    if (with_warnings) {
        if (changed_) {
            ErrorDialog::Spawn("Discard Changes",
                ErrorDialog::OK | ErrorDialog::CANCEL,
                "The project has unsaved changes.\nContinue?")->set_result_cb(
                    [this, filename](int result) {
                        if (result == ErrorDialog::OK) {
                            LoadWorker(filename);
                        }
                    });
            return true;

        }
        if (Cartridge::IsNESFile(filename)) {
            ErrorDialog::Spawn("Start New Project",
                ErrorDialog::OK | ErrorDialog::CANCEL,
                "Loading a NES file starts a new project.\n"
                "If you want to import a NES file into the current project,\n"
                "press 'Cancel' and select 'Import ROM'.\n")->set_result_cb(
                    [this, filename](int result) {
                        if (result == ErrorDialog::OK) {
                            LoadWorker(filename);
                        }
                    });
            return true;
        }
    }
    return LoadWorker(filename);
}

bool Project::LoadWorker(const std::string& filename) {
    if (Cartridge::IsNESFile(filename)) {
        project_.Clear();
        project_.set_name("New Project");
        cartridge_->LoadFile(filename);
        Commit("Unmodified ROM");
    } else {
        std::string content;
        if (!File::GetContents(filename, &content)) {
            LOG(ERROR, "Could not load project: ", filename);
            return false;
        }
        if (!project_.ParseFromString(content)) {
            LOG(INFO, "Could not parse project, trying TextFormat");
            if (!TextFormat::ParseFromString(content, &project_)) {
                LOG(ERROR, "Could not parse project from ", filename);
                return false;
            }
        }
#if 0
        LOG(INFO, "Loaded project: ", project_.name());
        LOG(INFO, "  compressed rom is ", project_.rom().size(), " bytes");
        File::SetContents(absl::StrCat(project_.name(), "-rom.nes"), project_.rom());
        for(int i=0; i<project_.history_size(); i++) {
            File::SetContents(absl::StrCat(project_.name(), "-", i, ".nes"), project_.history(i).rom());
            LOGF(INFO, "  commit %d: %s", i, project_.history(i).description().c_str());
            LOGF(INFO, "             created %lld", project_.history(i).create_time());
            LOGF(INFO, "             size %u", project_.history(i).rom().size());
        }
#endif
        auto rom = ZLib::Uncompress(project_.rom());
        if (rom.ok()) {
            cartridge_->LoadRom(rom.ValueOrDie());
        } else {
            LOG(ERROR, filename, ": error uncompressing ROM in project: ",
                    rom.status().ToString());
        }
    }
    *ConfigLoader<SessionConfig>::Get()->MutableConfig() = project_.settings();
    ImApp::Get()->ProcessMessage("loadpostprocess", reinterpret_cast<void*>(-1));
    return true;
}

bool Project::Save(const std::string& filename, bool as_text) {
    std::string content;
    project_.set_rom(ZLib::Compress(cartridge_->SaveRom()));
    *project_.mutable_settings() =
        ConfigLoader<SessionConfig>::Get()->GetConfig();
    if (as_text) {
        TextFormat::PrintToString(project_, &content);
    } else {
        project_.SerializeToString(&content);
    }
    bool ret = File::SetContents(filename, content);
    if (ret) {
        changed_ = false;
    } else {
        LOG(ERROR, "Could not save project: ", filename);
    }
    return ret; 
}

bool Project::ImportRom(const std::string& filename) {
    Commit(absl::StrCat("Before import of ", filename));
    cartridge_->LoadFile(filename);
    ImApp::Get()->ProcessMessage("loadpostprocess", reinterpret_cast<void*>(-1));
    return true;
}

bool Project::ExportRom(const std::string& filename) {
    cartridge_->SaveFile(filename);
    return true;
}

util::Status Project::ExportIps(const std::string& filename, int original, int modified) {
    auto orig = rom(original);
    if (!orig.ok()) {
        return orig.status();
    }
    auto mod = rom(modified);
    if (!mod.ok()) {
        return mod.status();
    }

    std::string patch = ips::CreatePatch(orig.ValueOrDie(), mod.ValueOrDie());
    if (!File::SetContents(filename, patch)) {
        return util::Status(util::error::Code::UNKNOWN, "Could not save file");
    }
    return util::Status();
}


StatusOr<std::string> Project::rom(int n) {
    if (n == 0) {
        return cartridge_->SaveRom();
    }
    if (n < 0) {
        // Negative indexes:
        // -1 = newest commit
        // -2 = second newest commit
        // etc..
        n = project_.history_size() + n;
    } else {
        // Positive indexes:
        // 1 = first commit, etc...
        n -= 1;
    }
    if (n < 0 || n >= project_.history_size()) {
        return util::Status(util::error::Code::INVALID_ARGUMENT,
                            "Invalid history index");
    }
    return ZLib::Uncompress(project_.history(n).rom());
}

void Project::Commit(const std::string& message) {
    auto* commit = project_.add_history();
    commit->set_create_time(os::utime_now());
    commit->set_description(message);
    commit->set_rom(ZLib::Compress(cartridge_->SaveRom()));
}

}  // z2util
