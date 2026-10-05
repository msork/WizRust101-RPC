#!/usr/bin/env bash
set -euo pipefail

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "Build the native macOS app on macOS with Xcode Command Line Tools installed." >&2
  exit 1
fi
if [[ -n "${WIZRUST101_RELEASE_DISCORD_APP_ID:-}" && ! "${WIZRUST101_RELEASE_DISCORD_APP_ID}" =~ ^[0-9]+$ ]]; then
  echo "WIZRUST101_RELEASE_DISCORD_APP_ID must contain only digits." >&2
  exit 1
fi
if [[ -n "${WIZRUST101_MACOS_NOTARY_PROFILE:-}" \
  && ( -z "${WIZRUST101_MACOS_CODESIGN_IDENTITY:-}" \
    || -z "${WIZRUST101_MACOS_INSTALLER_IDENTITY:-}" ) ]]; then
  echo "Notarization requires both Developer ID Application and Installer identities." >&2
  exit 1
fi

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="${1:-${ROOT}/target/macos}"
ARCH="${WIZRUST101_MACOS_ARCH:-arm64}"
APP_NAME="WizRust101-RPC.app"
APP_DIR="${OUTPUT_DIR}/${APP_NAME}"
rm -f "${OUTPUT_DIR}/WizRust101-RPC-macOS.zip"
STAGE="$(mktemp -d)"
ICONSET="${STAGE}/AppIcon.iconset"
mkdir -p "${OUTPUT_DIR}" "${ICONSET}" "${APP_DIR}/Contents/MacOS" "${APP_DIR}/Contents/Resources"
trap 'rm -rf "${STAGE}"' EXIT

export MACOSX_DEPLOYMENT_TARGET=15.0
case "${ARCH}" in
  arm64)
    TARGETS=(aarch64-apple-darwin)
    ;;
  x86_64)
    TARGETS=(x86_64-apple-darwin)
    ;;
  universal2)
    TARGETS=(aarch64-apple-darwin x86_64-apple-darwin)
    ;;
  *)
    echo "WIZRUST101_MACOS_ARCH must be arm64, x86_64, or universal2." >&2
    exit 1
    ;;
esac

for target in "${TARGETS[@]}"; do
  rustup target add "${target}"
  (cd "${ROOT}" && cargo build --locked --release --target "${target}" --bin wizrust101-rpc)
done
if [[ "${ARCH}" == "universal2" ]]; then
  lipo -create \
    "${ROOT}/target/aarch64-apple-darwin/release/wizrust101-rpc" \
    "${ROOT}/target/x86_64-apple-darwin/release/wizrust101-rpc" \
    -output "${APP_DIR}/Contents/MacOS/wizrust101-rpc"
else
  cp "${ROOT}/target/${TARGETS[0]}/release/wizrust101-rpc" \
    "${APP_DIR}/Contents/MacOS/wizrust101-rpc"
fi
chmod 755 "${APP_DIR}/Contents/MacOS/wizrust101-rpc"
cp "${ROOT}/packaging/macos/Info.plist" "${APP_DIR}/Contents/Info.plist"

SOURCE_ICON="${ROOT}/assets/icons/sizes/1024.png"
for entry in "16x16 16" "16x16@2x 32" "32x32 32" "32x32@2x 64" "128x128 128" "128x128@2x 256" "256x256 256" "256x256@2x 512" "512x512 512" "512x512@2x 1024"; do
  name="${entry%% *}"
  size="${entry##* }"
  sips -s format png -z "${size}" "${size}" "${SOURCE_ICON}" --out "${ICONSET}/icon_${name}.png" >/dev/null
done
iconutil -c icns "${ICONSET}" -o "${APP_DIR}/Contents/Resources/AppIcon.icns"
chmod 644 "${APP_DIR}/Contents/Info.plist" "${APP_DIR}/Contents/Resources/AppIcon.icns"

if [[ -n "${WIZRUST101_MACOS_CODESIGN_IDENTITY:-}" ]]; then
  codesign --force --options runtime --timestamp \
    --sign "${WIZRUST101_MACOS_CODESIGN_IDENTITY}" "${APP_DIR}"
  codesign --verify --deep --strict --verbose=2 "${APP_DIR}"
fi

rm -f "${OUTPUT_DIR}/WizRust101-RPC-macOS.pkg"
if [[ -n "${WIZRUST101_MACOS_INSTALLER_IDENTITY:-}" ]]; then
  pkgbuild --component "${APP_DIR}" --install-location /Applications \
    --identifier com.msork.WizRust101RPC --version 0.1.0 \
    --sign "${WIZRUST101_MACOS_INSTALLER_IDENTITY}" \
    "${OUTPUT_DIR}/WizRust101-RPC-macOS.pkg"
else
  pkgbuild --component "${APP_DIR}" --install-location /Applications \
    --identifier com.msork.WizRust101RPC --version 0.1.0 \
    "${OUTPUT_DIR}/WizRust101-RPC-macOS.pkg"
fi
if [[ -n "${WIZRUST101_MACOS_INSTALLER_IDENTITY:-}" ]]; then
  pkgutil --check-signature "${OUTPUT_DIR}/WizRust101-RPC-macOS.pkg"
fi

if [[ -n "${WIZRUST101_MACOS_NOTARY_PROFILE:-}" ]]; then
  xcrun notarytool submit "${OUTPUT_DIR}/WizRust101-RPC-macOS.pkg" \
    --keychain-profile "${WIZRUST101_MACOS_NOTARY_PROFILE}" --wait
  xcrun stapler staple "${OUTPUT_DIR}/WizRust101-RPC-macOS.pkg"
fi

RELEASE_DIR="${STAGE}/WizRust101-RPC-macOS"
mkdir -p "${RELEASE_DIR}"
ditto "${APP_DIR}" "${RELEASE_DIR}/WizRust101-RPC.app"
cp -p "${OUTPUT_DIR}/WizRust101-RPC-macOS.pkg" "${RELEASE_DIR}/WizRust101-RPC-macOS.pkg"
ditto -c -k --sequesterRsrc --keepParent "${RELEASE_DIR}" \
  "${OUTPUT_DIR}/WizRust101-RPC-macOS.zip"

echo "Created ${OUTPUT_DIR}/WizRust101-RPC-macOS.zip containing the app bundle and installer."
