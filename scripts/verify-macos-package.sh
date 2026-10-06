#!/usr/bin/env bash
set -euo pipefail

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "Package verification requires macOS tools." >&2
  exit 1
fi

OUTPUT_DIR="${1:?usage: verify-macos-package.sh OUTPUT_DIR ARCH}"
EXPECTED_ARCH="${2:?usage: verify-macos-package.sh OUTPUT_DIR ARCH}"
EXPECTED_APP_ID="${WIZRUST101_CI_EXPECTED_APP_ID:?set WIZRUST101_CI_EXPECTED_APP_ID to the expected build ID}"
APP_ZIP="${OUTPUT_DIR}/WizRust101-RPC-macOS-App.zip"
PKG_ZIP="${OUTPUT_DIR}/WizRust101-RPC-macOS-Pkg.zip"
[[ -s "${APP_ZIP}" && -s "${PKG_ZIP}" ]] || { echo "Separate macOS app/pkg ZIPs are missing." >&2; exit 1; }
TMP="$(mktemp -d)"
trap 'rm -rf "${TMP}"' EXIT
ditto -x -k "${APP_ZIP}" "${TMP}/app-artifact"
ditto -x -k "${PKG_ZIP}" "${TMP}/pkg-artifact"
APP_PAYLOADS="$(find "${TMP}/app-artifact" -mindepth 1 -maxdepth 1 ! -name '__MACOSX' -print)"
PKG_PAYLOADS="$(find "${TMP}/pkg-artifact" -mindepth 1 -maxdepth 1 ! -name '__MACOSX' -print)"
[[ "$(wc -l <<<"${APP_PAYLOADS}" | tr -d ' ')" == 1 && "${APP_PAYLOADS##*/}" == 'WizRust101-RPC.app' ]] || { echo "App ZIP must contain only the app bundle (plus optional macOS resource metadata)." >&2; exit 1; }
[[ "$(wc -l <<<"${PKG_PAYLOADS}" | tr -d ' ')" == 1 && "${PKG_PAYLOADS##*/}" == 'WizRust101-RPC-macOS.pkg' ]] || { echo "Pkg ZIP must contain only the installer package (plus optional macOS resource metadata)." >&2; exit 1; }
APP="${TMP}/app-artifact/WizRust101-RPC.app"
EXECUTABLE="${APP}/Contents/MacOS/wizrust101-rpc"
PKG="${TMP}/pkg-artifact/WizRust101-RPC-macOS.pkg"

[[ -d "${APP}/Contents/MacOS" && -d "${APP}/Contents/Resources" ]] || { echo "Missing app bundle directories." >&2; exit 1; }
[[ -x "${EXECUTABLE}" ]] || { echo "App executable is missing or not executable." >&2; exit 1; }
[[ -s "${APP}/Contents/Resources/AppIcon.icns" ]] || { echo "App icon is missing." >&2; exit 1; }
[[ -s "${PKG}" ]] || { echo "Installer package is missing from its ZIP." >&2; exit 1; }
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
grep -q 'version="26.10.05"' "${PACKAGE_INFO}" \
  || { echo "Installer package version is incorrect." >&2; exit 1; }
grep -q 'install-location="/Applications"' "${PACKAGE_INFO}" \
  || { echo "Installer package install location is incorrect: $(grep -o 'install-location="[^"]*"' "${PACKAGE_INFO}" || true)." >&2; exit 1; }
PKG_EXECUTABLE="$(find "${TMP}/expanded" -path '*/WizRust101-RPC.app/Contents/MacOS/wizrust101-rpc' -type f -print -quit)"
[[ -n "${PKG_EXECUTABLE}" && -x "${PKG_EXECUTABLE}" ]] \
  || { echo "Expanded package does not contain an executable WizRust101-RPC.app bundle." >&2; find "${TMP}/expanded" -maxdepth 7 -print >&2; exit 1; }
cmp "${EXECUTABLE}" "${PKG_EXECUTABLE}"

echo "Verified separate macOS app and pkg ZIPs for ${EXPECTED_ARCH}, including embedded release ID."
