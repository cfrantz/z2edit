# Copyright lowRISC contributors.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

load("//rules:repo.bzl", "http_archive_or_local")

IGFD_GITHASH = "845cfab617551b418d36be07606fe4e74d4c0a4c"
IGFD_SHA256 = ""

def imgui_repos(
        imgui = None,
        igfd = None):
    http_archive_or_local(
        name = "com_github_imgui",
        local = imgui,
        urls = ["https://github.com/ocornut/imgui/archive/refs/tags/v1.89.9.tar.gz"],
        sha256 = "1acc27a778b71d859878121a3f7b287cd81c29d720893d2b2bf74455bf9d52d6",
        strip_prefix = "imgui-1.89.9",
        build_file = Label("//third_party/imgui:BUILD.imgui.bazel"),
    )

    http_archive_or_local(
        name = "com_github_igfd",
        local = igfd,
        sha256 = IGFD_SHA256,
        url = "https://github.com/aiekick/ImGuiFileDialog/archive/{}.tar.gz".format(IGFD_GITHASH),
        strip_prefix = "ImGuiFileDialog-{}".format(IGFD_GITHASH),
        build_file = Label("//third_party/imgui:BUILD.igfd.bazel"),
    )
