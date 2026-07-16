#!/usr/bin/env bash
set -euo pipefail

TARGET_OS="linux"
TARGET_ARCH="x86_64"
FLUTTER_ARCH_DIR="x64"
DIST_DIR="dist"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if [[ ! -f "pubspec.yaml" ]]; then
  echo "[ERROR] pubspec.yaml not found. Please run this script from the ui project directory."
  exit 1
fi

HOST_ARCH="$(uname -m)"
if [[ "$HOST_ARCH" != "x86_64" ]]; then
  echo "[ERROR] this script is for Linux x86_64 build host only. current host: $HOST_ARCH"
  exit 1
fi

if ! command -v flutter >/dev/null 2>&1; then
  echo "[ERROR] flutter command not found."
  exit 1
fi

APP_NAME="$(grep -E '^name:' pubspec.yaml | head -n 1 | cut -d ':' -f 2- | sed 's/#.*$//' | xargs)"
VERSION_RAW="$(grep -E '^version:' pubspec.yaml | head -n 1 | cut -d ':' -f 2- | sed 's/#.*$//' | xargs)"

if [[ -z "$APP_NAME" ]]; then
  echo "[ERROR] failed to read app name from pubspec.yaml"
  exit 1
fi

if [[ -z "$VERSION_RAW" ]]; then
  echo "[ERROR] failed to read version from pubspec.yaml"
  exit 1
fi

# pubspec version example: 0.1.0+1
# artifact version uses only 0.1.0
VERSION="${VERSION_RAW%%+*}"

# Convert Flutter package name style to artifact name style
# sensor_studio_ui -> sensor-studio-ui
PACKAGE_NAME="${APP_NAME//_/-}"

ARTIFACT_NAME="${PACKAGE_NAME}-v${VERSION}-${TARGET_OS}-${TARGET_ARCH}"
BUNDLE_DIR="build/linux/${FLUTTER_ARCH_DIR}/release/bundle"
OUTPUT_DIR="${DIST_DIR}/${ARTIFACT_NAME}"

echo "[INFO] app name      : $APP_NAME"
echo "[INFO] version       : $VERSION_RAW"
echo "[INFO] artifact name : $ARTIFACT_NAME"

flutter pub get
flutter build linux --release

if [[ ! -d "$BUNDLE_DIR" ]]; then
  echo "[ERROR] build output not found: $BUNDLE_DIR"
  exit 1
fi

rm -rf "$OUTPUT_DIR"

mkdir -p "$DIST_DIR"
cp -a "$BUNDLE_DIR" "$OUTPUT_DIR"

cat > "${OUTPUT_DIR}/RELEASE_INFO.txt" <<EOF
Application: ${APP_NAME}
Version: ${VERSION_RAW}
Target: ${TARGET_OS}-${TARGET_ARCH}
Build Host: $(uname -s)-$(uname -m)
Built At UTC: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
EOF

echo "[INFO] release bundle created:"
echo "       ${OUTPUT_DIR}"