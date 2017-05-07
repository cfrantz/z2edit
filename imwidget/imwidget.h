#ifndef Z2UTIL_IMWIDGET_IMWIDGET_H
#define Z2UTIL_IMWIDGET_IMWIDGET_H


inline int UniqueID() {
    static int id;
    return ++id;
}

class ImWindowBase {
  public:
    explicit ImWindowBase()
      : ImWindowBase(true) {}
    explicit ImWindowBase(bool visible)
      : ImWindowBase(visible, true) {}
    explicit ImWindowBase(bool visible, bool want_dispose)
      : id_(UniqueID()), visible_(visible), want_dispose_(want_dispose) {}
    virtual ~ImWindowBase() {}

    virtual bool Draw() { return false; }
    virtual void Refresh() {}
    inline bool& visible() { return visible_; }
    inline void set_visible(bool v) { visible_ = v; }
    inline bool want_dispose() { return want_dispose_; }
  protected:
    int id_;
    bool visible_;
    bool want_dispose_;
};

#endif // Z2UTIL_IMWIDGET_IMWIDGET_H
