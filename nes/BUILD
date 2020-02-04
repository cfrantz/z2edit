package(default_visibility = ["//visibility:public"])

cc_library(
    name = "cartridge",
    srcs = ["cartridge.cc"],
    hdrs = ["cartridge.h"],
    deps = [
        "//external:gflags",
        "//imwidget:base",
        "//util:file",
    ],
)

cc_library(
    name = "chr_util",
    srcs = ["chr_util.cc"],
    hdrs = ["chr_util.h"],
    deps = [
        ":mappers",
    ],
)

cc_library(
    name = "cpu6502",
    srcs = ["cpu6502.cc"],
    hdrs = ["cpu6502.h"],
    deps = [
        ":mappers",
        "@com_google_absl//absl/strings",
    ],
)

cc_library(
    name = "enemylist",
    srcs = [
        "enemylist.cc",
    ],
    hdrs = [
        "enemylist.h",
    ],
    deps = [
        ":mappers",
        "//proto:rominfo",
        "//util:config",
        "//util:logging",
    ],
)

cc_library(
    name = "mappers",
    srcs = [
        "mapper.cc",
        "mapper1.cc",
        "mapper1.h",
        "memory.cc",
    ],
    hdrs = [
        "mapper.h",
        "memory.h",
    ],
    deps = [
        ":cartridge",
        "//imwidget:base",
        "//proto:rominfo",
        "//util:config",
        "//util:logging",
    ],
    alwayslink = 1,
)

cc_library(
    name = "text_encoding",
    srcs = ["text_encoding.cc"],
    hdrs = ["text_encoding.h"],
    deps = [
    ],
)

cc_library(
    name = "text_list",
    srcs = ["text_list.cc"],
    hdrs = ["text_list.h"],
    deps = [
        ":mappers",
        ":text_encoding",
        "//proto:rominfo",
        "//util:config",
        "//util:logging",
    ],
)

cc_library(
    name = "z2decompress",
    srcs = [
        "z2decomp_funcs.cc",
        "z2decompress.cc",
    ],
    hdrs = ["z2decompress.h"],
    deps = [
        ":mappers",
        "//external:gflags",
        "//proto:rominfo",
        "//util:config",
        "//util:logging",
    ],
)

cc_library(
    name = "z2objcache",
    srcs = [
        "z2objcache.cc",
    ],
    hdrs = ["z2objcache.h"],
    deps = [
        ":mappers",
        "//imwidget:glbitmap",
        "//imwidget:hwpalette",
        "//proto:rominfo",
        "//util:config",
        "//util:logging",
    ],
)
