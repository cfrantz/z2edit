#ifndef Z2UTIL_NES_ENEMYLIST_H
#define Z2UTIL_NES_ENEMYLIST_H
#include <cstdint>
#include <map>
#include <vector>
#include "proto/rominfo.pb.h"

class Mapper;
namespace z2util {

class EnemyListPack {
  public:
    EnemyListPack(Mapper* m) : mapper_(m), newareas_(0), bank_(0) {}
    EnemyListPack() : EnemyListPack(nullptr) {}

    void Unpack(int bank);
    void Add(int area, const std::vector<uint8_t>& data);
    bool Pack();
    inline void set_mapper(Mapper* m) { mapper_ = m; }
  private:
    void LoadEncounters();
    bool IsEncounter(int area);
    void ReadOne(int area, Address addr);

    Mapper* mapper_;
    int newareas_;
    int bank_;

    struct List {
        uint16_t newaddr;
        std::vector<uint8_t> data;
    };
    std::vector<uint16_t> area_;
    std::map<uint16_t, List> entry_;
    std::vector<uint8_t> encounters_;
};


}  // namespace z2util

#endif // Z2UTIL_NES_ENEMYLIST_H
