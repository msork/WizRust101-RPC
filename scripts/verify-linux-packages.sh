#!/usr/bin/env bash
set -euo pipefail

OUTPUT_DIR="${1:?usage: verify-linux-packages.sh OUTPUT_DIR}"
EXPECTED_APP_ID="${WIZRUST101_CI_EXPECTED_APP_ID:?expected release Discord App ID is required}"
APP_NAME=WizRust101-RPC
APP_ID=io.github.msork.WizRust101RPC
RELEASE_ZIP="${OUTPUT_DIR}/WizRust101-RPC-Linux.zip"
[[ -s "${RELEASE_ZIP}" ]] || { echo "Combined Linux artifact ZIP is missing." >&2; exit 1; }
mapfile -t ZIP_ENTRIES < <(unzip -Z1 "${RELEASE_ZIP}")
[[ "${#ZIP_ENTRIES[@]}" -eq 2 ]] \
  && { [[ "${ZIP_ENTRIES[0]}" == "${APP_NAME}-linux.AppImage" && "${ZIP_ENTRIES[1]}" == "${APP_NAME}-linux.flatpak" ]] \
    || [[ "${ZIP_ENTRIES[1]}" == "${APP_NAME}-linux.AppImage" && "${ZIP_ENTRIES[0]}" == "${APP_NAME}-linux.flatpak" ]]; } \
  || { echo "Linux artifact ZIP must contain only the AppImage and Flatpak at its root." >&2; exit 1; }
RELEASE_DIR="${OUTPUT_DIR}/release-archive"
rm -rf "${RELEASE_DIR}"
mkdir -p "${RELEASE_DIR}"
unzip -q "${RELEASE_ZIP}" -d "${RELEASE_DIR}"
FLATPAK="${RELEASE_DIR}/${APP_NAME}-linux.flatpak"
APPIMAGE="${RELEASE_DIR}/${APP_NAME}-linux.AppImage"
[[ -x "${APPIMAGE}" ]] || { echo "AppImage is not executable." >&2; exit 1; }
file "${APPIMAGE}" | grep -q 'ELF 64-bit LSB pie executable' || { echo "AppImage runtime is not x86_64 ELF." >&2; exit 1; }

EXTRACT="${OUTPUT_DIR}/appimage-extract"
rm -rf "${EXTRACT}"
mkdir "${EXTRACT}"
(cd "${EXTRACT}" && "${APPIMAGE}" --appimage-extract >/dev/null)
ROOT="${EXTRACT}/squashfs-root"
[[ -x "${ROOT}/AppRun" && -x "${ROOT}/usr/bin/wizrust101-rpc" ]] \
  || { echo "AppImage executable permissions or bundle structure are invalid." >&2; exit 1; }
[[ -s "${ROOT}/${APP_ID}.svg" && -s "${ROOT}/${APP_ID}.desktop" ]] \
  || { echo "AppImage root icon or desktop metadata is missing." >&2; exit 1; }
grep -qx 'Exec=AppRun' "${ROOT}/${APP_ID}.desktop"
grep -qx "Icon=${APP_ID}" "${ROOT}/${APP_ID}.desktop"
WIZRUST101_CI_EXPECTED_APP_ID="${EXPECTED_APP_ID}" APPIMAGE_EXTRACT_AND_RUN=1 \
  "${APPIMAGE}" --ci-load-check

flatpak install --user --noninteractive --assumeyes --bundle "${FLATPAK}"
flatpak info --show-metadata --user "${APP_ID}" >/dev/null
flatpak info --show-permissions --user "${APP_ID}" | grep -q 'org.kde.StatusNotifierWatcher'
DESKTOP="${HOME}/.local/share/flatpak/exports/share/applications/${APP_ID}.desktop"
[[ -s "${DESKTOP}" ]] || { echo "Flatpak desktop entry was not exported." >&2; exit 1; }
[[ -s "${HOME}/.local/share/flatpak/exports/share/icons/hicolor/scalable/apps/${APP_ID}.svg" ]] \
  || { echo "Flatpak icon was not exported." >&2; exit 1; }
flatpak run --user --command=wizrust101-rpc "${APP_ID}" --ci-load-check
echo "Validated the final Linux ZIP contents, executable AppImage, Flatpak install/metadata, and embedded ID."
