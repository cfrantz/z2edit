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
# protobuf
######################################################################
git_repository(
	name = "google_protobuf",
	remote = "https://github.com/google/protobuf.git",
	#tag = "v3.0.0-beta-2"
	tag = "v3.1.0"
)
