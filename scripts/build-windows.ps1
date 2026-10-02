param(
    [string]$InnoCompiler = "ISCC.exe"
)

$ErrorActionPreference = "Stop"
$repoRoot = Split-Path -Parent $PSScriptRoot

if (-not $env:WIZRUST101_RELEASE_DISCORD_APP_ID -or $env:WIZRUST101_RELEASE_DISCORD_APP_ID -notmatch '^\d+$') {
    throw "Set WIZRUST101_RELEASE_DISCORD_APP_ID to the project Discord Application ID for official builds."
}

Push-Location $repoRoot
try {
    rustup target add x86_64-pc-windows-msvc
    if ($LASTEXITCODE -ne 0) { throw "Installing the Windows Rust target failed with exit code $LASTEXITCODE" }
    cargo build --release --target x86_64-pc-windows-msvc --bin wizrust101-rpc
    if ($LASTEXITCODE -ne 0) { throw "Windows release build failed with exit code $LASTEXITCODE" }

    if ($InnoCompiler -eq "ISCC.exe") {
        $candidates = @(
            (Join-Path ${env:ProgramFiles(x86)} "Inno Setup 6\ISCC.exe"),
            (Join-Path $env:ProgramFiles "Inno Setup 6\ISCC.exe")
        )
        $InnoCompiler = $candidates | Where-Object { Test-Path $_ } | Select-Object -First 1
        if (-not $InnoCompiler) {
            $command = Get-Command "ISCC.exe" -ErrorAction SilentlyContinue
            if ($command) { $InnoCompiler = $command.Source }
        }
        if (-not $InnoCompiler) { throw "Install Inno Setup 6.7.3 or pass -InnoCompiler with its ISCC.exe path." }
    }

    $releaseDir = Join-Path $repoRoot "target\x86_64-pc-windows-msvc\release"
    & (Join-Path $PSScriptRoot "verify-windows-exe.ps1") -ExecutablePath (Join-Path $releaseDir "wizrust101-rpc.exe")
    if ($LASTEXITCODE -ne 0) { throw "Windows executable validation failed with exit code $LASTEXITCODE" }
    & $InnoCompiler "/DReleaseDir=$releaseDir" "packaging\windows\wizrust101-rpc.iss"
    if ($LASTEXITCODE -ne 0) { throw "Inno Setup failed with exit code $LASTEXITCODE" }
} finally {
    Pop-Location
}
