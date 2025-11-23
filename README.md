Termux Android Build Dead End

This repository exists to document the attempt to build an Android UI for U-Chat using Rust directly inside Termux. The idea was to avoid Android Studio, avoid Java and Kotlin, and compile everything with the NDK and Rust toolchains on the device itself.

The project includes experiments using crates such as ndk, ndk-glue, android-activity, and several Rust mobile UI libraries. It contains build logs, failed compilation attempts, notes, and partial code from the process.

The approach was eventually abandoned for several reasons. The Android Rust UI ecosystem is incomplete, inconsistent, and often unmaintained. Many crates have missing features, mismatched versions, or partial implementations. Resource linking, theming, and lifecycle management all expect Android Studio and the Gradle toolchain. Termux introduces more cross-compilation issues than it solves.

The final conclusion was that a native Android front end written in Java or Kotlin, combined with Rust for backend logic, is the most reliable approach. This repository remains as a record of what was attempted and why it did not work.
