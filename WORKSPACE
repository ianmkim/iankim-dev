load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# To find additional information on this release or newer ones visit:
# https://github.com/bazelbuild/rules_rust/releases
http_archive(
    name = "rules_rust",
    integrity = "sha256-Weev1uz2QztBlDA88JX6A1N72SucD1V8lBsaliM0TTg=",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.48.0/rules_rust-v0.48.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    versions = [
        "1.79.0",
    ],
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository", "render_config")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//third_party:Cargo.lock",
    lockfile = "//third_party:Cargo.Bazel.lock",
    packages = {
        "axum": crate.spec(
            version = "0.7.5",
        ),
        "axum-extra": crate.spec(
            version = "0.9.3",
        ),
        "tokio": crate.spec(
            version = "1.40.0",
            features = ["full"],
        ),
        "tower": crate.spec(
            version = "0.5.0",
            features = ["full"],
        ),
        "tower-http": crate.spec(
            version = "0.5.2",
            features = ["fs", "trace"],
        ),
        "tracing": crate.spec(
            version = "0.1.40",
        ),
        "tracing-subscriber": crate.spec(
            version = "0.3.18",
            features = ["env-filter"],
        ),
        "clap": crate.spec(
          version = "4.5.16",
          features = ["derive"],
        ),
        "minijinja": crate.spec(
            version = "2.2.0",
            features = ["loader"],
        ),
        "struct_iterable": crate.spec(
            version = "0.1.1",
        ),
        "walkdir": crate.spec(
            version = "2.5.0",
        ),
        "serde": crate.spec(
            version = "1.0.209",
            features = ["derive"],
        ),
        "markdown": crate.spec(
            version = "1.0.0-alpha.20",
        ),
    },
    render_config = render_config(
        default_package_name = ""
    ),
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

http_archive(
    name = "rules_pkg",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_pkg/releases/download/0.9.1/rules_pkg-0.9.1.tar.gz",
        "https://github.com/bazelbuild/rules_pkg/releases/download/0.9.1/rules_pkg-0.9.1.tar.gz",
    ],
    sha256 = "8f9ee2dc10c1ae514ee599a8b42ed99fa262b757058f65ad3c384289ff70c4b8",
)
load("@rules_pkg//:deps.bzl", "rules_pkg_dependencies")

rules_pkg_dependencies()

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_oci",
    sha256 = "79e7f80df2840d14d7bc79099b5ed4553398cce8cff1f0df97289a07f7fd213c",
    strip_prefix = "rules_oci-2.0.0-rc0",
    url = "https://github.com/bazel-contrib/rules_oci/releases/download/v2.0.0-rc0/rules_oci-v2.0.0-rc0.tar.gz",
)

load("@rules_oci//oci:dependencies.bzl", "rules_oci_dependencies")

rules_oci_dependencies()

load("@rules_oci//oci:repositories.bzl", "oci_register_toolchains")

oci_register_toolchains(name = "oci")

# You can pull your base images using oci_pull like this:
load("@rules_oci//oci:pull.bzl", "oci_pull")

oci_pull(
    name = "distroless_cc",
    digest = "sha256:8af1a446ef4ba426c1629851784bb9d80d4cf470e314c7b19755dfc5087b8b70",
    image = "gcr.io/distroless/cc",
    platforms = [
        "linux/amd64",
    ],
)

