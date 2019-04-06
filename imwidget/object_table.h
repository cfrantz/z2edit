#ifndef Z2UTIL_IMWIDGET_OBJECT_TABLE_H
#define Z2UTIL_IMWIDGET_OBJECT_TABLE_H
#include <vector>

#include "imwidget/imwidget.h"
#include "nes/mapper.h"
#include "nes/z2objcache.h"
#include "proto/rominfo.pb.h"

namespace z2util {

class ObjectTable: public ImWindowBase {
  public:
    ObjectTable();
    void Init();
    bool Draw() override;

    inline void set_mapper(Mapper* m) { mapper_ = m; }
  private:
    int group_;
    int item_;
    int palette_;
    float scale_;
    int selection_;
    int size_;

    Mapper* mapper_;
    Z2ObjectCache cache_;
    std::vector<const Map*> objtable_;
    std::vector<const char*> names_;
};

}  // namespace z2util

#endif // Z2UTIL_IMWIDGET_OBJECT_TABLE_H
