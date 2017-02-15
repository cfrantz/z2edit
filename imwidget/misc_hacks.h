#ifndef Z2UTIL_IMWIDGET_MISC_HACKS_H
#define Z2UTIL_IMWIDGET_MISC_HACKS_H
#include "imwidget/imwidget.h"
#include "proto/rominfo.pb.h"

class Mapper;
namespace z2util {

class MiscellaneousHacks: public ImWindowBase {
  public:
    MiscellaneousHacks(): ImWindowBase(false) {};

    bool Draw() override;
    inline void set_mapper(Mapper* m) { mapper_ = m; };
  private:
    void Palace5Hack();
    void PalaceContinueHack();

    bool MemcmpHack(const PokeData& data);
    void PutPokeData(const PokeData& data);
    void PutGameHack(const GameHack& hack);
    Mapper* mapper_;
};

}  // namespace
#endif // Z2UTIL_IMWIDGET_MISC_HACKS_H
