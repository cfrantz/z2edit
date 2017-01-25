package(default_visibility=["//visibility:public"])

cc_library(
    name = "romfile",
    hdrs = [
        "romfile.h",
    ],
    srcs = [
        "romfile.cc",
    ],
    deps = [
        "//util:file",
        "//util:string",
        "//external:gflags",
    ],
)

cc_library(
    name = "overworld",
    hdrs = [
        "overworld.h",
    ],
    srcs = [
        "overworld.cc",
    ],
    deps = [
        ":romfile",
        "//util:string",
    ],
)

cc_binary(
    name = "dump",
    srcs = [
        "dump.cc",
    ],
    deps = [
        ":romfile",
        ":overworld",
        "//external:gflags",
    ],
)
