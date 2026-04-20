#Requires -Version 5.0
<#
.SYNOPSIS
    Install sc (soundcharts-cli) on Windows.
.DESCRIPTION
    Downloads the latest release binary for the detected architecture,
    extracts it to %LOCALAPPDATA%\Programs\sc, and prints a PATH hint.
.EXAMPLE
    irm https://raw.githubusercontent.com/oneortwo/soundcharts-cli/main/install.ps1 | iex
#>

$ErrorActionPreference = 'Stop'

$repo = 'oneortwo/soundcharts-cli'
$binary = 'sc.exe'

$arch = if ($env:PROCESSOR_ARCHITECTURE -eq 'ARM64') { 'aarch64' } else { 'x86_64' }
$target = "$arch-pc-windows-msvc"

Write-Host "Detecting latest release..."
$release = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest" -UseBasicParsing
$tag = $release.tag_name
if (-not $tag) {
    Write-Error "Could not determine latest release"
    exit 1
}

$assetName = "sc-$target.zip"
$asset = $release.assets | Where-Object { $_.name -eq $assetName } | Select-Object -First 1
if (-not $asset) {
    Write-Error "No asset named $assetName in release $tag"
    exit 1
}

$installDir = Join-Path $env:LOCALAPPDATA 'Programs\sc'
New-Item -ItemType Directory -Force -Path $installDir | Out-Null

$tmpZip = Join-Path $env:TEMP "sc-$tag.zip"
Write-Host "Downloading sc $tag for $target..."
Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $tmpZip -UseBasicParsing

Write-Host "Extracting to $installDir..."
Expand-Archive -Path $tmpZip -DestinationPath $installDir -Force
Remove-Item $tmpZip

$exePath = Join-Path $installDir $binary
Write-Host "Installed sc to $exePath"

$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
if ($userPath -notlike "*$installDir*") {
    Write-Host ""
    Write-Host "Add $installDir to your PATH to use 'sc' globally. Run:"
    Write-Host "  [Environment]::SetEnvironmentVariable('Path', `"`$env:Path;$installDir`", 'User')"
    Write-Host "Then open a new terminal."
}
