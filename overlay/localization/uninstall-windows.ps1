[CmdletBinding(SupportsShouldProcess = $true, ConfirmImpact = "High")]
param(
    [string]$InstallDir,
    [switch]$RemoveFromPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($env:LOCALAPPDATA)) {
    throw "LOCALAPPDATA is unavailable. Pass an explicit -InstallDir."
}
if ([string]::IsNullOrWhiteSpace($InstallDir)) {
    $InstallDir = Join-Path $env:LOCALAPPDATA "Programs\codex-cli-zh-cn\bin"
}
$InstallDir = [System.IO.Path]::GetFullPath([Environment]::ExpandEnvironmentVariables($InstallDir))
$root = [System.IO.Path]::GetPathRoot($InstallDir)
if ([string]::Equals($InstallDir.TrimEnd('\'), $root.TrimEnd('\'), [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "Refusing to remove a drive root."
}

if (Test-Path -LiteralPath $InstallDir -PathType Container) {
    if ($PSCmdlet.ShouldProcess($InstallDir, "Remove only the codex-zh install directory")) {
        Remove-Item -LiteralPath $InstallDir -Recurse -Force
    }
}
else {
    Write-Host "Install directory is already absent: $InstallDir"
}

if ($RemoveFromPath) {
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $kept = @($userPath -split ";" | Where-Object {
        -not [string]::IsNullOrWhiteSpace($_) -and
        -not [string]::Equals($_.TrimEnd('\'), $InstallDir.TrimEnd('\'), [System.StringComparison]::OrdinalIgnoreCase)
    })
    if ($PSCmdlet.ShouldProcess("User PATH", "Remove $InstallDir")) {
        [Environment]::SetEnvironmentVariable("Path", ($kept -join ";"), "User")
    }
}

Write-Host "Official codex and all CODEX_HOME directories were left unchanged."
