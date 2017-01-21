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
    name = "error_dialog",
    hdrs = ["error_dialog.h"],
    srcs = ["error_dialog.cc"],
    deps = [
        "//:imapp-util",
        "//util:string",
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
        ":error_dialog",
        ":glbitmap",
        ":hwpalette",
        ":map_connect",
        ":overworld_encounters",
        "//nes:mappers",
        "//nes:z2decompress",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//util:config",
        "//util:imgui_sdl_opengl",
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
        "//:imapp-util",
        "//nes:mappers",
        "//nes:z2decompress",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//util:config",
        "//external:imgui",
    ],
)

cc_library(
    name = "multimap",
    hdrs = ["multimap.h"],
    srcs = ["multimap.cc"],
    deps = [
        ":glbitmap",
        ":simplemap",
        ":map_command",
        "//:imapp-util",
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
        ":error_dialog",
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
        ":multimap",
        "//nes:mappers",
        "//proto:rominfo",
        "//external:imgui",
    ],
)

cc_library(
    name = "misc_hacks",
    hdrs = ["misc_hacks.h"],
    srcs = ["misc_hacks.cc"],
    deps = [
        "//nes:mappers",
        "//external:imgui",
    ],
)

cc_library(
    name = "overworld_encounters",
    hdrs = ["overworld_encounters.h"],
    srcs = ["overworld_encounters.cc"],
    deps = [
        "//nes:mappers",
        "//util:config",
        "//external:imgui",
    ],
)

cc_library(
    name = "start_values",
    hdrs = ["start_values.h"],
    srcs = ["start_values.cc"],
    deps = [
        "//nes:mappers",
        "//util:config",
        "//external:imgui",
    ],
)

cc_library(
    name = "object_table",
    hdrs = ["object_table.h"],
    srcs = ["object_table.cc"],
    deps = [
        ":hwpalette",
        ":imutil",
        "//nes:mappers",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//external:imgui",
    ],
)
