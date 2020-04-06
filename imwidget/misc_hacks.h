#ifndef Z2UTIL_IMWIDGET_MISC_HACKS_H
#define Z2UTIL_IMWIDGET_MISC_HACKS_H
#include "imwidget/imwidget.h"
#include "proto/rominfo.pb.h"

class Mapper;
namespace z2util {

class MiscellaneousHacks: public ImWindowBase {
  public:
    MiscellaneousHacks(): ImWindowBase(false), tab_(0) { Init(); }

    void Init();
    void Refresh() override { CheckConfig(); }
    bool Draw() override;
    bool DrawMiscHacks();
    bool DrawDynamicBanks();
    void CheckConfig();
    inline void set_mapper(Mapper* m) { mapper_ = m; };

    const static int MAX_COLLECTABLE = 36;
  private:
    template<class GETALL, class GET>
    int Hack(const char* hackname, int n, GETALL getall, GET get);
    template<class GETALL>
    int EnabledIndex(GETALL getall);

    bool DrawPalaceCompletionItems();
    bool MemcmpHack(const PokeData& data);
    void PutPokeData(const PokeData& data);
    void PutGameHack(const GameHack& hack);

    int tab_;
    Mapper* mapper_;
    static const char *collectable_names_[MAX_COLLECTABLE];
};

}  // namespace
#endif // Z2UTIL_IMWIDGET_MISC_HACKS_H
