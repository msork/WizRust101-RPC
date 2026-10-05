param(
    [Parameter(Mandatory = $true)]
    [string]$ExecutablePath
)

$ErrorActionPreference = "Stop"
$exe = (Resolve-Path $ExecutablePath).Path
$mtCommand = Get-Command "mt.exe" -ErrorAction SilentlyContinue
if ($mtCommand) {
    $mt = $mtCommand.Source
} else {
    $sdkBin = Join-Path ${env:ProgramFiles(x86)} "Windows Kits\10\bin"
    $mt = Get-ChildItem -Path $sdkBin -Filter "mt.exe" -Recurse -ErrorAction SilentlyContinue |
        Where-Object { $_.Directory.Name -eq "x64" } |
        Sort-Object { [version]$_.Directory.Parent.Name } -Descending |
        Select-Object -First 1 -ExpandProperty FullName
}
if (-not $mt) { throw "Windows SDK mt.exe is required to inspect the executable manifest." }

$temporaryDirectory = if ($env:RUNNER_TEMP) { $env:RUNNER_TEMP } else { $env:TEMP }
$manifestPath = Join-Path $temporaryDirectory "wizrust101-rpc.manifest"
& $mt "-nologo" "-inputresource:$exe;#1" "-out:$manifestPath"
if ($LASTEXITCODE -ne 0) { throw "Could not extract RT_MANIFEST resource #1 from $exe" }

[xml]$manifest = Get-Content -LiteralPath $manifestPath -Raw
$namespaces = [System.Xml.XmlNamespaceManager]::new($manifest.NameTable)
$namespaces.AddNamespace("asm", "urn:schemas-microsoft-com:asm.v1")
$dependency = $manifest.SelectSingleNode(
    "//asm:dependency/asm:dependentAssembly/asm:assemblyIdentity[@name='Microsoft.Windows.Common-Controls']",
    $namespaces
)
if (-not $dependency -or $dependency.GetAttribute("version") -ne "6.0.0.0") {
    throw "The EXE manifest does not activate Microsoft.Windows.Common-Controls version 6.0.0.0."
}
Write-Host "Verified embedded Common Controls v6 manifest in $exe"

# The portable distribution is just this executable, so its app icon must be
# a PE icon resource rather than an installer-only ICO file.
Add-Type -AssemblyName System.Drawing
$iconPath = Join-Path (Split-Path $PSScriptRoot -Parent) "packaging\windows\wizrust101-rpc.ico"
$embeddedIcon = [System.Drawing.Icon]::ExtractAssociatedIcon($exe)
if (-not $embeddedIcon) { throw "No application icon resource was found in $exe" }
$sourceIcon = [System.Drawing.Icon]::new($iconPath, $embeddedIcon.Size)
$embeddedBitmap = $embeddedIcon.ToBitmap()
$sourceBitmap = $sourceIcon.ToBitmap()
try {
    if ($embeddedBitmap.Size -ne $sourceBitmap.Size) {
        throw "Embedded executable icon dimensions do not match the project icon."
    }
    for ($y = 0; $y -lt $embeddedBitmap.Height; $y++) {
        for ($x = 0; $x -lt $embeddedBitmap.Width; $x++) {
            if ($embeddedBitmap.GetPixel($x, $y).ToArgb() -ne $sourceBitmap.GetPixel($x, $y).ToArgb()) {
                throw "Embedded executable icon pixels do not match the project icon."
            }
        }
    }
} finally {
    $embeddedBitmap.Dispose()
    $sourceBitmap.Dispose()
    $embeddedIcon.Dispose()
    $sourceIcon.Dispose()
}
Write-Host "Verified project application icon resource in $exe"

# Process creation resolves the EXE's static imports before entering main. This
# flag exits before the tray/watcher starts, keeping the probe side-effect free.
$process = Start-Process -FilePath $exe -ArgumentList "--ci-load-check" -PassThru
if (-not $process.WaitForExit(15000)) {
    $process.Kill()
    throw "Windows loader startup check timed out after 15 seconds."
}
if ($process.ExitCode -ne 0) {
    throw "Windows loader startup check failed with exit code $($process.ExitCode)."
}
Write-Host "Verified Windows loader startup for $exe"
