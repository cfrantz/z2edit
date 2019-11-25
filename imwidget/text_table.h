#ifndef Z2UTIL_IMWIDGET_TEXT_TABLE_H
#define Z2UTIL_IMWIDGET_TEXT_TABLE_H
#include <vector>
#include "imwidget/imwidget.h"
#include "nes/text_list.h"

class Mapper;
namespace z2util {

class TextTableEditor: public ImWindowBase {
  public:
    TextTableEditor()
        : ImWindowBase(false), mapper_(nullptr), changed_(false), world_(0) {}
    void Init();
    void Refresh() override { Init(); }
    bool Draw() override;
    int TotalLength();

    inline void set_mapper(Mapper* m) { mapper_ = m; }
  private:
    void Load();
    void Save();


    Mapper* mapper_;
    bool changed_;
    int world_;
    TextListPack pack_;
    const static int MAX_STRINGS = 128;
    char data_[MAX_STRINGS][128];

};

}  // namespace z2util
#endif // Z2UTIL_IMWIDGET_TEXT_TABLE_H
