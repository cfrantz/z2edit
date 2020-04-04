#ifndef Z2UTIL_APP_H
#define Z2UTIL_APP_H
#include <memory>
#include <string>

#include "imwidget/imapp.h"
#include "imwidget/drops.h"
#include "imwidget/editor.h"
#include "imwidget/enemyattr.h"
#include "imwidget/hwpalette.h"
#include "imwidget/imwidget.h"
#include "imwidget/item_effects.h"
#include "imwidget/misc_hacks.h"
#include "imwidget/neschrview.h"
#include "imwidget/palace_gfx.h"
#include "imwidget/palette.h"
#include "imwidget/project.h"
#include "imwidget/rom_memory.h"
#include "imwidget/simplemap.h"
#include "imwidget/start_values.h"
#include "imwidget/text_table.h"
#include "imwidget/tile_transform.h"
#include "imwidget/object_table.h"
#include "imwidget/xptable.h"
#include "nes/cartridge.h"
#include "nes/mapper.h"
#include "nes/memory.h"

namespace z2util {

class Z2Edit: public ImApp {
  public:
    Z2Edit(const std::string& name) : ImApp(name, 1280, 720) {}
    ~Z2Edit() override {}

    void Init() override;
    void ProcessEvent(SDL_Event* event) override;
    void ProcessMessage(const std::string& msg, const void* extra) override;
    void Draw() override;

    void Load(const std::string& filename);

    // movekeepout is tri-state:
    // -1: default action based on FLAGS_move_from_keepout
    // 0: do not move from keepouts
    // 1: move from keepouts
    void LoadPostProcess(int movekeepout);
    void Help(const std::string& topickey);
  private:
    void LoadFile(DebugConsole* console, int argc, char **argv);
    void SaveFile(DebugConsole* console, int argc, char **argv);
    void HexdumpBytes(DebugConsole* console, int argc, char **argv);
    void WriteBytes(DebugConsole* console, int argc, char **argv);
    void WriteText(DebugConsole* console, int argc, char **argv);
    void WriteMapper(DebugConsole* console, int argc, char **argv);
    void HexdumpWords(DebugConsole* console, int argc, char **argv);
    void WriteWords(DebugConsole* console, int argc, char **argv);
    void Unassemble(DebugConsole* console, int argc, char **argv);
    void Assemble(DebugConsole* console, int argc, char **argv);
    void EnemyList(DebugConsole* console, int argc, char **argv);
    void InsertPrg(DebugConsole* console, int argc, char **argv);
    void CopyPrg(DebugConsole* console, int argc, char **argv);
    void InsertChr(DebugConsole* console, int argc, char **argv);
    void CopyChr(DebugConsole* console, int argc, char **argv);
    void CharClear(DebugConsole* console, int argc, char **argv);
    void CharCopy(DebugConsole* console, int argc, char **argv);
    void MemMove(DebugConsole* console, int argc, char **argv);
    void Swap(DebugConsole* console, int argc, char **argv);
    void BCopy(DebugConsole* console, int argc, char **argv);
    void SetVar(DebugConsole* console, int argc, char **argv);
    void Source(DebugConsole* console, int argc, char **argv);
    void RestoreBank(DebugConsole* console, int argc, char **argv);
    void DumpTownText(DebugConsole* console, int argc, char **argv);
    void ConnTable(DebugConsole* console, int argc, char **argv);
    void SendMessage(DebugConsole* console, int argc, char **argv);
    void SpawnEmulator();
    void SpawnEmulator(uint8_t bank, uint8_t region, uint8_t world,
        uint8_t town_code, uint8_t palace_code, uint8_t connector,
        uint8_t room);
    int EncodedText(int ch);
    bool ParseChr(const std::string& a, int* bank, uint8_t *addr);

    bool loaded_;
    int ibase_;
    int bank_;
    int chrbank_;
    int text_encoding_;
    std::string save_filename_;
    std::string export_filename_;
    NesHardwarePalette* hwpal_;
    std::unique_ptr<NesChrView> chrview_;
    std::unique_ptr<z2util::SimpleMap> simplemap_;
    std::unique_ptr<z2util::Drops> drops_;
    std::unique_ptr<z2util::Editor> editor_;
    std::unique_ptr<z2util::MiscellaneousHacks> misc_hacks_;
    std::unique_ptr<z2util::PalaceGraphics> palace_gfx_;
    std::unique_ptr<z2util::PaletteEditor> palette_editor_;
    std::unique_ptr<z2util::RomMemory> rom_memory_;
    std::unique_ptr<z2util::StartValues> start_values_;
    std::unique_ptr<z2util::TextTableEditor> text_table_;
    std::unique_ptr<z2util::TileTransform> tile_transform_;
    std::unique_ptr<z2util::ItemEffects> item_effects_;
    std::unique_ptr<z2util::ObjectTable> object_table_;
    std::unique_ptr<z2util::EnemyEditor> enemy_editor_;
    std::unique_ptr<z2util::ExperienceTable> experience_table_;

    Cartridge cartridge_;
    Project project_;
    z2util::Memory memory_;
    std::unique_ptr<Mapper> mapper_;
};

}  // namespace z2util
#endif // Z2UTIL_APP_H
