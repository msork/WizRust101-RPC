#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="${1:-${ROOT}/target/linux-packages}"
APP_ID=io.github.msork.WizRust101RPC
APP_NAME=WizRust101-RPC
APPDIR="${OUTPUT_DIR}/${APP_NAME}.AppDir"
APPIMAGE="${OUTPUT_DIR}/${APP_NAME}-linux.AppImage"
TOOL_DIR="${OUTPUT_DIR}/tools"

if [[ -z "${WIZRUST101_RELEASE_DISCORD_APP_ID:-}" || ! "${WIZRUST101_RELEASE_DISCORD_APP_ID}" =~ ^[0-9]+$ ]]; then
  echo "Set WIZRUST101_RELEASE_DISCORD_APP_ID to the project Discord Application ID for this release build." >&2
  exit 2
fi
for command in cargo flatpak flatpak-builder curl sha256sum zip; do
  command -v "${command}" >/dev/null || { echo "${command} is required." >&2; exit 2; }
done

mkdir -p "${OUTPUT_DIR}" "${TOOL_DIR}"
WIZRUST101_RELEASE_DISCORD_APP_ID="${WIZRUST101_RELEASE_DISCORD_APP_ID}" \
  cargo build --locked --release --bin wizrust101-rpc
export WIZRUST101_FLATPAK_BUNDLE_PATH="${OUTPUT_DIR}/${APP_NAME}-linux.flatpak"
export WIZRUST101_FLATPAK_PREBUILT_BINARY="${ROOT}/target/release/wizrust101-rpc"
bash "${ROOT}/scripts/build-flatpak.sh"

# The low-level upstream tool creates an AppImage from this deliberately small
# AppDir. Pin both its version and the separately versioned Type 2 runtime.
APPIMAGETOOL_VERSION=1.9.1
APPIMAGETOOL_SHA256="${WIZRUST101_APPIMAGETOOL_SHA256:-}"
RUNTIME_VERSION=20251108
RUNTIME_SHA256="${WIZRUST101_APPIMAGE_RUNTIME_SHA256:-}"
if [[ ! "${APPIMAGETOOL_SHA256}" =~ ^[[:xdigit:]]{64}$ || ! "${RUNTIME_SHA256}" =~ ^[[:xdigit:]]{64}$ ]]; then
  echo "AppImage tool and runtime release digests are required." >&2
  exit 2
fi
APPIMAGETOOL="${TOOL_DIR}/appimagetool-x86_64.AppImage"
RUNTIME="${TOOL_DIR}/runtime-x86_64"
curl --fail --location --retry 3 \
  "https://github.com/AppImage/appimagetool/releases/download/${APPIMAGETOOL_VERSION}/appimagetool-x86_64.AppImage" \
  --output "${APPIMAGETOOL}"
echo "${APPIMAGETOOL_SHA256}  ${APPIMAGETOOL}" | sha256sum --check --status
curl --fail --location --retry 3 \
  "https://github.com/AppImage/type2-runtime/releases/download/${RUNTIME_VERSION}/runtime-x86_64" \
  --output "${RUNTIME}"
echo "${RUNTIME_SHA256}  ${RUNTIME}" | sha256sum --check --status
chmod 0755 "${APPIMAGETOOL}"
chmod 0755 "${RUNTIME}"

rm -rf "${APPDIR}"
install -D -m 0755 "${ROOT}/target/release/wizrust101-rpc" "${APPDIR}/usr/bin/wizrust101-rpc"
install -D -m 0644 "${ROOT}/packaging/flatpak/icon.svg" "${APPDIR}/${APP_ID}.svg"
install -D -m 0644 "${ROOT}/packaging/flatpak/icon.svg" "${APPDIR}/usr/share/icons/hicolor/scalable/apps/${APP_ID}.svg"
install -D -m 0644 "${ROOT}/packaging/flatpak/io.github.msork.WizRust101RPC.desktop" \
  "${APPDIR}/usr/share/applications/${APP_ID}.desktop"
sed 's/^Exec=wizrust101-rpc$/Exec=AppRun/' \
  "${APPDIR}/usr/share/applications/${APP_ID}.desktop" > "${APPDIR}/${APP_ID}.desktop"
cat > "${APPDIR}/AppRun" <<'APPRUN'
#!/bin/sh
APPDIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
exec "${APPDIR}/usr/bin/wizrust101-rpc" "$@"
APPRUN
chmod 0755 "${APPDIR}/AppRun"

ARCH=x86_64 APPIMAGE_EXTRACT_AND_RUN=1 "${APPIMAGETOOL}" \
  --runtime-file="${RUNTIME}" "${APPDIR}" "${APPIMAGE}"
chmod 0755 "${APPIMAGE}"

# The downloadable artifact is one explicit ZIP with both packages at its root.
# Upload this ZIP as-is so Actions does not wrap it in a second archive.
RELEASE_ZIP="${OUTPUT_DIR}/WizRust101-RPC-Linux.zip"
rm -f "${RELEASE_ZIP}"
zip -j -X "${RELEASE_ZIP}" \
  "${OUTPUT_DIR}/${APP_NAME}-linux.flatpak" \
  "${APPIMAGE}"
echo "Created ${RELEASE_ZIP} containing the Flatpak and AppImage."
