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

    $outputDir = Join-Path $repoRoot "target\windows-installer"
    foreach ($obsoleteArchive in @(
        "WizRust101-RPC-Windows-Setup.exe.zip",
        "WizRust101-RPC-Windows-app.exe.zip"
    )) {
        Remove-Item -LiteralPath (Join-Path $outputDir $obsoleteArchive) -Force -ErrorAction SilentlyContinue
    }
    $portableName = "WizRust101-RPC-Windows-app.exe"
    $portableDir = Join-Path $outputDir "portable-stage"
    $portableExe = Join-Path $portableDir $portableName
    New-Item -ItemType Directory -Force -Path $portableDir | Out-Null
    Copy-Item -LiteralPath (Join-Path $releaseDir "wizrust101-rpc.exe") -Destination $portableExe -Force
    & (Join-Path $PSScriptRoot "verify-windows-exe.ps1") -ExecutablePath $portableExe
    if ($LASTEXITCODE -ne 0) { throw "Portable executable validation failed with exit code $LASTEXITCODE" }
    Push-Location $portableDir
    try {
        & ".\$portableName" --ci-load-check
        if ($LASTEXITCODE -ne 0) { throw "Portable executable startup check failed with exit code $LASTEXITCODE" }
    } finally {
        Pop-Location
    }

    $setupExe = Get-ChildItem -LiteralPath $outputDir -Filter "WizRust101-RPC-Setup-*-x64.exe" |
        Sort-Object LastWriteTime -Descending | Select-Object -First 1
    if (-not $setupExe) { throw "Inno Setup did not produce the expected setup executable." }
    $setupStage = Join-Path $outputDir "setup-stage"
    New-Item -ItemType Directory -Force -Path $setupStage | Out-Null
    $setupName = "WizRust101-RPC-Windows-Setup.exe"
    $normalizedSetup = Join-Path $setupStage $setupName
    Copy-Item -LiteralPath $setupExe.FullName -Destination $normalizedSetup -Force

    # GitHub Actions packages these two executables into one downloadable ZIP.
    # Keep the portable app standalone and place both choices at the archive root.
    $releaseStage = Join-Path $outputDir "release-stage"
    New-Item -ItemType Directory -Force -Path $releaseStage | Out-Null
    Copy-Item -LiteralPath $portableExe -Destination (Join-Path $releaseStage $portableName) -Force
    Copy-Item -LiteralPath $normalizedSetup -Destination (Join-Path $releaseStage $setupName) -Force
    $releaseEntries = @(Get-ChildItem -LiteralPath $releaseStage -File -Recurse)
    if ($releaseEntries.Count -ne 2 -or
        ($releaseEntries.Name -notcontains $portableName) -or
        ($releaseEntries.Name -notcontains $setupName)) {
        throw "Windows release artifact must contain only the setup and portable executables."
    }
    & (Join-Path $PSScriptRoot "verify-windows-exe.ps1") -ExecutablePath (Join-Path $releaseStage $portableName)
    if ($LASTEXITCODE -ne 0) { throw "Portable executable validation failed with exit code $LASTEXITCODE" }
    Push-Location $releaseStage
    try {
        & ".\$portableName" --ci-load-check
        if ($LASTEXITCODE -ne 0) { throw "Portable executable load check failed with exit code $LASTEXITCODE" }
    } finally {
        Pop-Location
    }
} finally {
    Pop-Location
}
