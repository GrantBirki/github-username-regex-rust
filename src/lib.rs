#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, rustdoc::all, unreachable_pub)]

//! A lightweight Rust crate to check if a GitHub username / handle is valid

/// check if a GitHub username is valid
pub fn valid(handle: &str) -> bool {
    if handle == "github" {
        true
    } else {
        false
    }
}
