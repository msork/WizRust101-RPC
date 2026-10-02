#ifndef ReleaseDir
  #error ReleaseDir must point to the Windows release build directory
#endif

#define AppName "WizRust101-RPC"
#define AppVersion "0.1.0"
#define AppExe "wizrust101-rpc.exe"

[Setup]
AppId={{12D29F8F-CF84-4A7D-916D-2BFE97578112}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher=WizRust101-RPC Project
DefaultDirName={localappdata}\Programs\WizRust101-RPC
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
SetupIconFile=wizrust101-rpc.ico
UninstallDisplayIcon={app}\wizrust101-rpc.ico
OutputDir=../../target/windows-installer
OutputBaseFilename=WizRust101-RPC-Setup-{#AppVersion}-x64
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
CloseApplications=yes
RestartApplications=no

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
Source: "{#ReleaseDir}\{#AppExe}"; DestDir: "{app}"; Flags: ignoreversion
Source: "wizrust101-rpc.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\WizRust101-RPC"; Filename: "{app}\{#AppExe}"; WorkingDir: "{app}"; IconFilename: "{app}\wizrust101-rpc.ico"
Name: "{autoprograms}\Uninstall WizRust101-RPC"; Filename: "{uninstallexe}"

[UninstallDelete]
Type: files; Name: "{app}\wizrust101-rpc.ico"
