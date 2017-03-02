package(default_visibility=["//visibility:public"])

cc_library(
    name = "cartridge",
    hdrs = ["cartridge.h"],
    srcs = ["cartridge.cc"],
    deps = [
        "//imwidget:base",
        "//external:gflags",
    ],
)

cc_library(
    name = "cpu6502",
    hdrs = ["cpu6502.h"],
    srcs = ["cpu6502.cc"],
    deps = [
        ":mappers",
    ],
)

cc_library(
    name = "enemylist",
    hdrs = [
        "enemylist.h",
    ],
    srcs = [
        "enemylist.cc"
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
    hdrs = [
        "mapper.h",
        "memory.h",
    ],
    srcs = [
        "mapper.cc",
        "mapper1.h",
        "mapper1.cc",
        "memory.cc"
    ],
    deps = [
        ":cartridge",
        "//proto:rominfo",
        "//imwidget:base",
        "//util:config",
        "//util:logging",
    ],
    alwayslink=1,
)

cc_library(
    name = "z2decompress",
    hdrs = ["z2decompress.h"],
    srcs = [
        "z2decompress.cc",
        "z2decomp_funcs.cc",
    ],
    deps = [
        ":mappers",
        "//util:config",
        "//util:logging",
        "//proto:rominfo",
        "//external:gflags",
    ],
)

cc_library(
    name = "z2objcache",
    hdrs = ["z2objcache.h"],
    srcs = [
        "z2objcache.cc",
    ],
    deps = [
        ":mappers",
        "//proto:rominfo",
        "//util:config",
        "//util:logging",
        "//imwidget:glbitmap",
        "//imwidget:hwpalette",
    ],
)

