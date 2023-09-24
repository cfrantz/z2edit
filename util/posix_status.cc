#include <map>
#include <string>
#include <errno.h>

#include "util/posix_status.h"

namespace util {
namespace error {

std::map<int, absl::StatusCode> posix_errors = {
    { E2BIG, absl::StatusCode::kOutOfRange },
    { EACCES, absl::StatusCode::kPermissionDenied },
    { EADDRINUSE, absl::StatusCode::kAlreadyExists },
    { EADDRNOTAVAIL, absl::StatusCode::kUnavailable },
    { EAFNOSUPPORT, absl::StatusCode::kUnimplemented },
    { EAGAIN, absl::StatusCode::kUnavailable },
    { EALREADY, absl::StatusCode::kAlreadyExists },
    { EBADF, absl::StatusCode::kInvalidArgument },
    { EBUSY, absl::StatusCode::kUnavailable },
    { ECANCELED, absl::StatusCode::kCancelled },
    { ECHILD, absl::StatusCode::kNotFound },
    { ECONNABORTED, absl::StatusCode::kAborted },
    { ECONNREFUSED, absl::StatusCode::kUnknown },
    { ECONNRESET, absl::StatusCode::kUnknown },
    { EDEADLK, absl::StatusCode::kUnknown },
    { EDEADLOCK, absl::StatusCode::kUnknown },
    { EDESTADDRREQ, absl::StatusCode::kUnknown },
    { EDOM, absl::StatusCode::kUnknown },
    { EEXIST, absl::StatusCode::kUnknown },
    { EFAULT, absl::StatusCode::kUnknown },
    { EFBIG, absl::StatusCode::kUnknown },
    { EHOSTUNREACH, absl::StatusCode::kUnknown },
    { EILSEQ, absl::StatusCode::kUnknown },
    { EINPROGRESS, absl::StatusCode::kUnknown },
    { EINTR, absl::StatusCode::kUnknown },
    { EINVAL, absl::StatusCode::kUnknown },
    { EIO, absl::StatusCode::kUnknown },
    { EISCONN, absl::StatusCode::kUnknown },
    { EISDIR, absl::StatusCode::kUnknown },
    { ELOOP, absl::StatusCode::kUnknown },
    { EMFILE, absl::StatusCode::kUnknown },
    { EMLINK, absl::StatusCode::kUnknown },
    { EMSGSIZE, absl::StatusCode::kUnknown },
    { ENAMETOOLONG, absl::StatusCode::kUnknown },
    { ENETDOWN, absl::StatusCode::kUnknown },
    { ENETRESET, absl::StatusCode::kUnknown },
    { ENETUNREACH, absl::StatusCode::kUnknown },
    { ENFILE, absl::StatusCode::kUnknown },
    { ENOBUFS, absl::StatusCode::kUnknown },
    { ENODEV, absl::StatusCode::kUnknown },
    { ENOENT, absl::StatusCode::kUnknown },
    { ENOEXEC, absl::StatusCode::kUnknown },
    { ENOLCK, absl::StatusCode::kUnknown },
    { ENOMEM, absl::StatusCode::kUnknown },
    { ENOPROTOOPT, absl::StatusCode::kUnknown },
    { ENOSPC, absl::StatusCode::kUnknown },
    { ENOSYS, absl::StatusCode::kUnknown },
    { ENOTCONN, absl::StatusCode::kUnknown },
    { ENOTDIR, absl::StatusCode::kUnknown },
    { ENOTEMPTY, absl::StatusCode::kUnknown },
    { ENOTSOCK, absl::StatusCode::kUnknown },
    { ENOTSUP, absl::StatusCode::kUnknown },
    { ENOTTY, absl::StatusCode::kUnknown },
    { ENXIO, absl::StatusCode::kUnknown },
    { EOPNOTSUPP, absl::StatusCode::kUnknown },
    { EOVERFLOW, absl::StatusCode::kUnknown },
    { EPERM, absl::StatusCode::kUnknown },
    { EPIPE, absl::StatusCode::kUnknown },
    { EPROTO, absl::StatusCode::kUnknown },
    { EPROTONOSUPPORT, absl::StatusCode::kUnknown },
    { EPROTOTYPE, absl::StatusCode::kUnknown },
    { ERANGE, absl::StatusCode::kUnknown },
    { EROFS, absl::StatusCode::kUnknown },
    { ESPIPE, absl::StatusCode::kUnknown },
    { ESRCH, absl::StatusCode::kUnknown },
    { ETIMEDOUT, absl::StatusCode::kUnknown },
    { EWOULDBLOCK, absl::StatusCode::kUnknown },
    { EXDEV, absl::StatusCode::kUnknown },
#ifndef _WIN32
    { EBADE, absl::StatusCode::kInvalidArgument },
    { EBADFD, absl::StatusCode::kInvalidArgument },
    { EBADMSG, absl::StatusCode::kInvalidArgument },
    { EBADR, absl::StatusCode::kInvalidArgument },
    { EBADRQC, absl::StatusCode::kInvalidArgument },
    { EBADSLT, absl::StatusCode::kInvalidArgument },
    { ECHRNG, absl::StatusCode::kOutOfRange },
    { ECOMM, absl::StatusCode::kInternal },
    { EDQUOT, absl::StatusCode::kUnknown },
    { EHOSTDOWN, absl::StatusCode::kUnknown },
    { EIDRM, absl::StatusCode::kUnknown },
    { EISNAM, absl::StatusCode::kUnknown },
    { EKEYEXPIRED, absl::StatusCode::kUnknown },
    { EKEYREJECTED, absl::StatusCode::kUnknown },
    { EKEYREVOKED, absl::StatusCode::kUnknown },
    { EL2HLT, absl::StatusCode::kUnknown },
    { EL2NSYNC, absl::StatusCode::kUnknown },
    { EL3HLT, absl::StatusCode::kUnknown },
    { EL3RST, absl::StatusCode::kUnknown },
    { ELIBACC, absl::StatusCode::kUnknown },
    { ELIBBAD, absl::StatusCode::kUnknown },
    { ELIBMAX, absl::StatusCode::kUnknown },
    { ELIBSCN, absl::StatusCode::kUnknown },
    { ELIBEXEC, absl::StatusCode::kUnknown },
    { EMEDIUMTYPE, absl::StatusCode::kUnknown },
    { EMULTIHOP, absl::StatusCode::kUnknown },
    { ENODATA, absl::StatusCode::kUnknown },
    { ENOKEY, absl::StatusCode::kUnknown },
    { ENOLINK, absl::StatusCode::kUnknown },
    { ENOMEDIUM, absl::StatusCode::kUnknown },
    { ENOMSG, absl::StatusCode::kUnknown },
    { ENONET, absl::StatusCode::kUnknown },
    { ENOPKG, absl::StatusCode::kUnknown },
    { ENOSR, absl::StatusCode::kUnknown },
    { ENOSTR, absl::StatusCode::kUnknown },
    { ENOTBLK, absl::StatusCode::kUnknown },
    { ENOTUNIQ, absl::StatusCode::kUnknown },
    { EPFNOSUPPORT, absl::StatusCode::kUnknown },
    { EREMCHG, absl::StatusCode::kUnknown },
    { EREMOTE, absl::StatusCode::kUnknown },
    { EREMOTEIO, absl::StatusCode::kUnknown },
    { ERESTART, absl::StatusCode::kUnknown },
    { ESHUTDOWN, absl::StatusCode::kUnknown },
    { ESOCKTNOSUPPORT, absl::StatusCode::kUnknown },
    { ESTALE, absl::StatusCode::kUnknown },
    { ESTRPIPE, absl::StatusCode::kUnknown },
    { ETIME, absl::StatusCode::kUnknown },
    { ETXTBSY, absl::StatusCode::kUnknown },
    { EUCLEAN, absl::StatusCode::kUnknown },
    { EUNATCH, absl::StatusCode::kUnknown },
    { EUSERS, absl::StatusCode::kUnknown },
    { EXFULL, absl::StatusCode::kUnknown },
#endif
};
} // namespace error

std::string StrError(int error) {
#ifdef _WIN32
    return std::string(strerror(error));
#else
    char buf[1024];
    return std::string(strerror_r(error, buf, sizeof(buf)));
#endif
}

absl::Status PosixStatus(int error) {
    return absl::Status(error::posix_errors[error], StrError(error));
}

} // namespace util
