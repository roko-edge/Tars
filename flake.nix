{
  description = "tars-ml: Machine Learning & NPU Hardware built from scratch";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        # Ferramentas de C++ e Build
        cppTools = with pkgs; [
          clang
          clang-tools
          cmake
          gnumake
          gdb
          pkg-config
          fontconfig
          graphviz
        ];

        # Ferramentas de Hardware / NPU (SystemVerilog)
        hardwareTools = with pkgs; [
          iverilog # Compilador e simulador IEEE 1800-2012 (inclui vvp)
          verilator # Linter estático e backend C++
          gtkwave # Visualizador de ondas VCD
          verible # LSP (verible-verilog-ls) e formatador
        ];

        # Toolchain do Rust
        rustTools = with pkgs; [
          rustc
          cargo
          rust-analyzer
          clippy
          rustfmt
        ];

        # Ferramentas de suporte ao Nix
        nixTools = with pkgs; [
          nil
          nixfmt
        ];
      in
      {
        devShells = {
          # Ambiente padrão unificado (carregado pelo direnv na raiz do projeto)
          default = pkgs.mkShell {
            name = "tars-ml-dev";
            packages = cppTools ++ hardwareTools ++ rustTools ++ nixTools;

            shellHook = ''
              export CMAKE_EXPORT_COMPILE_COMMANDS=1
              echo "Ambiente tars-ml carregado (C++, Hardware/NPU, Rust e Nix)!"
            '';
          };

          # Shell específico de hardware (compatível com npu/Makefile: nix develop ..#hardware)
          hardware = pkgs.mkShell {
            name = "tars-ml-hardware";
            packages = hardwareTools ++ [ pkgs.gnumake ];

            shellHook = ''
              echo "Ambiente NPU / SystemVerilog pronto (iverilog, verilator, gtkwave)!"
            '';
          };

          # Shell isolado para Rust (se desejar compilar apenas módulos em Rust)
          rust = pkgs.mkShell {
            name = "tars-ml-rust";
            packages = rustTools;
          };
        };
      }
    );
}
