######################################################################
# gflags
######################################################################
git_repository(
    name = "gflags_git",
    #commit = "HEAD",  # Use the current HEAD commit
    commit = "74bcd20c0e5b904a67e37abf0c1262824ff9030c",
    remote = "https://github.com/gflags/gflags.git",
)

bind(
    name = "gflags",
    actual = "@gflags_git//:gflags",
)

######################################################################
# imgui
######################################################################
new_git_repository(
    name = "imgui_git",
    tag = "v1.49",
    remote = "https://github.com/ocornut/imgui.git",
    build_file = "imgui.BUILD",
)

bind(
    name = "imgui",
    actual = "@imgui_git//:imgui",
)
bind(
    name = "imgui_sdl_opengl",
    actual = "@imgui_git//:imgui_sdl_opengl",
)

######################################################################
# IconFontCppHeaders
######################################################################
new_git_repository(
	name = "iconfonts",
	remote = "https://github.com/juliettef/IconFontCppHeaders.git",
    commit = "fda5f470b767f7b413e4a3995fa8cfe47f78b586",
    build_file = "iconfonts.BUILD",
)
bind(
    name = "fontawesome",
    actual = "@iconfonts//:fontawesome",
)

######################################################################
# protobuf
######################################################################
git_repository(
	name = "google_protobuf",
	remote = "https://github.com/google/protobuf.git",
	tag = "v3.1.0"
)

######################################################################
# native file dialog
######################################################################
new_git_repository(
	name = "nativefiledialog_git",
	remote = "https://github.com/mlabbe/nativefiledialog.git",
    commit = "5cfe5002eb0fac1e49777a17dec70134147931e2",
    build_file = "nfd.BUILD",
)

bind(
    name = "nfd",
    actual = "@nativefiledialog_git//:nfd",
)


######################################################################
# compilers for windows
######################################################################
git_repository(
    name = "mxebzl",
    remote = "https://github.com/cfrantz/mxebzl.git",
    tag = "20170701_RC02",
)

load("@mxebzl//tools:repository.bzl", "mxe_compilers")
mxe_compilers(
    deps = [
        "compiler",
        "SDL2",
        "SDL2-extras",
    ],
)
