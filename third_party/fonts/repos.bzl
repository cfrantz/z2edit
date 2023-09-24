# Copyright lowRISC contributors.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

load("//rules:repo.bzl", "http_archive_or_local")

ICONFONTS_GITHASH = "1a083cca7d650c0615e32a1d39d56892a2ce8c5b"
ICONFONTS_SHA256 = "bf7d96ff8d883e60e7deba0f2775e5926f7d56ac8159a5a159a9974a47635193"

def fonts_repos(iconfonts = None):
    http_archive_or_local(
        name = "com_github_iconfonts",
        local = iconfonts,
        sha256 = ICONFONTS_SHA256,
        strip_prefix = "IconFontCppHeaders-{}".format(ICONFONTS_GITHASH),
        url = "https://github.com/juliettef/IconFontCppHeaders/archive/{}.tar.gz".format(ICONFONTS_GITHASH),
        build_file = Label("//third_party/fonts:BUILD.iconfonts.bazel"),
    )
