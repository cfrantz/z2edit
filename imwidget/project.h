#ifndef Z2UTIL_IMWIDGET_PROJECT_H
#define Z2UTIL_IMWIDGET_PROJECT_H

#include "imwidget/imwidget.h"
#include "proto/project.pb.h"

class Cartridge;
namespace z2util {

class Project: public ImWindowBase {
  public:
    Project()
      : ImWindowBase(false), changed_(false), selection_(0) {}
    void Init();
    bool Draw() override;

    bool Load(const std::string& filename, bool with_warnings=true); 
    bool Save(const std::string& filename, bool as_text=false);
    bool ImportRom(const std::string& filename);
    bool ExportRom(const std::string& filename);
    void Commit(const std::string& message);

    inline void set_cartridge(Cartridge* c) { cartridge_ = c; }
    inline const std::string& name() { return project_.name(); }
  private:
    bool LoadWorker(const std::string& filename);
    Cartridge* cartridge_;
    bool changed_;
    int selection_;
    ProjectFile project_;
};

}  // z2util

#endif // Z2UTIL_IMWIDGET_PROJECT_H
