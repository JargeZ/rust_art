Rust Art
========

This is a simple Nannou project for people new to Rust or Nannou to start from.

The Nannou documentation is the best source of information, but there are lots of comments in [src/main.rs](src/main.rs).

Nannou home page: https://nannou.cc/

Nannou getting started guide: https://guide.nannou.cc/


Building and Running
--------------------

We've tested this project on Windows 11, Ubuntu Linux 23.10, and MacOS Sonoma.

First, you need a Rust toolchain installed. rustup is an official tool for easily settings up a toolchain on various platforms. The rustup install process is guided and should setup external tooling too: https://rustup.rs/

Second, assuming you have git installed, clone this repository:
```
git clone https://github.com/jbit/rust_art
```

If you don't have git and want a quick start, you can download the code as a zip file: https://github.com/jbit/rust_art/archive/refs/heads/main.zip

Assuming that rustup installed a toolchain, you should now be able to run this command from the `rust_art` directory:
```
cargo run
```

If everything went well it will display a window on screen!


Editing the code
----------------

It's recommended you install VSCode (https://code.visualstudio.com/) however various text editors support Rust.

Once VSCode is installed, open the `rust_art` directory.

VSCode should prompt you to install the "Rust Analyzer" plugin, it's recommended you use this plugin (it provides syntax highlighting, code navigation, and various other nice to haves).

You should be able to edit `src/main.rs` to make modifications.

To test your modifications, save `main.rs`, then either execute `cargo run` from the terminal, or press Ctrl+


How to make this project from scratch
-------------------------------------

It's simple to make this project from scratch yourself:


```
cargo new rust_art
cd rust_art
cargo add nannou
# add fun things to src/main.rs
cargo run --release
```
