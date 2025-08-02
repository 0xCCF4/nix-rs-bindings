Rust bindings to NIX's C API.

TODO

# Usage
```toml
[ dependencies ]
nix-api-sys = "0.2"
```

There must be a compatible version of [NIX](https://github.com/NixOS/nix/) installed on your system.
`nix-api-sys` will use `pkg-config` to find the NIX library and headers.

This crate is merely a low-level wrapper around the exposed C API of NIX.
A higher-level Rust API is currently in development.

# Documentation
See the official documentation at [github:NixOS/nix](https://github.com/NixOS/nix/blob/master/src/external-api-docs/README.md).

# Cargo features
This crate exposes the following Cargo features:
- `shared`: Instructs `pkgs-config` to search for shared libraries to link against. 
- `static`: Instructs `pkgs-config` to search for static libraries to link against.

The default behavior if none of these features are enabled is `shared`.
In case that both features are enabled, a compile error will be raised,
which can be handled by setting the environment variable `NIX_API_STATIC`.

`NIX_API_STATIC` overwrites the library search type and has a higher priority than
the Cargo features.
If `NIX_API_STATIC` is set to `1`/`true`/`static`, the `static` behavior is used,
otherwise the `shared` behavior is used.

# Contributing
Contributions are welcome! Please open an issue or a pull request on GitHub.
