<img src="./logo.svg" alt="logo" width="130" />

`peekrs` is a live-reloading file server for viewing rendered HTML — think Storybook, but (a heck of a lot) simpler.

# Motivation

When building component libraries (e.g., with Askama templates), the feedback loop is slow: edit a template in the text editor, switch to the browser, hit refresh, check the result, switch back to the text editor, repeat.

`peekrs` eliminates that friction. Point it to a directory of HTML files, and it serves them with a file tree and automatic reload on save. Your workflow becomes: edit a template in the text editor, check the result, repeat. Bye bye, context switching! 👋

# Usage

Have `peekrs` watch and serve your files in directory `foo` by running:

```console
$ peekrs foo
Serving http://127.0.0.1:3001 …
```

Then open `http://127.0.0.1:3001` in your web browser. You will see:

- **left pane**: file tree
- **right pane**: selected file preview
- **status bar**: connection status (hidden when connected)

<img width="630" height="437" alt="start screen" src="https://github.com/user-attachments/assets/dd760fec-989f-4492-9064-7d71f6f05855" />

Click on a file in the file tree. It will be shown in the right pane. Now edit that file with your favorite text editor and save it. The changes will appear in the right pane instantly.

<img width="630" height="441" alt="file selected" src="https://github.com/user-attachments/assets/7df37b41-b1b8-4d32-ba5d-7744b5ba8901" />

For more options, run `peekrs --help`.

# How it works

`peekrs` watches the specified directory for file changes. When a file changes, `peekrs` notifies clients via a WebSocket. The client then requests the new file. Simple. Effective. Goated. 🐐

# Supported files

I use `peekrs` for continuously rendering HTML files in a web browser while working on them in a text editor. However, one can use it with other file types, too.

`peekrs` serves the correct MIME types for the most popular file types one might want to continuously render in a web browser. It serves unknown files as binaries. If you'd like to add a MIME type, feel free to open a pull request.

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
