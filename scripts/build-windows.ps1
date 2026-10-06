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
        "WizRust101-RPC-Windows-app.exe.zip",
        "WizRust101-RPC-Windows.zip",
        "WizRust101-RPC-Windows-Setup.zip",
        "WizRust101-RPC-Windows-Portable.zip"
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

    $setupZip = Join-Path $outputDir "WizRust101-RPC-Windows-Setup.zip"
    $portableZip = Join-Path $outputDir "WizRust101-RPC-Windows-Portable.zip"
    Compress-Archive -LiteralPath $normalizedSetup -DestinationPath $setupZip -CompressionLevel Optimal
    Compress-Archive -LiteralPath $portableExe -DestinationPath $portableZip -CompressionLevel Optimal

    $verifyBase = if ($env:RUNNER_TEMP) { $env:RUNNER_TEMP } else { [IO.Path]::GetTempPath() }
    $verifyBase = [IO.Path]::GetFullPath($verifyBase)
    $verifyPrefix = $verifyBase.TrimEnd([IO.Path]::DirectorySeparatorChar) + [IO.Path]::DirectorySeparatorChar
    $archives = @(
        @{ Zip = $setupZip; Entry = $setupName; Kind = "setup" },
        @{ Zip = $portableZip; Entry = $portableName; Kind = "portable" }
    )
    foreach ($archive in $archives) {
        $verifyRoot = [IO.Path]::GetFullPath(
            (Join-Path $verifyBase ("wizrust101-windows-archive-" + [guid]::NewGuid().ToString("N")))
        )
        if (-not $verifyRoot.StartsWith($verifyPrefix, [StringComparison]::OrdinalIgnoreCase)) {
            throw "Windows archive validation path escaped the runner temporary directory."
        }
        New-Item -ItemType Directory -Path $verifyRoot | Out-Null
        Expand-Archive -LiteralPath $archive.Zip -DestinationPath $verifyRoot
        $verifiedEntries = @(Get-ChildItem -LiteralPath $verifyRoot)
        if ($verifiedEntries.Count -ne 1 -or
            ($verifiedEntries | Where-Object { $_.PSIsContainer }).Count -ne 0 -or
            $verifiedEntries[0].Name -ne $archive.Entry) {
            throw "$([IO.Path]::GetFileName($archive.Zip)) must contain only $($archive.Entry) at its root."
        }
        $bytes = [IO.File]::ReadAllBytes($verifiedEntries[0].FullName)
        if ($bytes.Length -lt 2 -or $bytes[0] -ne 0x4D -or $bytes[1] -ne 0x5A) {
            throw "$($archive.Entry) is not a Windows executable."
        }
        if ($archive.Kind -eq "setup") { continue }
        $verifiedPortable = $verifiedEntries[0].FullName
    & (Join-Path $PSScriptRoot "verify-windows-exe.ps1") -ExecutablePath $verifiedPortable
    if ($LASTEXITCODE -ne 0) { throw "Portable executable validation failed with exit code $LASTEXITCODE" }
    Push-Location $verifyRoot
    try {
        & ".\$($archive.Entry)" --ci-load-check
        if ($LASTEXITCODE -ne 0) { throw "Portable ZIP executable load check failed with exit code $LASTEXITCODE" }
    } finally {
        Pop-Location
    }
    }
} finally {
    Pop-Location
}
