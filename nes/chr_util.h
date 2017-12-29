#ifndef Z2UTIL_NES_CHR_UTIL_H
#define Z2UTIL_NES_CHR_UTIL_H
#include <cstdint>

class Mapper;
namespace z2util {

class ChrUtil {
  public:
    ChrUtil() : ChrUtil(nullptr) {}
    ChrUtil(Mapper* m) : mapper_(m) {}

    void Clear(int bank, uint8_t ch, bool with_id=false);
    void Copy(int dbank, uint8_t dst, int sbank, uint8_t src);
    void Swap(int dbank, uint8_t dst, int sbank, uint8_t src);
    inline void set_mapper(Mapper* m) { mapper_ = m; }

  private:
    Mapper* mapper_;
};


}  // namespace
#endif // Z2UTIL_NES_CHR_UTIL_H
