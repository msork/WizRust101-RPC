#!/usr/bin/env bash
set -euo pipefail

DIR="${1:-release-candidate}"
declare -A EXPECTED=(
  [WizRust101-RPC-Windows-Setup.zip]=WizRust101-RPC-Windows-Setup.exe
  [WizRust101-RPC-Windows-Portable.zip]=WizRust101-RPC-Windows-app.exe
  [WizRust101-RPC-macOS-App.zip]=WizRust101-RPC.app
  [WizRust101-RPC-macOS-Pkg.zip]=WizRust101-RPC-macOS.pkg
  [WizRust101-RPC-Linux-Flatpak.zip]=WizRust101-RPC-linux.flatpak
  [WizRust101-RPC-Linux-AppImage.zip]=WizRust101-RPC-linux.AppImage
)
for archive in "${!EXPECTED[@]}"; do
  path="${DIR}/${archive}"
  [[ -s "${path}" ]] || { echo "Missing release archive ${path}." >&2; exit 1; }
  unzip -tq "${path}" >/dev/null
  mapfile -t entries < <(unzip -Z1 "${path}" | sed '/\/$/d' | grep -v '^__MACOSX/' | grep -v '/._' | sort)
  expected="${EXPECTED[${archive}]}"
  case "${archive}" in
    WizRust101-RPC-macOS-App.zip)
      grep -Fxq 'WizRust101-RPC.app/Contents/Info.plist' <<<"$(unzip -Z1 "${path}")" || { echo "${archive} lacks the app bundle Info.plist." >&2; exit 1; }
      grep -Fxq 'WizRust101-RPC.app/Contents/MacOS/wizrust101-rpc' <<<"$(unzip -Z1 "${path}")" || { echo "${archive} lacks the app executable." >&2; exit 1; }
      ! grep -Eiq '\.(zip|pkg|dmg)$' <<<"$(unzip -Z1 "${path}")" || { echo "${archive} contains an unexpected nested archive." >&2; exit 1; }
      ;;
    *)
      [[ "${#entries[@]}" -eq 1 && "${entries[0]}" == "${expected}" ]] || {
        echo "${archive} must contain exactly ${expected}; found: ${entries[*]:-nothing}." >&2; exit 1;
      }
      ;;
  esac
done
echo "Validated all six release ZIPs and their expected payloads."
