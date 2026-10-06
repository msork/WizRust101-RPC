![WizRust101-RPC icon](assets/icons/sizes/128.png)

# WizRust101-RPC

A tray and menu-bar app that detects Wizard101 locations from Steam logs and shares them through Discord Rich Presence. Wizard101 support is Steam-only; the app does not launch or modify the game.

## Usage

1. Download the ZIP for your installation option from the [latest release](https://github.com/msork/WizRust101-RPC/releases/latest) and extract it.

2. Launch the app:

   - **Windows Setup:** run `WizRust101-RPC-Windows-Setup.exe` and open the app from the Start Menu.
   - **Windows Portable:** run `WizRust101-RPC-Windows-app.exe` from the extracted folder.
   - **Linux Flatpak:** install `WizRust101-RPC-linux.flatpak` with your software manager, or run `flatpak install --user ./WizRust101-RPC-linux.flatpak`, then launch with `flatpak run io.github.msork.WizRust101RPC`.
   - **Linux AppImage:** run `chmod +x WizRust101-RPC-linux.AppImage` once, then launch `./WizRust101-RPC-linux.AppImage`.
   - **macOS App:** open `WizRust101-RPC.app`.
   - **macOS Installer:** open `WizRust101-RPC-macOS.pkg`, complete installation, then launch the app from Applications.

   macOS builds are unsigned previews and have not been tested on a Mac.

3. Keep Discord running and launch Wizard101 through Steam. WizRust101-RPC appears in the system tray or menu bar and updates your Discord Rich Presence as you play.

## Downloads

Each release includes six separate ZIPs for these installation options.

| Download | Contains |
|---|---|
| `WizRust101-RPC-Windows-Setup.zip` | `WizRust101-RPC-Windows-Setup.exe` |
| `WizRust101-RPC-Windows-Portable.zip` | `WizRust101-RPC-Windows-app.exe` |
| `WizRust101-RPC-Linux-Flatpak.zip` | `WizRust101-RPC-linux.flatpak` |
| `WizRust101-RPC-Linux-AppImage.zip` | `WizRust101-RPC-linux.AppImage` |
| `WizRust101-RPC-macOS-App.zip` | `WizRust101-RPC.app` |
| `WizRust101-RPC-macOS-Pkg.zip` | `WizRust101-RPC-macOS.pkg` |

Windows 11 x64 has passed live acceptance with native Steam and Discord. Linux packages are CI-validated; current builds have not received live testing. macOS 15 packages are unsigned, unnotarized previews and are not production-supported.

## Rich Presence

Discord displays the location from the pinned [WizRust101-DB](https://github.com/msork/WizRust101-DB), the known world when available, and matching or generic Wizard101 artwork. The session timer continues across zone changes. Official builds include the project Discord Application ID; users do not need to configure one.

## License

WizRust101-RPC is licensed under the [MIT License](LICENSE).
