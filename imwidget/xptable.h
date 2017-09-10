#ifndef Z2UTIL_IMWIDGET_XPTABLE_H
#define Z2UTIL_IMWIDGET_XPTABLE_H
#include <functional>
#include <vector>
#include "imwidget/imwidget.h"
#include "proto/rominfo.pb.h"

class Mapper;
namespace z2util {

class ExperienceTable: public ImWindowBase {
  public:
    ExperienceTable()
      : ImWindowBase(false), changed_(false) {}
    void Init();

    void Refresh() override { Init(); }
    bool Draw() override;

    inline void set_mapper(Mapper* m) { mapper_ = m; }

  private:
    void LoadSaveWorker(
        std::function<void(const char*, const Address&)>,
        std::function<void(const char*, const Address&, int)>,
        std::function<void(char*, const Address&, int)>);
    void Load();
    void Save();
    void Sanitize(char *name);

    struct Unpacked {
        const char *name;
        int val[8];
    };
    Mapper* mapper_;
    bool changed_;
    std::vector<Unpacked> data_;
    char spell_names_[8][9];
};

}  // z2util
#endif // Z2UTIL_IMWIDGET_XPTABLE_H
