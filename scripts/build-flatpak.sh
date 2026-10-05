#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${WIZRUST101_RELEASE_DISCORD_APP_ID:-}" || ! "${WIZRUST101_RELEASE_DISCORD_APP_ID}" =~ ^[0-9]+$ ]]; then
  echo "Set WIZRUST101_RELEASE_DISCORD_APP_ID to the project Discord Application ID for this release build." >&2
  exit 2
fi
if ! command -v flatpak-builder >/dev/null 2>&1; then
  echo "flatpak-builder is required to build and install the Flatpak." >&2
  exit 2
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
build_root="${WIZRUST101_FLATPAK_BUILD_DIR:-${repo_root}/target/flatpak}"
build_dir="${build_root}/build"
repo_dir="${build_root}/repo"
bundle_path="${WIZRUST101_FLATPAK_BUNDLE_PATH:-}"
mkdir -p "${build_root}"
staging_dir="$(mktemp -d "${build_root}/source.XXXXXX")"
trap 'rm -rf "${staging_dir}"' EXIT

cp -R "${repo_root}/src" "${repo_root}/data" "${repo_root}/build.rs" "${staging_dir}/"
cp "${repo_root}/Cargo.toml" "${repo_root}/Cargo.lock" "${staging_dir}/"
mkdir -p "${staging_dir}/packaging/flatpak"
cp "${repo_root}/packaging/flatpak/cargo-sources.json" \
  "${repo_root}/packaging/flatpak/io.github.msork.WizRust101RPC.desktop" \
  "${repo_root}/packaging/flatpak/io.github.msork.WizRust101RPC.metainfo.xml" \
  "${repo_root}/packaging/flatpak/icon.svg" \
  "${staging_dir}/packaging/flatpak/"
sed "s/__WIZRUST101_RELEASE_DISCORD_APP_ID__/${WIZRUST101_RELEASE_DISCORD_APP_ID}/g" \
  "${repo_root}/packaging/flatpak/io.github.msork.WizRust101RPC.yml" \
  > "${staging_dir}/packaging/flatpak/io.github.msork.WizRust101RPC.yml"

# rofiles-fuse is not available in all build containers; Flatpak's regular
# staging mode is adequate for this small package and keeps the build portable.
if [[ -n "${bundle_path}" ]]; then
  flatpak-builder --disable-rofiles-fuse --force-clean \
    --repo="${repo_dir}" \
    "${build_dir}" \
    "${staging_dir}/packaging/flatpak/io.github.msork.WizRust101RPC.yml"
  flatpak build-bundle "${repo_dir}" "${bundle_path}" io.github.msork.WizRust101RPC stable \
    --runtime-repo=https://dl.flathub.org/repo/flathub.flatpakrepo
else
  flatpak-builder --disable-rofiles-fuse --user --install --force-clean \
    --repo="${repo_dir}" \
    "${build_dir}" \
    "${staging_dir}/packaging/flatpak/io.github.msork.WizRust101RPC.yml"
fi
