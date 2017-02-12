#ifndef Z2UTIL_IMWIDGET_MAP_CONNECT_H
#define Z2UTIL_IMWIDGET_MAP_CONNECT_H
#include <cstdint>
#include <vector>

#include "proto/rominfo.pb.h"
#include "nes/mapper.h"

namespace z2util {

class OverworldConnectorList;
class OverworldConnector {
  public:
    OverworldConnector(Mapper* mapper, Address address,
                       uint8_t offset, int overworld, int subworld);
    OverworldConnector(const OverworldConnector& other);
    bool DrawInPopup();

    inline int offset() const { return offset_; };
    inline int xpos() const { return x_; };
    inline int ypos() const { return y_; };
  private:
    void Read();
    void Write();
    const char* label(const char *name);

    Mapper* mapper_;
    Address address_;

    int offset_;
    int overworld_;
    int subworld_;

    int x_;
    int y_;
    int map_;
    int dest_world_;
    int dest_overworld_;

    bool ext_;
    bool second_;
    bool exit_2_lower_;
    int entry_;
    bool entry_right_;
    bool passthru_;
    bool fall_;

    int label_nr_;
    char labels_[11][32];
    friend OverworldConnectorList;
};

class OverworldConnectorList {
  public:
    OverworldConnectorList();

    void Init(Mapper* mapper, Address address, int overworld, int subworld,
              int n=63);

    bool Draw();
    bool DrawInEditor(int x, int y);

    OverworldConnector* GetAtXY(int x, int y);
    OverworldConnector* Swap(int a, int b);

    void GetXY(int n, int* x, int *y) {
        const auto& item = list_[n];
        *x = item.xpos();
        *y = item.ypos();
    }
    void Save();

    inline bool show() const { return show_; }
    inline bool changed() const { return changed_; }

  private:
    Mapper* mapper_;
    int add_offset_;
    std::vector<OverworldConnector> list_;
    bool show_;
    bool changed_;
};

}  // namespace z2util
#endif // Z2UTIL_IMWIDGET_MAP_CONNECT_H
