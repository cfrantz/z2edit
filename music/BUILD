cc_library(
    name = "music",
    srcs = [ "music.cc" ],
    hdrs = [ "music.h" ],
)

cc_binary(
    name = "test",
    srcs = ["main.cc"],
    deps = [":music"],
)

cc_binary(
    name = "solstice",
    srcs = ["solstice.cc"],
    deps = [":music"],
)

cc_binary(
    name = "hack2",
    srcs = ["hack2.cc"],
    deps = [":music"],
)

cc_binary(
    name = "credits",
    srcs = ["credits.cc"],
    deps = [":music"],
)
