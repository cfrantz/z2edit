#ifndef Z2HD_UTIL_FILE_H
#define Z2HD_UTIL_FILE_H
#include <memory>
#include <stdio.h>
#include <stdint.h>

#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

#include "util/status.h"
#include "util/statusor.h"
#include "util/string.h"

class Stat {
  public:
    enum Mode {
        IFMT = S_IFMT,
        IFSOCK = S_IFSOCK,
        IFLNK = S_IFLNK,
        IFREG = S_IFREG,
        IFBLK = S_IFBLK,
        IFDIR = S_IFDIR,
        IFCHR = S_IFCHR,
        IFIFO = S_IFIFO,
        ISUID = S_ISUID,
        ISGID = S_ISGID,
        ISVTX = S_ISVTX,
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
    static StatusOr<Stat> Filename(const string& filename);
    static StatusOr<Stat> FileDescriptor(int fd);
    static StatusOr<Stat> Link(const string& filename);

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
    static std::unique_ptr<File> Open(const string& filename,
                                      const string& mode);
    static bool GetContents(const string& filename, string* contents);
    static bool SetContents(const string& filename, const string& contents);

    static string Basename(const string& path);
    static string Dirname(const string& path);

    virtual ~File();
    Stat FStat();
    int64_t Length();

    bool Read(string* buf);
    bool Read(string* buf, int64_t len);
    bool Read(void* buf, int64_t len);
    bool Write(const string& buf);
    bool Write(const string& buf, int64_t len);
    bool Write(const void* buf, int64_t len);

    void Close();
  private:
    File(FILE* fp);

    FILE* fp_;
};

#endif // Z2HD_UTIL_FILE_H
