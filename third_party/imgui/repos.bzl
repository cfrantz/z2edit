# Copyright lowRISC contributors.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

load("//rules:repo.bzl", "http_archive_or_local")

def imgui_repos(imgui = None):
    http_archive_or_local(
        name = "com_github_imgui",
        local = imgui,
        urls = ["https://github.com/ocornut/imgui/archive/refs/tags/v1.89.9.tar.gz"],
        sha256 = "1acc27a778b71d859878121a3f7b287cd81c29d720893d2b2bf74455bf9d52d6",
        strip_prefix = "imgui-1.89.9",
        build_file = Label("//third_party/imgui:BUILD.imgui.bazel"),
    )
