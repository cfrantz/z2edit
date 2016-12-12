package(default_visibility=["//visibility:public"])

cc_library(
    name = "imutil",
    hdrs = ["imutil.h"],
    srcs = ["imutil.cc"],
    deps = [
        "//external:imgui",
    ],
)

cc_library(
    name = "debug_console",
    hdrs = ["debug_console.h"],
    srcs = ["debug_console.cc"],
    deps = [
        "//external:imgui",
    ],
)

cc_library(
    name = "glbitmap",
    hdrs = ["glbitmap.h"],
    srcs = ["glbitmap.cc"],
    deps = [
        "//external:imgui",
    ],
)

cc_library(
    name = "hwpalette",
    hdrs = ["hwpalette.h"],
    srcs = ["hwpalette.cc"],
    deps = [
        "//external:imgui",
    ],
)

cc_library(
    name = "neschrview",
    hdrs = ["neschrview.h"],
    srcs = ["neschrview.cc"],
    deps = [
        ":glbitmap",
        "//nes:mappers",
        "//external:imgui",
    ],
)

cc_library(
    name = "editor",
    copts = [
        "-fpermissive",
        "-Wno-unused-function",
        "-Wno-unused-variable",
        "-Wno-unused-but-set-variable",
        "-Wno-write-strings",
        "-Wno-comment",
        "-Wno-narrowing",
    ],
    hdrs = ["editor.h"],
    srcs = ["editor.cc"],
    deps = [
        ":imutil",
        ":glbitmap",
        ":hwpalette",
        ":map_connect",
        "//nes:mappers",
        "//nes:z2decompress",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//util:config",
        "//util:stb-tilemap-editor",
        "//external:imgui",
    ],
)

cc_library(
    name = "simplemap",
    hdrs = ["simplemap.h"],
    srcs = ["simplemap.cc"],
    deps = [
        ":imutil",
        ":glbitmap",
        ":hwpalette",
        ":map_command",
        "//nes:mappers",
        "//nes:z2decompress",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//util:config",
        "//external:imgui",
    ],
)

cc_library(
    name = "map_command",
    hdrs = ["map_command.h"],
    srcs = ["map_command.cc"],
    deps = [
        ":imutil",
        "//util:config",
        "//nes:mappers",
        "//proto:rominfo",
        "//external:imgui",
    ],
)

cc_library(
    name = "map_connect",
    hdrs = ["map_connect.h"],
    srcs = ["map_connect.cc"],
    deps = [
        ":imutil",
        "//nes:mappers",
        "//proto:rominfo",
        "//external:imgui",
    ],
)
