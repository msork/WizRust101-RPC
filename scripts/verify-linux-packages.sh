#!/usr/bin/env bash
set -euo pipefail

OUTPUT_DIR="${1:?usage: verify-linux-packages.sh OUTPUT_DIR}"
EXPECTED_APP_ID="${WIZRUST101_CI_EXPECTED_APP_ID:?expected release Discord App ID is required}"
APP_NAME=WizRust101-RPC
APP_ID=io.github.msork.WizRust101RPC
FLATPAK_ZIP="${OUTPUT_DIR}/WizRust101-RPC-Linux-Flatpak.zip"
APPIMAGE_ZIP="${OUTPUT_DIR}/WizRust101-RPC-Linux-AppImage.zip"
FLATPAK_DIR="${OUTPUT_DIR}/flatpak-archive"
APPIMAGE_DIR="${OUTPUT_DIR}/appimage-archive"

verify_single_file_zip() {
  local archive="$1" expected="$2"
  [[ -s "${archive}" ]] || { echo "Missing archive: ${archive}" >&2; exit 1; }
  mapfile -t entries < <(unzip -Z1 "${archive}")
  [[ "${#entries[@]}" -eq 1 && "${entries[0]}" == "${expected}" ]] \
    || { echo "${archive} must contain exactly ${expected} at its root." >&2; exit 1; }
  unzip -tq "${archive}" >/dev/null
}

verify_single_file_zip "${FLATPAK_ZIP}" "${APP_NAME}-linux.flatpak"
verify_single_file_zip "${APPIMAGE_ZIP}" "${APP_NAME}-linux.AppImage"
rm -rf "${FLATPAK_DIR}" "${APPIMAGE_DIR}"
mkdir -p "${FLATPAK_DIR}" "${APPIMAGE_DIR}"
unzip -q "${FLATPAK_ZIP}" -d "${FLATPAK_DIR}"
unzip -q "${APPIMAGE_ZIP}" -d "${APPIMAGE_DIR}"
FLATPAK="${FLATPAK_DIR}/${APP_NAME}-linux.flatpak"
APPIMAGE="${APPIMAGE_DIR}/${APP_NAME}-linux.AppImage"
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
echo "Validated both final Linux ZIPs, executable AppImage, Flatpak install/metadata, and embedded ID."
