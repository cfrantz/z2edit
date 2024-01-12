#ifndef EMPTY_PROJECT_UTIL_STATUS_H
#define EMPTY_PROJECT_UTIL_STATUS_H

#include "util/macros.h"
#include "absl/log/log.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"

#define RETURN_IF_ERROR(expr) do { \
        if (absl::Status _=(expr); !_.ok()) { \
            return _; \
        } \
    } while(0)

#define __ASSIGN_OR_RETURN(tmp, decl, expr) \
    auto tmp = (expr); \
    if (!tmp.ok()) return tmp.status(); \
    decl = std::move(*tmp)

#define ASSIGN_OR_RETURN(decl, expr) \
    __ASSIGN_OR_RETURN(PASTE(expr__, __LINE__), decl, expr)

/**
 * An absl::StatusOr<T>-like object that holds a reference to T.
 */
template <typename T>
class StatusOrRef {
  public:
    StatusOrRef(T* ref) : ref_(ref), const_(false), status_(absl::OkStatus()) {}
    StatusOrRef(const T* ref) : ref_(const_cast<T*>(ref)), const_(true),  status_(absl::OkStatus()) {}
    StatusOrRef(absl::Status status) : ref_(nullptr), const_(false), status_(status) {}
    bool ok() const { return status_.ok(); }
    const absl::Status& status() const { return status_; }

    T& operator*() {
        if (const_) LOG(FATAL) << "Non-const access to const data";
        return *ref_;
    }
    T* operator->() {
        if (const_) LOG(FATAL) << "Non-const access to const data";
        return ref_;
    }
    const T& operator*() const { return *ref_; }
    const T* operator->() const { return ref_; }

  private:
    T* ref_;
    bool const_;
    absl::Status status_;

};

#endif // EMPTY_PROJECT_UTIL_STATUS_H
