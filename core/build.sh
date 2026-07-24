#!/usr/bin/env bash
set -euo pipefail

IMAGE_NAME="sensor-studio-core-builder"
APP_NAME="sensor-studio-core"
LIB_NAME="${APP_NAME//-/_}"
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"

DIST_DIR="dist"
ANDROID_TARGET_DIR="target-android"

if [[ -z "$VERSION" ]]; then
  echo "[ERROR] failed to read version from Cargo.toml"
  exit 1
fi

echo "[INFO] building Android arm64-v8a shared library..."

CARGO_TARGET_DIR="$ANDROID_TARGET_DIR" \
  cargo android

echo "[INFO] building Docker image..."

docker build -t "$IMAGE_NAME" .

echo "[INFO] building Linux x86_64 executable..."

docker run --rm \
  --mount type=bind,src="$(pwd)",dst=/work \
  -w /work \
  "$IMAGE_NAME" \
  cargo build --target x86_64-unknown-linux-gnu --release

echo "[INFO] building Linux aarch64 executable..."

docker run --rm \
  --mount type=bind,src="$(pwd)",dst=/work \
  -w /work \
  "$IMAGE_NAME" \
  cargo build --target aarch64-unknown-linux-gnu --release

mkdir -p "$DIST_DIR"

cp "target/x86_64-unknown-linux-gnu/release/$APP_NAME" \
   "$DIST_DIR/${APP_NAME}-v${VERSION}-linux-x86_64"

cp "target/aarch64-unknown-linux-gnu/release/$APP_NAME" \
   "$DIST_DIR/${APP_NAME}-v${VERSION}-linux-aarch64"

cp "$ANDROID_TARGET_DIR/aarch64-linux-android/release/lib${LIB_NAME}.so" \
   "$DIST_DIR/lib${LIB_NAME}-v${VERSION}-android-arm64-v8a.so"

echo "[INFO] artifacts created:"
echo "  - $DIST_DIR/${APP_NAME}-v${VERSION}-linux-x86_64"
echo "  - $DIST_DIR/${APP_NAME}-v${VERSION}-linux-aarch64"
echo "  - $DIST_DIR/lib${LIB_NAME}-v${VERSION}-android-arm64-v8a.so"