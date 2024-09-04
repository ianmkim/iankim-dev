package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_push")

rust_binary(
  name = "main",
  srcs = ["main.rs"],
  data = [
    "assets",
    "templates",
    "writings",
  ],
  deps = [
    "//routes",
    "//app:state",
    "@crate_index//:axum",
    "@crate_index//:minijinja",
    "@crate_index//:axum-extra",
    "@crate_index//:tokio",
    "@crate_index//:tower",
    "@crate_index//:tower-http",
    "@crate_index//:tracing",
    "@crate_index//:tracing-subscriber",
    "@crate_index//:clap",
  ]
)

filegroup(
  name = "assets",
  srcs = glob(["assets/*"]),
)

filegroup(
  name = "templates",
  srcs = glob(["templates/*"])
)

filegroup(
  name = "writings",
  srcs = glob(["writings/*"])
)

# Step 2: Compress it to layer using pkg_tar
pkg_tar(
    name = "main_layer",
    srcs = [
      ":main",
      ":assets",
      ":templates",
      ":writings",
    ],
    strip_prefix = ".",
)

# Step 3: Build image and add built layer to it
oci_image(
    name = "main_image",
    base = "@ubuntu-jammy",
    tars = [":main_layer"],
    entrypoint = ["/main"],
)

oci_push(
    name = "push",
    image = ":main_image",
    repository = "ghcr.io/ianmkim/iankim-dev",
    remote_tags = ["latest"]
)
