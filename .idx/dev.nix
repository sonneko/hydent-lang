{ pkgs, ... }: {
  channel = "unstable";
  packages = [
    pkgs.cargo
    pkgs.rustc
    pkgs.rustfmt
    pkgs.stdenv.cc
    pkgs.clippy
    pkgs.nodejs_latest
    pkgs.llvm_21
  ];
  env = {
    RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
    LLVM_SYS_211_PREFIX = "${pkgs.llvm.dev}";
  };
  idx = {
    extensions = [
      "rust-lang.rust-analyzer"
      "tamasfe.even-better-toml"
      "serayuzgur.crates"
      "vadimcn.vscode-lldb"
    ];
    workspace = {
      onCreate = {

      };
    };
  };
}