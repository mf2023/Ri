# DMSC Java Native Libraries

This directory contains the native JNI libraries for different platforms.

## Directory Structure

```
native/
├── windows/
│   ├── x64/dmsc.dll
│   └── x86/dmsc.dll
├── linux/
│   ├── x64/libdmsc.so
│   └── arm64/libdmsc.so
└── macos/
    ├── x64/libdmsc.dylib
    └── arm64/libdmsc.dylib
```

## Building Native Libraries

To build the native libraries, run:

```bash
# Build for current platform
cargo build --features java --release

# The output will be in:
# Windows: target/release/dmsc.dll
# Linux: target/release/libdmsc.so
# macOS: target/release/libdmsc.dylib
```

## Cross-Compilation

For cross-compilation, you need to set up the appropriate toolchains:

```bash
# Linux x64
rustup target add x86_64-unknown-linux-gnu
cargo build --target x86_64-unknown-linux-gnu --features java --release

# Linux arm64
rustup target add aarch64-unknown-linux-gnu
cargo build --target aarch64-unknown-linux-gnu --features java --release

# Windows x64
rustup target add x86_64-pc-windows-msvc
cargo build --target x86_64-pc-windows-msvc --features java --release

# macOS x64
rustup target add x86_64-apple-darwin
cargo build --target x86_64-apple-darwin --features java --release

# macOS arm64
rustup target add aarch64-apple-darwin
cargo build --target aarch64-apple-darwin --features java --release
```

## Copying Libraries

After building, copy the libraries to this directory:

```bash
# Windows x64
cp target/release/dmsc.dll src/main/resources/native/windows/x64/

# Linux x64
cp target/release/libdmsc.so src/main/resources/native/linux/x64/

# macOS x64
cp target/release/libdmsc.dylib src/main/resources/native/macos/x64/
```
