uchat build tools

This workspace contains all native build utilities for the UChat platform.  
Each tool is focused, minimal, and can be used independently.

structure
    uchat-native
        Builds the Rust native library into libuchat_native.so for Android
        Uses the standalone NDK toolchains directly
        Produces an optimised no-std compatible shared library

    uchat-apksigner
        Fast pure Rust APK signer
        Generates META-INF block and RSA signature
        Compatible with apksigner verify when manifest is kept intact

    uchat-hybrid-signer
        Full signing pipeline using ring for RSA PKCS1 SHA256
        Loads PKCS8 private keys
        Preserves all existing entries
        Injects MANIFEST SF and RSA entries

    scripts
        Utilities for packaging, zipalign, and automated release steps

quick start
    cd uchat-buildtools
    cargo build --release

build native library
    cd uchat-native
    cargo build --release
    output in target/release/libuchat_native.so

sign an apk using hybrid signer
    cd uchat-buildtools
    ./target/release/uchat-hybrid-signer \
        path/to/key.pem \
        path/to/input.apk \
        path/to/output.apk

expected output layout
    META-INF/MANIFEST.MF
    META-INF/CERT.SF
    META-INF/CERT.RSA
    classes.dex
    lib/<abi>/libuchat_native.so
    resources.arsc
    AndroidManifest.xml

goals
    simple reproducible builds
    native Android pipeline without cargo-apk
    deterministic signatures
    fully controlled apk structure