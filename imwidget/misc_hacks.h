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
    template<class GETALL, class GET>
    void Hack(const char* hackname, int n, GETALL getall, GET get);

    bool MemcmpHack(const PokeData& data);
    void PutPokeData(const PokeData& data);
    void PutGameHack(const GameHack& hack);
    Mapper* mapper_;
};

}  // namespace
#endif // Z2UTIL_IMWIDGET_MISC_HACKS_H
