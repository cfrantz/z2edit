package(default_visibility = ["//visibility:public"])

cc_library(
    name = "base",
    srcs = [
        "debug_console.cc",
        "imapp.cc",
        "imutil.cc",
    ],
    hdrs = [
        "debug_console.h",
        "imapp.h",
        "imutil.h",
        "imwidget.h",
    ],
    deps = [
        "//external:gflags",
        "//external:imgui",
        "//util:fpsmgr",
        "//util:gamecontrollerdb",
        "//util:imgui_sdl_opengl",
        "//util:logging",
        "//util:os",
    ],
)

cc_library(
    name = "error_dialog",
    srcs = ["error_dialog.cc"],
    hdrs = ["error_dialog.h"],
    deps = [
        ":base",
        "//external:imgui",
        "@com_google_absl//absl/strings",
    ],
)

cc_library(
    name = "glbitmap",
    srcs = ["glbitmap.cc"],
    hdrs = ["glbitmap.h"],
    deps = [
        "//external:imgui",
    ],
)

cc_library(
    name = "hwpalette",
    srcs = ["hwpalette.cc"],
    hdrs = ["hwpalette.h"],
    deps = [
        ":base",
        "//external:imgui",
    ],
)

cc_library(
    name = "neschrview",
    srcs = ["neschrview.cc"],
    hdrs = ["neschrview.h"],
    deps = [
        ":base",
        ":error_dialog",
        ":glbitmap",
        "//external:imgui",
        "//external:nfd",
        "//nes:mappers",
    ],
)

cc_library(
    name = "editor",
    srcs = ["editor.cc"],
    hdrs = ["editor.h"],
    copts = [
        "-fpermissive",
        "-Wno-unused-function",
        "-Wno-unused-variable",
        "-Wno-unused-but-set-variable",
        "-Wno-write-strings",
        "-Wno-comment",
        "-Wno-narrowing",
    ],
    deps = [
        ":base",
        ":error_dialog",
        ":glbitmap",
        ":hwpalette",
        ":map_connect",
        ":overworld_encounters",
        ":randomize",
        "//external:gflags",
        "//external:imgui",
        "//nes:mappers",
        "//nes:z2decompress",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//util:config",
        "//util:imgui_sdl_opengl",
        "//util:stb-tilemap-editor",
    ],
)

cc_library(
    name = "palette",
    srcs = ["palette.cc"],
    hdrs = ["palette.h"],
    deps = [
        ":base",
        ":error_dialog",
        ":hwpalette",
        "//external:gflags",
        "//external:imgui",
        "//nes:mappers",
        "//proto:rominfo",
        "//util:config",
    ],
)

cc_library(
    name = "simplemap",
    srcs = [
        "map_command.cc",
        "simplemap.cc",
    ],
    hdrs = [
        "map_command.h",
        "simplemap.h",
    ],
    deps = [
        ":base",
        ":error_dialog",
        ":glbitmap",
        ":hwpalette",
        ":overworld_encounters",
        "//external:fontawesome",
        "//external:imgui",
        "//nes:enemylist",
        "//nes:mappers",
        "//nes:text_list",
        "//nes:z2decompress",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//util:config",
        "@com_google_absl//absl/strings",
    ],
)

cc_library(
    name = "multimap",
    srcs = ["multimap.cc"],
    hdrs = ["multimap.h"],
    deps = [
        ":base",
        ":glbitmap",
        ":simplemap",
        "//alg:fdg",
        "//alg:palace_gen",
        "//external:imgui",
        "//nes:mappers",
        "//nes:z2decompress",
        "//nes:z2objcache",
        "//proto:generator",
        "//proto:rominfo",
        "//util:config",
    ],
)

cc_library(
    name = "map_connect",
    srcs = ["map_connect.cc"],
    hdrs = ["map_connect.h"],
    deps = [
        ":base",
        ":multimap",
        "//external:imgui",
        "//nes:mappers",
        "//proto:rominfo",
    ],
)

