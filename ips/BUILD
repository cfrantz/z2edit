package(default_visibility = ["//visibility:public"])

cc_library(
    name = "ips",
    srcs = [
        "ips.cc",
    ],
    hdrs = [
        "ips.h",
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
        "//external:gflags",
        "//util:file",
        "//util:status",
    ],
)
