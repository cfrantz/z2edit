# Copyright lowRISC contributors.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

load("@rules_pkg//:deps.bzl", "rules_pkg_dependencies")
load("@rules_python//python:repositories.bzl", "py_repositories")

def bazel_deps():
    rules_pkg_dependencies()
    py_repositories()
