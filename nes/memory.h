#ifndef Z2UTIL_NES_MEMORY_H
#define Z2UTIL_NES_MEMORY_H

#include "proto/rominfo.pb.h"

class Mapper;

namespace z2util {

class Memory {
  public:
    Memory();

    int CheckForKeepout(Address baseaddr, const std::string& name, int len,
                        bool move=false);
    int CheckBankForKeepout(int bank, bool move=false);
    int MoveMapOutOfKeepout(const Address& pointer);

    int CheckBank3Special(bool move=false);

    void CheckAllBanksForKeepout(bool move=false);

    void Reset() { moved_.clear(); }
    inline void set_mapper(Mapper* m) { mapper_ = m; }
    static bool InKeepoutRegion(const Address& addr);
    static int key(const Address& a) {
        return (a.bank() << 16) | a.address();
    }
  private:
    Mapper* mapper_;
    std::map<int, int> moved_;
};

}  // namespace

#endif // Z2UTIL_NES_MEMORY_H
