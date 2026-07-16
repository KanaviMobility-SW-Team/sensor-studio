#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
DIST_DIR="$ROOT_DIR/dist"
CORE_DIST_DIR="$ROOT_DIR/core/dist"
UI_DIST_DIR="$ROOT_DIR/ui/dist"

move_dist_contents() {
  local source_dir="$1"
  local component_name="$2"

  if [[ ! -d "$source_dir" ]]; then
    echo "[ERROR] $component_name dist directory was not created: $source_dir"
    return 1
  fi

  if [[ -z "$(find "$source_dir" -mindepth 1 -maxdepth 1 -print -quit)" ]]; then
    echo "[WARN] No $component_name artifacts found in: $source_dir"
    return 0
  fi

  echo "[INFO] Collecting $component_name artifacts..."

  find "$source_dir" \
    -mindepth 1 \
    -maxdepth 1 \
    -exec mv -- {} "$DIST_DIR/" \;
}

echo "[INFO] Preparing root dist directory..."

rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

echo "[INFO] Building Sensor Studio Core..."

(
  cd "$ROOT_DIR/core"
  bash ./build.sh
)

move_dist_contents "$CORE_DIST_DIR" "Core"

echo "[INFO] Building Sensor Studio UI..."

(
  cd "$ROOT_DIR/ui"
  bash ./build_linux_x86_64.sh
)

move_dist_contents "$UI_DIST_DIR" "UI"

echo "[INFO] Sensor Studio build completed successfully."
echo "[INFO] Artifacts: $DIST_DIR"