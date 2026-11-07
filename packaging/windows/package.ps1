param(
    [string]$Arch = "x64",
    [string]$Version = "",
    [string]$AppName = "",
    [string]$AppTitle = "",
    [string]$Manufacturer = "",
    [string]$UpgradeCode = "A1B2C3D4-E5F6-7890-ABCD-EF1234567890"
)

$ErrorActionPreference = "Stop"

if (-not $AppName -or -not $Version) {
    if (Test-Path "Cargo.toml") {
        $content = Get-Content "Cargo.toml" -Raw
        if (-not $AppName -and $content -match 'name\s*=\s*"([^"]+)"') {
            $AppName = $matches[1]
        }
        if (-not $Version -and $content -match 'version\s*=\s*"([^"]+)"') {
            $Version = $matches[1]
        }
    }
}

if (-not $AppTitle) {
    $AppTitle = $AppName
}
if (-not $Manufacturer) {
    $Manufacturer = $AppName
}

$TargetMap = @{
    "x64"   = "x86_64-pc-windows-msvc"
    "arm64" = "aarch64-pc-windows-msvc"
    "x86"   = "i686-pc-windows-msvc"
}

$Target = $TargetMap[$Arch]
if (-not $Target) {
    throw "Unsupported architecture: $Arch. Use x64, arm64, or x86."
}

Write-Host "Building $AppName v$Version for $Arch ($Target)..." -ForegroundColor Cyan
cargo build --release --target $Target

$ExePath = "target/$Target/release/$AppName.exe"
if (-not (Test-Path $ExePath)) {
    throw "Executable not found at $ExePath"
}

$DistDir = "dist/release"
New-Item -ItemType Directory -Force -Path $DistDir | Out-Null

$MsiOutput = "$DistDir/$AppName-$Version-windows-$Arch.msi"

$IconPath = "packaging/windows/app.ico"
if (-not (Test-Path $IconPath)) {
    # If no custom icon exists, use dummy or pass executable
    $IconPath = $ExePath
}

Write-Host "Compiling MSI using WiX v5..." -ForegroundColor Cyan
wix build `
    -arch $Arch `
    -d AppName="$AppName" `
    -d AppTitle="$AppTitle" `
    -d Manufacturer="$Manufacturer" `
    -d Version="$Version" `
    -d UpgradeCode="$UpgradeCode" `
    -d SourceExe="$ExePath" `
    -d IconPath="$IconPath" `
    -d HelpUrl="https://github.com/$Manufacturer/$AppName" `
    -out $MsiOutput `
    packaging/windows/main.wxs

Write-Host "Successfully generated MSI: $MsiOutput" -ForegroundColor Green
