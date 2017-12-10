#ifndef Z2HD_UTIL_FILE_H
#define Z2HD_UTIL_FILE_H
#include <cstdio>
#include <cstdint>
#include <memory>

#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

#include "util/status.h"
#include "util/statusor.h"

class Stat {
  public:
    enum Mode {
        IFMT = S_IFMT,
#ifndef _WIN32
        IFSOCK = S_IFSOCK,
        IFLNK = S_IFLNK,
        ISUID = S_ISUID,
        ISGID = S_ISGID,
        ISVTX = S_ISVTX,
#endif
        IFREG = S_IFREG,
        IFBLK = S_IFBLK,
        IFDIR = S_IFDIR,
        IFCHR = S_IFCHR,
        IFIFO = S_IFIFO,
        IRWXU = S_IRWXU,
        IRUSR = S_IRUSR,
        IWUSR = S_IWUSR,
        IXUSR = S_IXUSR,
        IRWXG = S_IRWXG,
        IRGRP = S_IRGRP,
        IWGRP = S_IWGRP,
        IXGRP = S_IXGRP,
        IRWXO = S_IRWXO,
        IROTH = S_IROTH,
        IWOTH = S_IWOTH,
        IXOTH = S_IXOTH,
    };
    static StatusOr<Stat> Filename(const std::string& filename);
    static StatusOr<Stat> FileDescriptor(int fd);
    static StatusOr<Stat> Link(const std::string& filename);

    inline bool IsRegular() const {
        return S_ISREG(Mode());
    }
    inline bool IsDirectory() const {
        return S_ISREG(Mode());
    }
    inline bool IsCharDev() const {
        return S_ISREG(Mode());
    }
    inline bool IsBlockDev() const {
        return S_ISREG(Mode());
    }
    inline bool IsFifo() const {
        return S_ISREG(Mode());
    }
    inline bool IsSymlink() const {
        return S_ISREG(Mode());
    }
    inline bool IsSocket() const {
        return S_ISREG(Mode());
    }

    inline int64_t Size() const {
        return stat_.st_size;
    }
    inline mode_t Mode() const {
        return stat_.st_mode;
    }

  private:
    struct stat stat_;
};

class File {
  public:
    static std::unique_ptr<File> Open(const std::string& filename,
                                      const std::string& mode);
    static bool GetContents(const std::string& filename, std::string* contents);
    static bool SetContents(const std::string& filename, const std::string& contents);

    static std::string Basename(const std::string& path);
    static std::string Dirname(const std::string& path);

    virtual ~File();
    Stat FStat();
    int64_t Length();

    bool Read(std::string* buf);
    bool Read(std::string* buf, int64_t len);
    bool Read(void* buf, int64_t *len);
    bool Write(const std::string& buf);
    bool Write(const std::string& buf, int64_t len);
    bool Write(const void* buf, int64_t len);

    void Close();
  private:
    File(FILE* fp);

    FILE* fp_;
};

#endif // Z2HD_UTIL_FILE_H
