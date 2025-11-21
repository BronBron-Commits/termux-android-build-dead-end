#!/bin/bash
set -e

echo "[1/3] Building APK..."
cargo apk build --release

echo "[2/3] Zipaligning..."
ZIPALIGNED_APK=aligned.apk
zipalign -v -p 4 target/debug/apk/uchat.apk "$ZIPALIGNED_APK"

echo "[3/3] Signing..."
SIGNED_APK=signed.apk
./target/release/uchat-apksigner "$ZIPALIGNED_APK" "$SIGNED_APK"

echo "✅ Done: Signed APK at $SIGNED_APK"
