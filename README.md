U-Chat Buildtools  
A modular Rust framework for constructing, aligning, and signing Android APKs without relying on the traditional Android build stack.

Overview  
This workspace provides a complete toolchain for turning raw native artifacts into installable APKs. It supports v1 and v2 Android signature schemes, modular cryptographic backends, ZIP manipulation utilities, manifest editing, and standalone command-line tools.

Design Goals  
  Fully modular crates that can be combined or replaced  
  Maximum compatibility with Android’s signing specifications  
  Usable in Termux, Linux, WSL, and CI environments  
  Zero reliance on Java or Android Studio  
  Layered architecture enabling debug-friendly output  
  Future-proof design for v3/v4 signature schemes

Workspace Structure  
  uchat-zip  
    Low-level ZIP reader and writer utilities.

  uchat-zipfs  
    High-level virtual file system over ZIP archives.

  uchat-manifest  
    Reading, writing, and transforming AndroidManifest.xml.

  uchat-rsa  
    PKCS1/PKCS8 RSA utilities for legacy signing and compatibility.

  uchat-v1sig  
    Creates Android v1 signature structures:  
      MANIFEST.MF  
      CERT.SF  
      CERT.RSA

  uchat-v1sig-assembler  
    Assembles all v1 signature layers and injects them into APKs.

  uchat-v2sig  
    Implements Android v2 signature blocks, digests, and signer entries.

  uchat-signer-meta  
    Unified controller for loading keys and selecting v1/v2 strategies.

  uchat-signer-bin  
    CLI interface for signing operations:  
      uchat-signer <key> <input.apk> <output.apk>

  uchat-zipalign  
    Aligns ZIP entries to 4-byte boundaries for Android installation.

Usage  
  Build the entire toolchain:  
    cargo build --release

  Sign an APK using the meta signer:  
    ./target/release/uchat-signer <key.pem> <unsigned.apk> <signed.apk>

Notes  
  All crates are intentionally decoupled.  
  Any component can be replaced with user-defined implementations.  
  Designed for developers building custom Android pipelines, Rust-native apps,
  reproducible APK systems, or bare-metal Termux Android toolchains.