#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <libgen.h>
#include <limits.h>

#include "util/file.h"
#include "util/statusor.h"
#include "util/string.h"

StatusOr<Stat> Stat::Filename(const string& filename) {
    Stat s;
    if (stat(filename.c_str(), &s.stat_) == -1) {
        return util::PosixStatus(errno);
    }
    return s;
}

StatusOr<Stat> Stat::FileDescriptor(int fd) {
    Stat s;
    if (fstat(fd, &s.stat_) == -1) {
        return util::PosixStatus(errno);
    }
    return s;
}

StatusOr<Stat> Stat::Link(const string& filename) {
    Stat s;
    if (stat(filename.c_str(), &s.stat_) == -1) {
        return util::PosixStatus(errno);
    }
    return s;
}

std::unique_ptr<File> File::Open(const string& filename,
                                 const string& mode) {
    FILE* fp = fopen(filename.c_str(), mode.c_str());
    if (fp == nullptr)
        return nullptr;
    return std::unique_ptr<File>(new File(fp));
}

bool File::GetContents(const string& filename, string* contents) {
    std::unique_ptr<File> f(Open(filename, "rb"));
    if (f == nullptr)
        return false;
    return f->Read(contents);
}

bool File::SetContents(const string& filename, const string& contents) {
    std::unique_ptr<File> f(Open(filename, "wb"));
    if (f == nullptr)
        return false;
    return f->Write(contents);
}

string File::Basename(const string& path) {
    char buf[PATH_MAX];
    memcpy(buf, path.data(), path.size());
    buf[path.size()] = '\0';
    return string(basename(buf));
}

string File::Dirname(const string& path) {
    char buf[PATH_MAX];
    memcpy(buf, path.data(), path.size());
    buf[path.size()] = '\0';
    return string(dirname(buf));
}

File::File(FILE* fp) : fp_(fp) {}

File::~File() {
    Close();
}

Stat File::FStat() {
    return Stat::FileDescriptor(fileno(fp_)).ValueOrDie();
}

int64_t File::Length() {
//	long pos = ftell(fp_);
//	fseek(fp_, 0, SEEK_END);
//	long end = ftell(fp_);
//	fseek(fp_, pos, SEEK_SET);
//	return end;
    return FStat().Size();
}

bool File::Read(void* buf, int64_t len) {
    fread(buf, 1, len, fp_);
    return !ferror(fp_);
}

bool File::Read(string* buf, int64_t len) {
    buf->resize(len);
    return Read(&buf->front(), len);
}

bool File::Read(string* buf) {
    return Read(buf, Length());
}

bool File::Write(const void* buf, int64_t len) {
    fwrite(buf, 1, len, fp_);
    return !ferror(fp_);
}

bool File::Write(const string& buf, int64_t len) {
    return Write(buf.data(), len);
}

bool File::Write(const string& buf) {
    return Write(buf.data(), buf.size());
}

void File::Close() {
    if (fp_) {
        fclose(fp_);
        fp_ = nullptr;
    }
}
