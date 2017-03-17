#ifndef Z2UTIL_NES_TEXT_ENCODING_H
#define Z2UTIL_NES_TEXT_ENCODING_H

class TextEncoding {
  public:
    static int Identity(int ch);
    static int FromZelda2(int ch);
    static int ToZelda2(int ch);
  private:
    static unsigned char zelda2_to_ascii_[256];
    static unsigned char ascii_to_zelda2_[256];
};

#endif // Z2UTIL_NES_TEXT_ENCODING_H
