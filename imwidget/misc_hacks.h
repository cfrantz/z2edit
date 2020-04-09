#ifndef Z2UTIL_IMWIDGET_MISC_HACKS_H
#define Z2UTIL_IMWIDGET_MISC_HACKS_H
#include "imwidget/imwidget.h"
#include "proto/rominfo.pb.h"

class Mapper;
namespace z2util {

class MiscellaneousHacks: public ImWindowBase {
  public:
    MiscellaneousHacks(): ImWindowBase(false), tab_(0) {};

    void Refresh() override {
        CheckOverworldTileHack();
    }
    bool Draw() override;
    bool DrawMiscHacks();
    bool DrawDynamicBanks();
    bool DrawSwimEnables();
    void CheckOverworldTileHack();
    inline void set_mapper(Mapper* m) { mapper_ = m; };

  private:
    template<class GETALL, class GET, class CHECK>
    int Hack(const char* hackname, int n, GETALL getall, GET get, CHECK check);
    template<class GETALL>
    int EnabledIndex(GETALL getall);

    bool MemcmpHack(const PokeData& data);
    void PutPokeData(const PokeData& data);
    void PutGameHack(const GameHack& hack);

    int tab_;
    Mapper* mapper_;
};

}  // namespace
#endif // Z2UTIL_IMWIDGET_MISC_HACKS_H
