#ifndef Z2UTIL_IMWIDGET_GLBITMAP_H
#define Z2UTIL_IMWIDGET_GLBITMAP_H
#include <cstdint>
#include <memory>

// FIXME(cfrantz): probably include the real opengl headers
#include <SDL2/SDL_opengl.h>

class GLBitmap {
  public:
    GLBitmap(int w, int h);
    uint32_t* Allocate(uint32_t* data=nullptr);
    void Update();
    void Draw(int w=0, int h=0);

    inline uint32_t* data() { return data_.get(); }
    inline GLuint texture_id() { return texture_id_; }
    inline void SetPixel(int x, int y, uint32_t color) {
        data_[y * width_ + x] = color;
    }
    void Box(int x, int y, int w, int h, uint32_t color);
    void FilledBox(int x, int y, int w, int h, uint32_t color);
    void Blit(int x, int y, int w, int h, uint32_t* pixels);

  private:
    int width_;
    int height_;
    std::unique_ptr<uint32_t[]> data_;
    GLuint texture_id_;
};

#endif // Z2UTIL_IMWIDGET_GLBITMAP_H
