#include "imwidget/glbitmap.h"
#include "imgui.h"
#include <SDL2/SDL.h>

GLBitmap::GLBitmap()
  : width_(0),
    height_(0),
    texture_id_(0) {}

GLBitmap::GLBitmap(int w, int h, uint32_t* data)
  : width_(w),
    height_(h),
    texture_id_(0)
{
    Allocate(data);
}

GLBitmap::GLBitmap(GLBitmap&& other)
  : width_(other.width_),
    height_(other.height_),
    texture_id_(other.texture_id_),
    data_(other.data_),
    owned_data_(other.owned_data_.release())
{
    other.texture_id_ = 0;
}

GLBitmap::~GLBitmap() {
    if (texture_id_)
        glDeleteTextures(1, &texture_id_);
}

uint32_t* GLBitmap::Allocate(uint32_t* data, bool claim_ownership) {
    data_ = data ? data : new uint32_t[width_ * height_]();
    owned_data_.reset(claim_ownership ? data_ : nullptr);

    if (texture_id_)
        glDeleteTextures(1, &texture_id_);

    glEnable(GL_TEXTURE_2D);
    glGenTextures(1, &texture_id_);
    glBindTexture(GL_TEXTURE_2D, texture_id_);
    glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
    glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
    glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
    glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA,
                 width_, height_, 0, GL_RGBA,
                 GL_UNSIGNED_BYTE, (void*)data_);
    glBindTexture(GL_TEXTURE_2D, 0);
    return data_;
}

void GLBitmap::Update() {
    glBindTexture(GL_TEXTURE_2D, texture_id_);
    glTexSubImage2D(GL_TEXTURE_2D, 0,
                    0, 0, width_, height_,
                    GL_RGBA, GL_UNSIGNED_BYTE, (void*)data_);
    glBindTexture(GL_TEXTURE_2D, 0);
}

void GLBitmap::Draw(int w, int h) {
    if (w == 0) w = width_;
    if (h == 0) h = height_;
    ImGui::Image(ImTextureID(uintptr_t(texture_id_)), ImVec2(w, h));
}

void GLBitmap::DrawAt(int x, int y, int w, int h) {
    ImGui::SetCursorPos(ImVec2(x, y));
    Draw(w, h);
}

void GLBitmap::DrawAt(int x, int y, float scale) {
    ImGui::SetCursorPos(ImVec2(x, y));
    Draw(int(width_*scale), int(height_*scale));
}

void GLBitmap::Box(int x, int y, int w, int h, uint32_t color) {
    int xx, yy;
    int y0, y1;

    if (x >= width_ || y >= height_) return;
    if (x+w >= width_) w = width_ - x - 1;
    if (y+h >= height_) h = height_ - y - 1;
    color |= 0xFF000000;

    y0 = y * width_;
    y1 = (y+h-1) * width_;
    for(xx=0; xx<w; xx++) {
        data_[y0 + x + xx] = color;
        data_[y1 + x + xx] = color;
    }
    h = h * width_;
    y = y * width_;
    for(yy=0; yy<h; yy+=width_) {
        data_[yy + y + x] = color;
        data_[yy + y + x + w - 1] = color;
    }
}

void GLBitmap::FilledBox(int x, int y, int w, int h, uint32_t color) {
    if (x >= width_ || y >= height_) return;
    if (x+w > width_) w = width_ - x;
    if (y+h > height_) h = height_ - y;
    color |= 0xFF000000;

    int ww = width_ - w;
    uint32_t *p = data_ + x + y * width_;
    for(int yy=0; yy<h; yy++, p+=ww) {
        for(int xx=0; xx<w; xx++, p++) {
            *p = color;
        }
    }
}

void GLBitmap::Blit(int x, int y, int w, int h, uint32_t* pixels) {
    for(int yy=0; yy<h; yy++) {
        int dy = y + yy;
        for(int xx=0; xx<w && dy>=0 && dy<height_; xx++) {
            int dx = x + xx;
            uint32_t val = pixels[yy * w +xx];
            // TODO(cfrantz): real alpha blending
            if (val >> 24) {
                if (dx >= 0 && dx < width_) {
                    data_[dy * width_ + dx] = val;
                }
            }
        }
    }
}

bool GLBitmap::Save(const std::string& filename) {
    SDL_Surface *surface = SDL_CreateRGBSurface(0, width_, height_, 32,
                                                0x000000FF,
                                                0x0000FF00,
                                                0x00FF0000,
                                                0xFF000000);

    uint8_t *src = (uint8_t*)data_;
    uint8_t *dst = (uint8_t*)surface->pixels;
    for(int y=0; y<height_; y++) {
        memcpy(dst, src, width_ * 4);
        dst += surface->pitch;
        src += width_ * 4;
    }

    bool retval = (SDL_SaveBMP(surface, filename.c_str()) == 0);
    SDL_FreeSurface(surface);
    return retval;
}

bool GLBitmap::Load(const std::string& filename) {
    bool retval = false;
    SDL_Surface *orig = nullptr, *surface = nullptr;
    uint8_t *dst = nullptr, *src = nullptr;
    orig = SDL_LoadBMP(filename.c_str());
    if (!orig) goto exitproc;
    surface = SDL_ConvertSurfaceFormat(orig, SDL_PIXELFORMAT_ABGR8888, 0);
    if (!surface) goto exitproc;

    width_ = surface->w;
    height_ = surface->h;
    src = (uint8_t*)surface->pixels;
    dst = (uint8_t*)Allocate();
    for(int y=0; y<height_; y++) {
        memcpy(dst, src, width_ * 4);
        dst += width_ * 4;
        src += surface->pitch;
    }
    Update();
    retval = true;
exitproc:
    SDL_FreeSurface(surface);
    SDL_FreeSurface(orig);
    return retval;
}
