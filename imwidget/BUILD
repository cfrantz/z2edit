package(default_visibility=["//visibility:public"])

cc_library(
    name = "base",
    hdrs = [
        "imutil.h",
        "imwidget.h",
    ],
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
        ":base",
        "//util:logging",
        "//external:imgui",
    ],
)

cc_library(
    name = "error_dialog",
    hdrs = ["error_dialog.h"],
    srcs = ["error_dialog.cc"],
    deps = [
        ":base",
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
        ":base",
        "//external:imgui",
    ],
)

cc_library(
    name = "neschrview",
    hdrs = ["neschrview.h"],
    srcs = ["neschrview.cc"],
    deps = [
        ":base",
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
        ":base",
        ":error_dialog",
        ":glbitmap",
        ":hwpalette",
        ":map_connect",
        ":overworld_encounters",
        "//:imapp-util",
        "//nes:mappers",
        "//nes:z2decompress",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//util:config",
        "//util:imgui_sdl_opengl",
        "//util:stb-tilemap-editor",
        "//external:gflags",
        "//external:imgui",
    ],
)

cc_library(
    name = "simplemap",
    hdrs = [
        "simplemap.h",
        "map_command.h",
    ],
    srcs = [
        "simplemap.cc",
        "map_command.cc",
    ],
    deps = [
        ":base",
        ":error_dialog",
        ":glbitmap",
        ":hwpalette",
        ":overworld_encounters",
        "//:imapp-util",
        "//nes:enemylist",
        "//nes:mappers",
        "//nes:z2decompress",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//util:config",
        "//external:imgui",
        "//external:fontawesome",
    ],
)

cc_library(
    name = "multimap",
    hdrs = ["multimap.h"],
    srcs = ["multimap.cc"],
    deps = [
        ":base",
        ":glbitmap",
        ":simplemap",
        "//alg:fdg",
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
    name = "map_connect",
    hdrs = ["map_connect.h"],
    srcs = ["map_connect.cc"],
    deps = [
        ":base",
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
        ":base",
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
        ":base",
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
        ":base",
        ":hwpalette",
        "//nes:mappers",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//external:imgui",
    ],
)
