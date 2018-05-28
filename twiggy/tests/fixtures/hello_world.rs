//! Rebuild `hello_elf.o` with:
//!
//! ```
//! rustc +nightly -g --target x86_64-unknown-linux-gnu hello_world.rs -o hello_elf.o -C lto=fat -C opt-level=z
//! ```
//!
//! Rebuild `hello_mach.o` with:
//!
//! ```
//! rustc +nightly -g --target x86_64-apple-darwin hello_world.rs -o hello_mach.o -C lto=fat -C opt-level=z
//! ```
//! NOTE: The above is not working for me on Ubuntu. This causes an error when `ld` is invoked.

fn main() {
    println!("Hello, world!");
}
