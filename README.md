# rip

`rip` is a (mostly personal) task runner.

It is experimental, largely untested, and probably presumes a macos and zsh
shell in multiple places. It's mostly a personal task runner.

## setup

Some rips help with provisioning devices. Here's how to setup a new mac with rust, rip, and brew.

- Install rust via [rustup](https://rustup.rs/) (additional info can be found in [the rust book](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#installing-rustup-on-linux-or-macos)). It could look something like this:

  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

  xcode-select --install
  ```

- Install `rip`

  ```sh
  cargo install --git git@github.com:aegatlin/rip.git
  ```

- Install [brew](https://brew.sh/)

  ```sh
  rip mac brew
  ```

  or,

  ```sh
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  ```
