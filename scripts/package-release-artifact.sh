#!/usr/bin/env bash
set -euo pipefail

SOURCE_DIR="${1:?usage: package-release-artifact.sh SOURCE_DIR EXPECTED_PAYLOAD OUTPUT_ZIP}"
EXPECTED="${2:?usage: package-release-artifact.sh SOURCE_DIR EXPECTED_PAYLOAD OUTPUT_ZIP}"
OUTPUT_ZIP="${3:?usage: package-release-artifact.sh SOURCE_DIR EXPECTED_PAYLOAD OUTPUT_ZIP}"

diagnose_mismatch() {
  echo "Downloaded artifact payload mismatch; expected exactly '${EXPECTED}' in '${SOURCE_DIR}'. Extracted entries:" >&2
  if [[ -d "${SOURCE_DIR}" ]]; then
    find "${SOURCE_DIR}" -mindepth 1 -printf '  %y %P\n' | sort >&2 || true
  else
    echo "  source directory is missing" >&2
  fi
  exit 1
}

[[ -d "${SOURCE_DIR}" ]] || diagnose_mismatch
[[ -n "$(find "${SOURCE_DIR}" -mindepth 1 -print -quit)" ]] || diagnose_mismatch
mkdir -p "$(dirname "${OUTPUT_ZIP}")"
OUTPUT_ZIP="$(cd "$(dirname "${OUTPUT_ZIP}")" && pwd)/$(basename "${OUTPUT_ZIP}")"
rm -f "${OUTPUT_ZIP}"

if [[ "${EXPECTED}" == "WizRust101-RPC.app" ]]; then
  mapfile -t roots < <(find "${SOURCE_DIR}" -mindepth 1 -maxdepth 1 -printf '%f\n')
  APP="${SOURCE_DIR}/${EXPECTED}"
  [[ "${#roots[@]}" -eq 1 && "${roots[0]}" == "${EXPECTED}" && -d "${APP}" ]] || diagnose_mismatch
  [[ -s "${APP}/Contents/Info.plist" \
    && -f "${APP}/Contents/MacOS/wizrust101-rpc" \
    && -s "${APP}/Contents/Resources/AppIcon.icns" ]] || diagnose_mismatch
  chmod 755 "${APP}/Contents/MacOS/wizrust101-rpc"
  find "${APP}" -type d -exec chmod 755 {} +
  (cd "${SOURCE_DIR}" && zip -q -X -r "${OUTPUT_ZIP}" "${EXPECTED}")
else
  mapfile -t roots < <(find "${SOURCE_DIR}" -mindepth 1 -maxdepth 1 -printf '%f\n')
  PAYLOAD="${SOURCE_DIR}/${EXPECTED}"
  [[ "${#roots[@]}" -eq 1 && "${roots[0]}" == "${EXPECTED}" && -f "${PAYLOAD}" && ! -L "${PAYLOAD}" ]] || diagnose_mismatch
  zip -q -X -j "${OUTPUT_ZIP}" "${PAYLOAD}"
fi

echo "Repacked downloaded payload '${EXPECTED}' as '${OUTPUT_ZIP}'."
