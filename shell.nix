 { pkgs ? import <nixpkgs> {} }:

 (pkgs.buildFHSEnv {
   name = "bazelenv";
   targetPkgs = pkgs: [
     pkgs.bazel
     pkgs.glibc
     pkgs.gcc
     pkgs.zsh
     pkgs.zlib
     pkgs.docker
   ];
 }).env

