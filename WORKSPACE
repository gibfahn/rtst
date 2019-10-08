# rules_rust: https://github.com/bazelbuild/rules_rust/

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "io_bazel_rules_rust",
    sha256 = "b6da34e057a31b8a85e343c732de4af92a762f804fc36b0baa6c001423a70ebc",
    strip_prefix = "rules_rust-55f77017a7f5b08e525ebeab6e11d8896a4499d2",
    urls = [
        # Master branch as of 2019-10-07
        "https://github.com/bazelbuild/rules_rust/archive/55f77017a7f5b08e525ebeab6e11d8896a4499d2.tar.gz",
    ],
)

http_archive(
    name = "bazel_skylib",
    sha256 = "2ea8a5ed2b448baf4a6855d3ce049c4c452a6470b1efd1504fdb7c1c134d220a",
    strip_prefix = "bazel-skylib-0.8.0",
    url = "https://github.com/bazelbuild/bazel-skylib/archive/0.8.0.tar.gz",
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")

# rules_docker: https://github.com/bazelbuild/rules_docker#rust_image

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# Download the rules_docker repository at release v0.11.0
http_archive(
    name = "io_bazel_rules_docker",
    sha256 = "d0b345518236e240d513fe0f59f6d3da274f035480273a7eb00af7d216ae2a06",
    strip_prefix = "rules_docker-0.11.1",
    urls = ["https://github.com/bazelbuild/rules_docker/releases/download/v0.11.1/rules_docker-v0.11.1.tar.gz"],
)

# You *must* import the Rust rules before setting up the rust_image rules.

load(
    "@io_bazel_rules_docker//repositories:repositories.bzl",
    container_repositories = "repositories",
)

container_repositories()

load(
    "@io_bazel_rules_docker//rust:image.bzl",
    _rust_image_repos = "repositories",
)

_rust_image_repos()
