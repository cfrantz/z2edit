#include <errno.h>
#include <map>

#include "util/status.h"
#include "util/string.h"

namespace util {
namespace error {

std::map<int, Code> posix_errors = {
    { E2BIG, Code::OUT_OF_RANGE },
    { EACCES, Code::PERMISSION_DENIED },
    { EADDRINUSE, Code::ALREADY_EXISTS },
    { EADDRNOTAVAIL, Code::UNAVAILABLE },
    { EAFNOSUPPORT, Code::UNIMPLEMENTED },
    { EAGAIN, Code::UNAVAILABLE },
    { EALREADY, Code::ALREADY_EXISTS },
    { EBADF, Code::INVALID_ARGUMENT },
    { EBUSY, Code::UNAVAILABLE },
    { ECANCELED, Code::CANCELLED },
    { ECHILD, Code::NOT_FOUND },
    { ECONNABORTED, Code::ABORTED },
    { ECONNREFUSED, Code::UNKNOWN },
    { ECONNRESET, Code::UNKNOWN },
    { EDEADLK, Code::UNKNOWN },
    { EDEADLOCK, Code::UNKNOWN },
    { EDESTADDRREQ, Code::UNKNOWN },
    { EDOM, Code::UNKNOWN },
    { EEXIST, Code::UNKNOWN },
    { EFAULT, Code::UNKNOWN },
    { EFBIG, Code::UNKNOWN },
    { EHOSTUNREACH, Code::UNKNOWN },
    { EILSEQ, Code::UNKNOWN },
    { EINPROGRESS, Code::UNKNOWN },
    { EINTR, Code::UNKNOWN },
    { EINVAL, Code::UNKNOWN },
    { EIO, Code::UNKNOWN },
    { EISCONN, Code::UNKNOWN },
    { EISDIR, Code::UNKNOWN },
    { ELOOP, Code::UNKNOWN },
    { EMFILE, Code::UNKNOWN },
    { EMLINK, Code::UNKNOWN },
    { EMSGSIZE, Code::UNKNOWN },
    { ENAMETOOLONG, Code::UNKNOWN },
    { ENETDOWN, Code::UNKNOWN },
    { ENETRESET, Code::UNKNOWN },
    { ENETUNREACH, Code::UNKNOWN },
    { ENFILE, Code::UNKNOWN },
    { ENOBUFS, Code::UNKNOWN },
    { ENODEV, Code::UNKNOWN },
    { ENOENT, Code::UNKNOWN },
    { ENOEXEC, Code::UNKNOWN },
    { ENOLCK, Code::UNKNOWN },
    { ENOMEM, Code::UNKNOWN },
    { ENOPROTOOPT, Code::UNKNOWN },
    { ENOSPC, Code::UNKNOWN },
    { ENOSYS, Code::UNKNOWN },
    { ENOTCONN, Code::UNKNOWN },
    { ENOTDIR, Code::UNKNOWN },
    { ENOTEMPTY, Code::UNKNOWN },
    { ENOTSOCK, Code::UNKNOWN },
    { ENOTSUP, Code::UNKNOWN },
    { ENOTTY, Code::UNKNOWN },
    { ENXIO, Code::UNKNOWN },
    { EOPNOTSUPP, Code::UNKNOWN },
    { EOVERFLOW, Code::UNKNOWN },
    { EPERM, Code::UNKNOWN },
    { EPIPE, Code::UNKNOWN },
    { EPROTO, Code::UNKNOWN },
    { EPROTONOSUPPORT, Code::UNKNOWN },
    { EPROTOTYPE, Code::UNKNOWN },
    { ERANGE, Code::UNKNOWN },
    { EROFS, Code::UNKNOWN },
    { ESPIPE, Code::UNKNOWN },
    { ESRCH, Code::UNKNOWN },
    { ETIMEDOUT, Code::UNKNOWN },
    { EWOULDBLOCK, Code::UNKNOWN },
    { EXDEV, Code::UNKNOWN },
#ifndef _WIN32
    { EBADE, Code::INVALID_ARGUMENT },
    { EBADFD, Code::INVALID_ARGUMENT },
    { EBADMSG, Code::INVALID_ARGUMENT },
    { EBADR, Code::INVALID_ARGUMENT },
    { EBADRQC, Code::INVALID_ARGUMENT },
    { EBADSLT, Code::INVALID_ARGUMENT },
    { ECHRNG, Code::OUT_OF_RANGE },
    { ECOMM, Code::INTERNAL },
    { EDQUOT, Code::UNKNOWN },
    { EHOSTDOWN, Code::UNKNOWN },
    { EIDRM, Code::UNKNOWN },
    { EISNAM, Code::UNKNOWN },
    { EKEYEXPIRED, Code::UNKNOWN },
    { EKEYREJECTED, Code::UNKNOWN },
    { EKEYREVOKED, Code::UNKNOWN },
    { EL2HLT, Code::UNKNOWN },
    { EL2NSYNC, Code::UNKNOWN },
    { EL3HLT, Code::UNKNOWN },
    { EL3RST, Code::UNKNOWN },
    { ELIBACC, Code::UNKNOWN },
    { ELIBBAD, Code::UNKNOWN },
    { ELIBMAX, Code::UNKNOWN },
    { ELIBSCN, Code::UNKNOWN },
    { ELIBEXEC, Code::UNKNOWN },
    { EMEDIUMTYPE, Code::UNKNOWN },
    { EMULTIHOP, Code::UNKNOWN },
    { ENODATA, Code::UNKNOWN },
    { ENOKEY, Code::UNKNOWN },
    { ENOLINK, Code::UNKNOWN },
    { ENOMEDIUM, Code::UNKNOWN },
    { ENOMSG, Code::UNKNOWN },
    { ENONET, Code::UNKNOWN },
    { ENOPKG, Code::UNKNOWN },
    { ENOSR, Code::UNKNOWN },
    { ENOSTR, Code::UNKNOWN },
    { ENOTBLK, Code::UNKNOWN },
    { ENOTUNIQ, Code::UNKNOWN },
    { EPFNOSUPPORT, Code::UNKNOWN },
    { EREMCHG, Code::UNKNOWN },
    { EREMOTE, Code::UNKNOWN },
    { EREMOTEIO, Code::UNKNOWN },
    { ERESTART, Code::UNKNOWN },
    { ESHUTDOWN, Code::UNKNOWN },
    { ESOCKTNOSUPPORT, Code::UNKNOWN },
    { ESTALE, Code::UNKNOWN },
    { ESTRPIPE, Code::UNKNOWN },
    { ETIME, Code::UNKNOWN },
    { ETXTBSY, Code::UNKNOWN },
    { EUCLEAN, Code::UNKNOWN },
    { EUNATCH, Code::UNKNOWN },
    { EUSERS, Code::UNKNOWN },
    { EXFULL, Code::UNKNOWN },
#endif
};
} // namespace error

string StrError(int error) {
#ifdef _WIN32
    return string(strerror(error));
#else
    char buf[1024];
    strerror_r(error, buf, sizeof(buf));
    return string(buf);
#endif
}

Status PosixStatus(int error) {
    return Status(error::posix_errors[error], StrError(error));
}

} // namespace util