cc_library(
    name = "misc_hacks",
    srcs = ["misc_hacks.cc"],
    hdrs = ["misc_hacks.h"],
    deps = [
        ":base",
        "//external:imgui",
        "//nes:mappers",
        "//proto:rominfo",
        "//util:config",
    ],
)

cc_library(
    name = "overworld_encounters",
    srcs = ["overworld_encounters.cc"],
    hdrs = ["overworld_encounters.h"],
    deps = [
        "//external:imgui",
        "//nes:mappers",
        "//util:config",
    ],
)

cc_library(
    name = "palace_gfx",
    srcs = ["palace_gfx.cc"],
    hdrs = ["palace_gfx.h"],
    deps = [
        ":base",
        "//external:imgui",
        "//nes:mappers",
        "//proto:rominfo",
        "//util:config",
    ],
)

cc_library(
    name = "randomize",
    srcs = ["randomize.cc"],
    hdrs = ["randomize.h"],
    deps = [
        ":map_connect",
        "//alg:terrain",
        "//external:imgui",
        "//proto:rominfo",
        "//util:config",
        "//util:stb-tilemap-editor",
    ],
)

cc_library(
    name = "start_values",
    srcs = ["start_values.cc"],
    hdrs = ["start_values.h"],
    deps = [
        ":base",
        "//external:imgui",
        "//nes:mappers",
        "//util:config",
    ],
)

cc_library(
    name = "object_table",
    srcs = ["object_table.cc"],
    hdrs = ["object_table.h"],
    deps = [
        ":base",
        ":hwpalette",
        "//external:imgui",
        "//nes:mappers",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//util:config",
    ],
)

cc_library(
    name = "enemyattr",
    srcs = ["enemyattr.cc"],
    hdrs = ["enemyattr.h"],
    deps = [
        ":base",
        ":error_dialog",
        "//external:gflags",
        "//external:imgui",
        "//nes:mappers",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//util:config",
    ],
)

cc_library(
    name = "xptable",
    srcs = ["xptable.cc"],
    hdrs = ["xptable.h"],
    deps = [
        ":base",
        ":error_dialog",
        "//external:gflags",
        "//external:imgui",
        "//nes:mappers",
        "//nes:text_encoding",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//util:config",
    ],
)

cc_library(
    name = "text_table",
    srcs = ["text_table.cc"],
    hdrs = ["text_table.h"],
    deps = [
        ":base",
        ":error_dialog",
        "//external:gflags",
        "//external:imgui",
        "//nes:mappers",
        "//nes:text_list",
        "//proto:rominfo",
        "//util:config",
    ],
)

cc_library(
    name = "project",
    srcs = ["project.cc"],
    hdrs = ["project.h"],
    deps = [
        ":base",
        ":error_dialog",
        "//external:imgui",
        "//ips",
        "//nes:cartridge",
        "//proto:project",
        "//util:compress",
        "//util:file",
        "//util:logging",
        "//util:os",
        "//util:status",
    ],
)

cc_library(
    name = "drops",
    srcs = ["drops.cc"],
    hdrs = ["drops.h"],
    deps = [
        ":base",
        ":simplemap",
        "//external:imgui",
        "//nes:mappers",
        "//proto:rominfo",
        "//util:config",
    ],
)

cc_library(
    name = "tile_transform",
    srcs = ["tile_transform.cc"],
    hdrs = ["tile_transform.h"],
    deps = [
        ":base",
        "//external:imgui",
        "//nes:mappers",
        "//proto:rominfo",
        "//util:config",
    ],
)

cc_library(
    name = "item_effects",
    srcs = ["item_effects.cc"],
    hdrs = ["item_effects.h"],
    deps = [
        ":base",
        "//external:imgui",
        "//nes:mappers",
        "//proto:rominfo",
        "//util:config",
    ],
)

cc_library(
    name = "rom_memory",
    srcs = ["rom_memory.cc"],
    hdrs = ["rom_memory.h"],
    deps = [
        ":base",
        ":glbitmap",
        "//external:gflags",
        "//external:imgui",
        "//nes:mappers",
    ],
)
