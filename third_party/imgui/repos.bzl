# Copyright lowRISC contributors.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

load("//rules:repo.bzl", "http_archive_or_local")

IGFD_GITHASH = "845cfab617551b418d36be07606fe4e74d4c0a4c"
IGFD_SHA256 = "7e8baa6b1476237d14d0a48b69b66fc50c2f08c892de4fa86b1fe6346cf497a3"

# ImGui string_view branch
IMGUI_GITHASH = "5400c01eb2b394430bc21b063e7408268ff0d48e"
IMGUI_SHA256 = "21517da5dcd4534fcb0d15dcac6e32ac0d1e389a2469fd857cd6df933f469544"

def imgui_repos(
        imgui = None,
        igfd = None,
        implot = None):
    #http_archive_or_local(
    #    name = "com_github_imgui",
    #    local = imgui,
    #    urls = ["https://github.com/ocornut/imgui/archive/refs/tags/v1.89.9.tar.gz"],
    #    sha256 = "1acc27a778b71d859878121a3f7b287cd81c29d720893d2b2bf74455bf9d52d6",
    #    strip_prefix = "imgui-1.89.9",
    #    build_file = Label("//third_party/imgui:BUILD.imgui.bazel"),
    #)
    http_archive_or_local(
        name = "com_github_imgui",
        local = imgui,
        sha256 = IMGUI_SHA256,
        url = "https://github.com/ocornut/imgui/archive/{}.tar.gz".format(IMGUI_GITHASH),
        strip_prefix = "imgui-{}".format(IMGUI_GITHASH),
        build_file = Label("//third_party/imgui:BUILD.imgui.bazel"),
        patches = [
            "//third_party/imgui/patches:imgui_string_view.patch",
        ],
        patch_args = ["-p1"],
    )

    http_archive_or_local(
        name = "com_github_igfd",
        local = igfd,
        sha256 = IGFD_SHA256,
        url = "https://github.com/aiekick/ImGuiFileDialog/archive/{}.tar.gz".format(IGFD_GITHASH),
        strip_prefix = "ImGuiFileDialog-{}".format(IGFD_GITHASH),
        build_file = Label("//third_party/imgui:BUILD.igfd.bazel"),
        patches = [
            "//third_party/imgui/patches:igfd.patch",
        ],
        patch_args = ["-p1"],
    )

    http_archive_or_local(
        name = "com_github_implot",
        local = implot,
        url = "https://github.com/epezent/implot/archive/refs/tags/v0.16.tar.gz",
        sha256 = "",
        strip_prefix = "implot-0.16",
        build_file = Label("//third_party/imgui:BUILD.implot.bazel"),
    )
