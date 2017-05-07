package(default_visibility=["//visibility:public"])

cc_library(
    name = "browser",
    hdrs = [
        "browser.h",
    ],
    srcs = [
        "browser.cc",
    ],
)

cc_library(
    name = "compress",
    linkopts = ["-lz"],
    hdrs = [
        "compress.h",
    ],
    srcs = [
        "compress.cc",
    ],
    deps = [
        "//util:status",
    ],
)

cc_library(
    name = "config",
    hdrs = [
        "config.h",
    ],
    deps = [
        "//util:file",
        "//util:logging",
        "//util:os",
        "//util:string",
    ],
)

cc_library(
	name = "file",
	srcs = [
		"file.cc",
	],
	hdrs = [
		"file.h",
		"string.h",
	],
	deps = [
		":status",
	]
)

cc_library(
	name = "logging",
	hdrs = [
		"logging.h",
	],
	srcs = [
		"logging.cc",
	],
	deps = [
        ":string",
        "//external:gflags",
	]
)

cc_library(
	name = "os",
	srcs = [
		"os.cc",
	],
	hdrs = [
		"os.h",
		"string.h",
	],
	deps = [
        "//util:string",
	]
)

cc_library(
	name = "string",
	hdrs = [
		"string.h",
		"stringpiece.h",
		"stringprintf.h",
		"strutil.h",
	],
	deps = [
		"@google_protobuf//:protobuf",
	]
)

cc_library(
	name = "status",
	hdrs = [
		"status.h",
		"statusor.h",
		"string.h",
	],
	srcs = [
		"status.cc",
	],
	deps = [
		"@google_protobuf//:protobuf",
	]
)


cc_library(
	name = "fpsmgr",
	hdrs = [
		"fpsmgr.h",
	],
)

cc_library(
	name = "stb-tilemap-editor",
	hdrs = [
		"stb_tilemap_editor.h",
    ]
)

genrule(
    name = "gen_fontawsome_h",
    srcs = ["data/fontawesome-webfont.ttf"],
    outs = ["fontawesome.h"],
    cmd = "xxd -i $< $@",
)

cc_library(
    name = "imgui_sdl_opengl",
    hdrs = [
        "imgui_impl_sdl.h",
    ],
    srcs = [
        "imgui_impl_sdl.cpp",
        "fontawesome.h",
    ],
    deps = [
        "//external:imgui",
        "//external:fontawesome",
    ],
)
