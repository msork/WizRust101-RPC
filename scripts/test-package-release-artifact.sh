#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "${TMP}"' EXIT

mkdir -p "${TMP}/windows"
printf 'fixture executable\n' > "${TMP}/windows/WizRust101-RPC-Windows-app.exe"
bash "${ROOT}/scripts/package-release-artifact.sh" \
  "${TMP}/windows" WizRust101-RPC-Windows-app.exe "${TMP}/portable.zip"
[[ "$(unzip -Z1 "${TMP}/portable.zip")" == WizRust101-RPC-Windows-app.exe ]]

printf 'unexpected sibling\n' > "${TMP}/windows/extra.txt"
if output="$(bash "${ROOT}/scripts/package-release-artifact.sh" \
  "${TMP}/windows" WizRust101-RPC-Windows-app.exe "${TMP}/invalid.zip" 2>&1)"; then
  echo "Artifact repackager accepted an unexpected sibling payload." >&2
  exit 1
fi
grep -Fq 'extra.txt' <<<"${output}" || {
  echo "Artifact mismatch diagnostic omitted the downloaded files." >&2
  exit 1
}

APP="${TMP}/macOS/WizRust101-RPC.app"
mkdir -p "${APP}/Contents/MacOS" "${APP}/Contents/Resources"
printf 'plist\n' > "${APP}/Contents/Info.plist"
printf 'binary\n' > "${APP}/Contents/MacOS/wizrust101-rpc"
printf 'icon\n' > "${APP}/Contents/Resources/AppIcon.icns"
bash "${ROOT}/scripts/package-release-artifact.sh" \
  "${TMP}/macOS" WizRust101-RPC.app "${TMP}/macos-app.zip"
unzip -tq "${TMP}/macos-app.zip" >/dev/null
unzip -Z1 "${TMP}/macos-app.zip" | grep -Fxq \
  'WizRust101-RPC.app/Contents/MacOS/wizrust101-rpc'
unzip -Z1 "${TMP}/macos-app.zip" | grep -Fxq \
  'WizRust101-RPC.app/Contents/Resources/AppIcon.icns'

echo "Artifact repackager tests passed."
