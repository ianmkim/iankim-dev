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
        )
    },
    # Setting the default package name to `""` forces the use of the macros defined in this repository
    # to always use the root package when looking for dependencies or aliases. This should be considered
    # optional as the repository also exposes alises for easy access to all dependencies.
    render_config = render_config(
        default_package_name = ""
    ),
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()
