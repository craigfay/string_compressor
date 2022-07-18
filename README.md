
## What does this library do?
It exports the `StringCompressor` struct that can be used to create minimally compact, non-colliding,
URL-safe, base-64 aliases for a set of strings. It's useful for building minifying tools, among other things.

## When should I use this?
- When you need to hash strings and need your hashed values to be as small as possible.
- When you need hashed values to be safe for use in URLs and file names.
- When you need to un-hash previously hashed strings.

## When should I use something else?
- When you need to uniquely hash more than 2^64 strings using a single instance. In the future, `StringCompressor` will support higher concurrently unique hashes.
- When you need your hasher not to affect memory usage. In the future, `StringCompressor` will support a future flag that allows zero scaling in memory, at the expense of disallowing un-hashing.
- When hashed values need to be transmitted and un-hashed by another program.

## What characters can a hashed value contain?
- Lowercase and uppercase alphabetical ASCII characters: `a` through `z` and `A` through `Z`
- Numerical ASCII characters: `0` through `9`
- Dashes and underscores: `-` and `_`
- Hashed values are un-padded to minimize size.

## How can I use this in my own Rust project?

Add this crate as a dependency in you're `Cargo.toml`:

```toml
[dependencies]

# Install from Crates.io
string_compressor = { version = "1.0.1" }

# OR install from GitHub Release
string_compressor = { git = "https://github.com/craigfay/string_compressor", tag = "v1.0.1" }

# OR install from GitHub Branch
string_compressor = { git = "https://github.com/craigfay/string_compressor", branch = "main"  }
```

## Links
- [Documentation](https://docs.rs/string_compressor/latest/string_compressor/struct.StringCompressor.html)
- [Crate Page](https://crates.io/crates/string_compressor)

