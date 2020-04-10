package(default_visibility = ["//visibility:public"])

cc_library(
    name = "romfile",
    srcs = [
        "romfile.cc",
    ],
    hdrs = [
        "romfile.h",
    ],
    deps = [
        "//external:gflags",
        "//util:file",
        "//util:string",
    ],
)

cc_library(
    name = "overworld",
    srcs = [
        "overworld.cc",
    ],
    hdrs = [
        "overworld.h",
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
        ":overworld",
        ":romfile",
        "//external:gflags",
    ],
)

cc_binary(
    name = "chr_dump",
    srcs = ["chr_dump.cc"],
    deps = ["//:app"],
    linkopts = [
        "-lpthread",
        "-lm",
        "-lGL",
    ],
)
