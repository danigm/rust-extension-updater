# CLI Tool to update flatpak rust extension

This is a simple tool to update this extension:

https://github.com/flathub/org.freedesktop.Sdk.Extension.rust-stable

# How to use it

```
update <JSON> <CURRENT_VERSION> <NEXT_VERSION>
```

 * JSON is the current extension manifest.
 * CURRENT\_VERSION is the current version, for example 1.27.2
 * NEXT\_VERSION is the next version, for example 1.28.0

This will output a new json so you can redirect the output to a file:

```
cargo build --release
 ./target/release/update org.freedesktop.Sdk.Extension.rust-stable.json 1.27.2 1.28.0 > org.freedesktop.Sdk.Extension.rust-stable.1.28.0.json
```
