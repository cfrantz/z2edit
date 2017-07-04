#ifndef Z2UTIL_NES_TEXTLIST_H
#define Z2UTIL_NES_TEXTLIST_H
#include <cstdint>
#include <map>
#include <vector>
#include "proto/rominfo.pb.h"

class Mapper;
namespace z2util {

class TextListPack {
  public:
    TextListPack(Mapper* m) : mapper_(m), newtext_(0), bank_(0) {}
    TextListPack() : TextListPack(nullptr) {}

    void Unpack(int bank);
    bool Pack();
    bool Get(int world, int index, std::string* val);
    bool Set(int world, int index, const std::string& val);
    int Length(int world);
    //void Add(int index, const std::vector<uint8_t>& data);
    inline void set_mapper(Mapper* m) { mapper_ = m; }
  private:
    void ReadOne(int world, int index, Address addr);
    std::string ReadNesString(const Address& addr);
    void WriteNesString(const Address& addr, const std::string& val);
    void ResetAddrs();

    Mapper* mapper_;
    int newtext_;
    int bank_;

    struct List {
        uint16_t newaddr;
        std::string data;
    };

    // Index by world, index
    std::vector<std::vector<uint16_t>> index_;
    std::map<uint16_t, List> entry_;
};


}  // namespace z2util

#endif // Z2UTIL_NES_TEXTLIST_H
