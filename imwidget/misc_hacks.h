#ifndef Z2UTIL_IMWIDGET_MISC_HACKS_H
#define Z2UTIL_IMWIDGET_MISC_HACKS_H

class Mapper;
namespace z2util {

class MiscellaneousHacks {
  public:
    MiscellaneousHacks(): visible_(false) {};

    void Draw();
    inline void set_mapper(Mapper* m) { mapper_ = m; };
    inline bool* visible() { return &visible_; };
  private:
    bool visible_;
    Mapper* mapper_;
};

}  // namespace
#endif // Z2UTIL_IMWIDGET_MISC_HACKS_H
