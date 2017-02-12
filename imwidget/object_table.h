#ifndef Z2UTIL_IMWIDGET_OBJECT_TABLE_H
#define Z2UTIL_IMWIDGET_OBJECT_TABLE_H

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
    float scale_;
    Mapper* mapper_;

    Address table_;
    Address chr_;
    Z2ObjectCache cache_;
};

}  // namespace z2util

#endif // Z2UTIL_IMWIDGET_OBJECT_TABLE_H
