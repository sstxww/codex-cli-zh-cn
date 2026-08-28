[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Assert-True {
    param([bool]$Condition, [string]$Message)
    if (-not $Condition) { throw "Assertion failed: $Message" }
}

$repoRoot = Split-Path -Parent $PSScriptRoot
$installer = Join-Path $PSScriptRoot "install-windows.ps1"
$packager = Join-Path $PSScriptRoot "package-windows.ps1"
$temporaryRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("codex-zh-installer-test-" + [Guid]::NewGuid().ToString("N"))

try {
    New-Item -ItemType Directory -Path $temporaryRoot -Force | Out-Null
    $fakeBinary = Join-Path $temporaryRoot "fake-codex-zh.exe"
    "test binary content" | Set-Content -LiteralPath $fakeBinary -Encoding ASCII
    $assets = Join-Path $temporaryRoot "assets"
    & $packager -BinaryPath $fakeBinary -OutputDirectory $assets

    $archive = Join-Path $assets "codex-cli-zh-cn-windows-x64.zip"
    Assert-True (Test-Path -LiteralPath $archive) "packager creates archive"

    $installDir = Join-Path $temporaryRoot "installed\bin"
    & $installer -ArchivePath $archive -InstallDir $installDir
    Assert-True (Test-Path -LiteralPath (Join-Path $installDir "codex-zh.exe")) "installer copies executable"
    Assert-True (Test-Path -LiteralPath (Join-Path $installDir "codex-zh.cmd")) "installer copies launcher"
    Assert-True (-not (Test-Path -LiteralPath (Join-Path $installDir "codex.exe"))) "installer never replaces official command"
    Assert-True (-not (Test-Path -LiteralPath (Join-Path $installDir "auth.json"))) "installer never copies authentication"

    Write-Host "All codex-cli-zh-cn packaging and installer tests passed."
}
finally {
    if (Test-Path -LiteralPath $temporaryRoot) {
        Remove-Item -LiteralPath $temporaryRoot -Recurse -Force
    }
}
