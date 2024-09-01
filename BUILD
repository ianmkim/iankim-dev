package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
  name = "main",
  srcs = ["main.rs"],
  data = [
    "assets",
  ],
  deps = [
    "@crate_index//:axum",
    "@crate_index//:axum-extra",
    "@crate_index//:tokio",
    "@crate_index//:tower",
    "@crate_index//:tower-http",
    "@crate_index//:tracing",
    "@crate_index//:tracing-subscriber",
    "@crate_index//:clap",
  ]
)
