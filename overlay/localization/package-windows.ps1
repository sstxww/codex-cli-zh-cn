[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$BinaryPath,

    [Parameter(Mandatory = $true)]
    [string]$OutputDirectory
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$BinaryPath = [System.IO.Path]::GetFullPath($BinaryPath)
$OutputDirectory = [System.IO.Path]::GetFullPath($OutputDirectory)
if (-not (Test-Path -LiteralPath $BinaryPath -PathType Leaf)) {
    throw "codex-zh binary was not found: $BinaryPath"
}

$assetName = "codex-cli-zh-cn-windows-x64.zip"
$temporaryRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("codex-zh-package-" + [Guid]::NewGuid().ToString("N"))
try {
    New-Item -ItemType Directory -Path $temporaryRoot -Force | Out-Null
    New-Item -ItemType Directory -Path $OutputDirectory -Force | Out-Null
    Copy-Item -LiteralPath $BinaryPath -Destination (Join-Path $temporaryRoot "codex-zh.exe")
    @(
        "@echo off",
        "setlocal",
        '"%~dp0codex-zh.exe" %*',
        "exit /b %ERRORLEVEL%"
    ) | Set-Content -LiteralPath (Join-Path $temporaryRoot "codex-zh.cmd") -Encoding ASCII
    @(
        "Unofficial Simplified Chinese interactive TUI for OpenAI Codex.",
        "Command: codex-zh",
        "Upstream version: 0.150.0-alpha.8",
        "Project: https://github.com/sstxww/codex-cli-zh-cn",
        "Keep the official codex command installed for rollback and non-interactive/admin commands."
    ) | Set-Content -LiteralPath (Join-Path $temporaryRoot "README.txt") -Encoding UTF8
    $archivePath = Join-Path $OutputDirectory $assetName
    if (Test-Path -LiteralPath $archivePath) {
        Remove-Item -LiteralPath $archivePath -Force
    }
    Compress-Archive -Path (Join-Path $temporaryRoot "*") -DestinationPath $archivePath -CompressionLevel Optimal

    Write-Host "Archive: $archivePath"
}
finally {
    if (Test-Path -LiteralPath $temporaryRoot) {
        Remove-Item -LiteralPath $temporaryRoot -Recurse -Force
    }
}
