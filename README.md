![WizRust101-RPC icon](assets/icons/sizes/128.png)

# WizRust101-RPC

A tray and menu-bar app that detects Wizard101 locations from Steam logs and shares them through Discord Rich Presence. Wizard101 support is Steam-only; the app does not launch or modify the game.

## Usage

1. Download the appropriate ZIP for your platform from the [latest release](https://github.com/msork/WizRust101-RPC/releases/latest).

2. Install and launch **WizRust101-RPC**:

   - **Windows Setup:**
     1. Extract the ZIP file by right-clicking it in File Explorer and selecting **Extract All**.
     2. Run `WizRust101-RPC-Windows-Setup.exe` and follow the on-screen instructions.
     3. Search for **WizRust101-RPC** in the Start Menu and open it.

   - **Windows Portable:**
     1. Extract the ZIP file by right-clicking it in File Explorer and selecting **Extract All**.
     2. *(Optional)* Move `WizRust101-RPC-Windows-app.exe` somewhere you can easily find it later.
     3. Run `WizRust101-RPC-Windows-app.exe` to start **WizRust101-RPC**.

   - **macOS PKG:** *Needs testers*
     1. Extract the ZIP file by double-clicking it in Finder.
     2. Open `WizRust101-RPC-macOS.pkg` and follow the on-screen instructions.
     3. Open **WizRust101-RPC** from your Applications folder.

   - **macOS APP:** *Needs testers*
     1. Extract the ZIP file by double-clicking it in Finder.
     2. *(Optional)* Move `WizRust101-RPC.app` somewhere you can easily find it later, such as `~/Applications`.
     3. Open `WizRust101-RPC.app` to start **WizRust101-RPC**.

   - **Linux Flatpak:**
     1. Extract the ZIP file:
        ```bash
        unzip WizRust101-RPC-Linux-Flatpak.zip
        ```
     2. Make sure Flatpak is installed, then install **WizRust101-RPC**:
        ```bash
        flatpak install ./WizRust101-RPC-linux.flatpak
        ```
     3. Start **WizRust101-RPC**:
        ```bash
        flatpak run io.github.msork.WizRust101RPC
        ```

   - **Linux AppImage:**
     1. Extract the ZIP file:
        ```bash
        unzip WizRust101-RPC-Linux-AppImage.zip
        ```
     2. *(Optional)* Move `WizRust101-RPC-linux.AppImage` somewhere you can easily find it later. For example:
        ```bash
        cp ./WizRust101-RPC-linux.AppImage ~/Desktop/WizRust101-RPC-linux.AppImage
        ```
     3. Make the AppImage executable and start **WizRust101-RPC**:
        ```bash
        chmod +x ./WizRust101-RPC-linux.AppImage
        ./WizRust101-RPC-linux.AppImage
        ```

3. You should now see the **WizRust101-RPC** icon in your system tray or menu bar: ![WizRust101-RPC tray icon](assets/icons/sizes/16.png)

4. Make sure the **Discord desktop app** is running.

5. Launch **Wizard101 through Steam**.

   > **Note:** WizRust101-RPC currently supports only the **Steam version of Wizard101**.

6. That's it! WizRust101-RPC will automatically detect your game and update your Discord Rich Presence as you play.

## Downloads

The planned first public version is **v26.10.05**, with six separate ZIPs for these installation options.

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
