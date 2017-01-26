package(default_visibility=["//visibility:public"])

cc_library(
    name = "cartridge",
    hdrs = ["cartridge.h"],
    srcs = ["cartridge.cc"],
    deps = [
        "//imwidget:debug_console",
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
    name = "mappers",
    hdrs = [
        "mapper.h",
    ],
    srcs = [
        "mapper.cc",
        "mapper1.h",
        "mapper1.cc",
#        "mapper2.cc",
#        "mapper3.cc",
#        "mapper4.cc",
    ],
    deps = [
        ":cartridge",
        "//proto:rominfo",
        "//imwidget:debug_console",
    ],
    alwayslink = 1,
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

