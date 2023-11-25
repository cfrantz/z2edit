load("//rules:repo.bzl", "http_archive_or_local")

def pybind11_repos(local = None):
    http_archive_or_local(
        name = "pybind11",
        local = local,
        sha256 = "832e2f309c57da9c1e6d4542dedd34b24e4192ecb4d62f6f4866a737454c9970",
        strip_prefix = "pybind11-2.10.4",
        url = "https://github.com/pybind/pybind11/archive/refs/tags/v2.10.4.tar.gz",
        build_file = "//third_party/python:BUILD.pybind11.bazel",
    )
