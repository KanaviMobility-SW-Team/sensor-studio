#!/usr/bin/env bash
set -euo pipefail

IMAGE_NAME="sensor-studio-core-builder"
APP_NAME="sensor-studio-core"
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
DIST_DIR="dist"

if [[ -z "$VERSION" ]]; then
  echo "[ERROR] failed to read version from Cargo.toml"
  exit 1
fi

docker build -t "$IMAGE_NAME" .

docker run --rm \
  --mount type=bind,src="$(pwd)",dst=/work \
  -w /work \
  "$IMAGE_NAME" \
  cargo build --target x86_64-unknown-linux-gnu --release

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

echo "[INFO] artifacts created:"
echo "  - $DIST_DIR/${APP_NAME}-v${VERSION}-linux-x86_64"
echo "  - $DIST_DIR/${APP_NAME}-v${VERSION}-linux-aarch64"