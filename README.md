# github-username-regex-rust

[![test](https://github.com/GrantBirki/github-username-regex-rust/actions/workflows/test.yml/badge.svg)](https://github.com/GrantBirki/github-username-regex-rust/actions/workflows/test.yml) [![lint](https://github.com/GrantBirki/github-username-regex-rust/actions/workflows/lint.yml/badge.svg)](https://github.com/GrantBirki/github-username-regex-rust/actions/workflows/lint.yml) [![build](https://github.com/GrantBirki/github-username-regex-rust/actions/workflows/build.yml/badge.svg)](https://github.com/GrantBirki/github-username-regex-rust/actions/workflows/build.yml) [![release](https://github.com/GrantBirki/github-username-regex-rust/actions/workflows/release.yml/badge.svg)](https://github.com/GrantBirki/github-username-regex-rust/actions/workflows/release.yml)

A lightweight Rust crate to check if a GitHub username / handle is valid

## Installation 📦

You can use this crate from [crates.io](https://crates.io/crates/github_username_regex) with the following command:

```bash
cargo add github_username_regex
```

> This will add the latest version of this crate to your `Cargo.toml` file

Or you can use a version:

```toml
# Cargo.toml

[dependencies]
github_username_regex = "X.X.X" # where X.X.X is the version you want to use
```

## Usage 💻

```rust
todo
```

## Release 🚀

To release a new version of this gem, simply edit the [`Cargo.toml`](Cargo.toml) file in this repo. You just need to update the `version` value. When you commit (or merge) your changes to `main`, a new version will be automatically released via GitHub Actions to [crates.io](https://crates.io). Addtionally, a new release tag will be pushed to this repository as well.

## Note 📝

This Crate doesn't take reserved usernames into consideration. For example, it matches `help`, `about` and `pricing`, though they are reserved words and cannot be used as GitHub usernames.

## Credits 🙏

This Crate is based on the following npm [package](https://github.com/shinnn/github-username-regex)
