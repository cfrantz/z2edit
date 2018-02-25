package(default_visibility=["//visibility:public"])

cc_library(
    name = "base",
    hdrs = [
        "debug_console.h",
        "imapp.h",
        "imutil.h",
        "imwidget.h",
    ],
    srcs = [
        "debug_console.cc",
        "imapp.cc",
        "imutil.cc",
    ],
    deps = [
        "//util:fpsmgr",
        "//util:imgui_sdl_opengl",
        "//util:logging",
        "//external:gflags",
        "//external:imgui",
    ],
)

cc_library(
    name = "error_dialog",
    hdrs = ["error_dialog.h"],
    srcs = ["error_dialog.cc"],
    deps = [
        ":base",
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
        ":error_dialog",
        ":glbitmap",
        "//nes:mappers",
        "//external:imgui",
        "//external:nfd",
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
        ":randomize",
        ":overworld_encounters",
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
    name = "palette",
    hdrs = ["palette.h"],
    srcs = ["palette.cc"],
    deps = [
        ":base",
        ":error_dialog",
        ":hwpalette",
        "//nes:mappers",
        "//proto:rominfo",
        "//util:config",
        "//external:imgui",
        "//external:gflags",
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
        "//nes:enemylist",
        "//nes:mappers",
        "//nes:text_list",
        "//nes:z2decompress",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//util:config",
        "//util:string",
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
        "//alg:palace_gen",
        "//nes:mappers",
        "//nes:z2decompress",
        "//nes:z2objcache",
        "//proto:generator",
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
        "//proto:rominfo",
        "//util:config",
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
    name = "palace_gfx",
    hdrs = ["palace_gfx.h"],
    srcs = ["palace_gfx.cc"],
    deps = [
        ":base",
        "//nes:mappers",
        "//proto:rominfo",
        "//util:config",
        "//external:imgui",
    ],
)

cc_library(
    name = "randomize",
    hdrs = ["randomize.h"],
    srcs = ["randomize.cc"],
    deps = [
        ":map_connect",
        "//alg:terrain",
        "//proto:rominfo",
        "//util:config",
        "//util:stb-tilemap-editor",
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

cc_library(
    name = "enemyattr",
    hdrs = ["enemyattr.h"],
    srcs = ["enemyattr.cc"],
    deps = [
        ":base",
        ":error_dialog",
        "//nes:mappers",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//util:config",
        "//external:imgui",
        "//external:gflags",
    ],
)

cc_library(
    name = "xptable",
    hdrs = ["xptable.h"],
    srcs = ["xptable.cc"],
    deps = [
        ":base",
        ":error_dialog",
        "//nes:mappers",
        "//nes:text_encoding",
        "//nes:z2objcache",
        "//proto:rominfo",
        "//util:config",
        "//external:imgui",
        "//external:gflags",
    ],
)

cc_library(
    name = "text_table",
    hdrs = ["text_table.h"],
    srcs = ["text_table.cc"],
    deps = [
        ":base",
        ":error_dialog",
        "//nes:mappers",
        "//nes:text_list",
        "//proto:rominfo",
        "//util:config",
        "//external:imgui",
        "//external:gflags",
    ],
)

cc_library(
    name = "project",
    hdrs = ["project.h"],
    srcs = ["project.cc"],
    deps = [
        ":base",
        ":error_dialog",
        "//ips:ips",
        "//nes:cartridge",
        "//proto:project",
        "//util:compress",
        "//util:file",
        "//util:os",
        "//util:logging",
        "//util:status",
        "//external:imgui",
    ],
)

cc_library(
    name = "drops",
    hdrs = ["drops.h"],
    srcs = ["drops.cc"],
    deps = [
        ":base",
        ":simplemap",
        "//nes:mappers",
        "//proto:rominfo",
        "//util:config",
        "//external:imgui",
    ],
)

cc_library(
    name = "tile_transform",
    hdrs = ["tile_transform.h"],
    srcs = ["tile_transform.cc"],
    deps = [
        ":base",
        "//nes:mappers",
        "//proto:rominfo",
        "//util:config",
        "//external:imgui",
    ],
)

