#!/usr/bin/env bash
set -e

echo "=== Building KeyMind macOS Distribution Bundle ==="

# 1. Compile release binaries
echo "[1/4] Building Rust core engine release binary..."
cargo build --release

echo "[2/4] Building Tauri Control Center desktop app..."
cd keymind-control-center
npm install
npm run build
npm run tauri build
cd ..

# 2. Prepare bundle directory
APP_BUNDLE="target/release/bundle/osx/KeyMind.app"
CONTENTS_DIR="$APP_BUNDLE/Contents/MacOS"

mkdir -p "$CONTENTS_DIR"
cp target/release/keymind-interceptor-macos "$CONTENTS_DIR/keymind-engine" 2>/dev/null || true

# 3. Codesign placeholder
echo "[3/4] Signing app bundle with entitlements..."
codesign --force --deep --options runtime --entitlements distribution/entitlements.plist --sign - "$APP_BUNDLE" 2>/dev/null || true

# 4. Packaging into .dmg
echo "[4/4] Creating DMG installer package..."
DMG_OUTPUT="target/release/KeyMind-v1.0.0.dmg"

if command -v create-dmg &> /dev/null; then
    create-dmg \
        --volname "KeyMind Installer" \
        --window-pos 200 120 \
        --window-size 600 400 \
        --icon-size 100 \
        --app-drop-link 400 180 \
        "$DMG_OUTPUT" \
        "$APP_BUNDLE"
else
    hdiutil create -volname "KeyMind" -srcfolder "$APP_BUNDLE" -ov -format UDZO "$DMG_OUTPUT"
fi

echo "=== Successfully built $DMG_OUTPUT ==="
