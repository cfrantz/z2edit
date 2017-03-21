#ifndef Z2UTIL_APP_H
#define Z2UTIL_APP_H
#include <memory>
#include <string>

#include "imwidget/imapp.h"
#include "imwidget/editor.h"
#include "imwidget/enemyattr.h"
#include "imwidget/hwpalette.h"
#include "imwidget/imwidget.h"
#include "imwidget/misc_hacks.h"
#include "imwidget/neschrview.h"
#include "imwidget/palace_gfx.h"
#include "imwidget/palette.h"
#include "imwidget/simplemap.h"
#include "imwidget/start_values.h"
#include "imwidget/object_table.h"
#include "imwidget/xptable.h"
#include "nes/cartridge.h"
#include "nes/mapper.h"
#include "nes/memory.h"

namespace z2util {

class Z2Edit: public ImApp {
  public:
    Z2Edit(const std::string& name) : ImApp(name, 1280, 720, false) {}
    ~Z2Edit() override {}

    void Init() override;
    void ProcessEvent(SDL_Event* event) override;
    void Draw() override;

    void Load(const std::string& filename);
    void Load(const std::string& filename, bool movekeepout);
    void Help(const std::string& topickey);
  private:
    void LoadFile(DebugConsole* console, int argc, char **argv);
    void SaveFile(DebugConsole* console, int argc, char **argv);
    void HexdumpBytes(DebugConsole* console, int argc, char **argv);
    void WriteBytes(DebugConsole* console, int argc, char **argv);
    void WriteMapper(DebugConsole* console, int argc, char **argv);
    void HexdumpWords(DebugConsole* console, int argc, char **argv);
    void WriteWords(DebugConsole* console, int argc, char **argv);
    void Unassemble(DebugConsole* console, int argc, char **argv);
    void Assemble(DebugConsole* console, int argc, char **argv);
    void EnemyList(DebugConsole* console, int argc, char **argv);
    void InsertPrg(DebugConsole* console, int argc, char **argv);
    void CopyPrg(DebugConsole* console, int argc, char **argv);
    void CopyChr(DebugConsole* console, int argc, char **argv);
    void MemMove(DebugConsole* console, int argc, char **argv);
    void Swap(DebugConsole* console, int argc, char **argv);
    void BCopy(DebugConsole* console, int argc, char **argv);
    void SetVar(DebugConsole* console, int argc, char **argv);
    void Source(DebugConsole* console, int argc, char **argv);
    void SpawnEmulator(const std::string& romfile);
    int EncodedText(int ch);

    bool loaded_;
    int ibase_;
    int bank_;
    int text_encoding_;
    std::string save_filename_;
    NesHardwarePalette* hwpal_;
    std::unique_ptr<NesChrView> chrview_;
    std::unique_ptr<z2util::SimpleMap> simplemap_;
    std::unique_ptr<z2util::Editor> editor_;
    std::unique_ptr<z2util::MiscellaneousHacks> misc_hacks_;
    std::unique_ptr<z2util::PalaceGraphics> palace_gfx_;
    std::unique_ptr<z2util::PaletteEditor> palette_editor_;
    std::unique_ptr<z2util::StartValues> start_values_;
    std::unique_ptr<z2util::ObjectTable> object_table_;
    std::unique_ptr<z2util::EnemyEditor> enemy_editor_;
    std::unique_ptr<z2util::ExperienceTable> experience_table_;

    Cartridge cartridge_;
    z2util::Memory memory_;
    std::unique_ptr<Mapper> mapper_;
};

}  // namespace z2util
#endif // Z2UTIL_APP_H
