<img src="./logo.svg" alt="logo" width="130" />

`peekrs` is a live-reloading file server for previewing HTML components — like Storybook, but (a heck of a lot) simpler.

# Motivation

When building component libraries (e.g., with Askama templates), the feedback loop is slow: edit a template, switch to the browser, hit refresh, check the result, switch back to the editor, repeat.

`peekrs` eliminates that friction. Point it at a directory of HTML files, and it serves them with a file tree and automatic reload on save. Your workflow becomes: edit a temple, check the result, repeat. Bye bye, context switching! 👋

# Usage

Serve your HTML files in directory `components` by running:

```console
$ peekrs components
Serving http://127.0.0.1:3001 …
```

Then open `http://127.0.0.1:3001` in your browser. You will see:

- **left pane**: file tree
- **right pane**: selected file preview
- **status bar**: connection status (hidden when connected)

Pick a file from the file tree. It will be shown in the right pane. Now edit that file with your favorite text editor and save it. The changes will appear in the right pane instantly.

For more options, run `peekrs --help`.

# How it works

The server watches the specified directory for file changes. When a file changes, it notifies clients via a WebSocket. The client then requests the new file. Simple. Effective. Yuge.

# Installation

## Manually

If you're using nix, run

```console
$ nix develop
```

You will enter a nix shell that has access to all the binaries in this project, including the rust toolchain.

If you're not using nix, reconsider your life decisions, then [install rust](https://rust-lang.github.io/rustup/) and

```console
$ cargo install --path .
```

## Via flake.nix

If you want to use this in a project that has a `flake.nix`:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    peekrs.url = "github:twiddler/peekrs";
  };

  outputs = { self, nixpkgs, peekrs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          peekrs.packages.${system}.default
        ];
      };
    };
}
```
