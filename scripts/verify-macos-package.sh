#!/usr/bin/env bash
set -euo pipefail

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "Package verification requires macOS tools." >&2
  exit 1
fi

OUTPUT_DIR="${1:?usage: verify-macos-package.sh OUTPUT_DIR ARCH}"
EXPECTED_ARCH="${2:?usage: verify-macos-package.sh OUTPUT_DIR ARCH}"
EXPECTED_APP_ID="${WIZRUST101_CI_EXPECTED_APP_ID:?set WIZRUST101_CI_EXPECTED_APP_ID to the expected build ID}"
APP="${OUTPUT_DIR}/WizRust101-RPC.app"
EXECUTABLE="${APP}/Contents/MacOS/wizrust101-rpc"
PKG="${OUTPUT_DIR}/WizRust101-RPC-${EXPECTED_ARCH}.pkg"
APP_ZIP="${OUTPUT_DIR}/WizRust101-RPC-${EXPECTED_ARCH}.app.zip"
TMP="$(mktemp -d)"
trap 'rm -rf "${TMP}"' EXIT

[[ -d "${APP}/Contents/MacOS" && -d "${APP}/Contents/Resources" ]] || { echo "Missing app bundle directories." >&2; exit 1; }
[[ -x "${EXECUTABLE}" ]] || { echo "App executable is missing or not executable." >&2; exit 1; }
[[ -s "${APP}/Contents/Resources/AppIcon.icns" ]] || { echo "App icon is missing." >&2; exit 1; }
[[ -s "${PKG}" && -s "${APP_ZIP}" ]] || { echo "Installer or app ZIP is missing." >&2; exit 1; }
plutil -lint "${APP}/Contents/Info.plist"
assert_plist() {
  local key="$1" expected="$2" actual
  actual="$(/usr/libexec/PlistBuddy -c "Print :${key}" "${APP}/Contents/Info.plist")"
  if [[ "${actual}" != "${expected}" ]]; then
    echo "Info.plist ${key}: expected '${expected}', found '${actual}'." >&2
    exit 1
  fi
}
assert_plist CFBundleIdentifier com.msork.WizRust101RPC
assert_plist CFBundleExecutable wizrust101-rpc
assert_plist CFBundlePackageType APPL
assert_plist LSMinimumSystemVersion 15.0
assert_plist LSUIElement true
iconutil -c iconset "${APP}/Contents/Resources/AppIcon.icns" -o "${TMP}/verified.iconset"

ARCHS="$(lipo -archs "${EXECUTABLE}" | tr ' ' '\n' | sort | tr '\n' ' ' | xargs)"
case "${EXPECTED_ARCH}:${ARCHS}" in
  arm64:arm64|x86_64:x86_64|universal2:arm64\ x86_64) ;;
  *) echo "Expected ${EXPECTED_ARCH}, found Mach-O architectures: ${ARCHS}" >&2; exit 1 ;;
esac

# The executable exits before AppKit/tray startup, verifies the exact ID embedded
# at compile time, and never prints either expected or embedded secret values.
WIZRUST101_CI_EXPECTED_APP_ID="${EXPECTED_APP_ID}" "${EXECUTABLE}" --ci-load-check

SIGNATURE_STATUS="$(pkgutil --check-signature "${PKG}" 2>&1 || true)"
grep -qi 'no signature' <<<"${SIGNATURE_STATUS}" \
  || { echo "Expected an unsigned development package." >&2; exit 1; }
pkgutil --expand-full "${PKG}" "${TMP}/expanded"
PACKAGE_INFO="$(find "${TMP}/expanded" -name PackageInfo -print -quit)"
[[ -n "${PACKAGE_INFO}" ]] || { echo "Expanded installer has no PackageInfo metadata." >&2; exit 1; }
grep -q 'identifier="com.msork.WizRust101RPC"' "${PACKAGE_INFO}" \
  || { echo "Installer package identifier is incorrect." >&2; exit 1; }
grep -q 'version="0.1.0"' "${PACKAGE_INFO}" \
  || { echo "Installer package version is incorrect." >&2; exit 1; }
grep -q 'install-location="/Applications"' "${PACKAGE_INFO}" \
  || { echo "Installer package install location is incorrect: $(grep -o 'install-location="[^"]*"' "${PACKAGE_INFO}" || true)." >&2; exit 1; }
[[ -x "${TMP}/expanded/Applications/WizRust101-RPC.app/Contents/MacOS/wizrust101-rpc" ]] \
  || { echo "Expanded package does not contain an executable app bundle at /Applications." >&2; exit 1; }
cmp "${EXECUTABLE}" "${TMP}/expanded/Applications/WizRust101-RPC.app/Contents/MacOS/wizrust101-rpc"

ditto -x -k "${APP_ZIP}" "${TMP}/zip"
[[ -x "${TMP}/zip/WizRust101-RPC.app/Contents/MacOS/wizrust101-rpc" ]] \
  || { echo "App ZIP does not preserve the bundle executable." >&2; exit 1; }
cmp "${EXECUTABLE}" "${TMP}/zip/WizRust101-RPC.app/Contents/MacOS/wizrust101-rpc"
echo "Verified ${EXPECTED_ARCH} app bundle, unsigned installer, app ZIP, and embedded release ID."
