package(default_visibility=["//visibility:public"])

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
    name = "simplemap",
    hdrs = ["simplemap.h"],
    srcs = ["simplemap.cc"],
    deps = [
        ":glbitmap",
        ":hwpalette",
        ":map_command",
        "//nes:mappers",
        "//nes:z2decompress",
        "//proto:rominfo",
        "//external:imgui",
    ],
)

cc_library(
    name = "map_command",
    hdrs = ["map_command.h"],
    srcs = ["map_command.cc"],
    deps = [
        "//util:config",
        "//nes:mappers",
        "//proto:rominfo",
        "//external:imgui",
    ],
    
)
