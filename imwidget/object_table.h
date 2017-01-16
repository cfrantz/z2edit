#ifndef Z2UTIL_IMWIDGET_OBJECT_TABLE_H
#define Z2UTIL_IMWIDGET_OBJECT_TABLE_H

#include "nes/mapper.h"
#include "nes/z2objcache.h"
#include "proto/rominfo.pb.h"

namespace z2util {

class ObjectTable {
  public:
    ObjectTable();
    void Init();
    void Draw();

    inline void set_mapper(Mapper* m) { mapper_ = m; }
    inline bool* visible() { return &visible_; }
  private:
    bool visible_;
    float scale_;
    Mapper* mapper_;

    Address table_;
    Address chr_;
    Z2ObjectCache cache_;
};

}  // namespace z2util

#endif // Z2UTIL_IMWIDGET_OBJECT_TABLE_H
