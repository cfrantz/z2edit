package(default_visibility=["//visibility:public"])

cc_library(
    name = "ips",
    hdrs = [
        "ips.h",
    ],
    srcs = [
        "ips.cc",
    ],
    deps = [
        "//util:status",
    ],
)

cc_binary(
    name = "ipspatch",
    srcs = ["ipspatch.cc"],
    deps = [
        ":ips",
        "//util:file",
        "//util:status",
        "//external:gflags",
    ],
)
