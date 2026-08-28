[CmdletBinding(SupportsShouldProcess = $true)]
param(
    [string]$InstallDir,
    [string]$ArchivePath,
    [string]$ReleaseTag = "latest",
    [switch]$AddToPath
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

$assetName = "codex-cli-zh-cn-windows-x64.zip"
$temporaryRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("codex-zh-install-" + [Guid]::NewGuid().ToString("N"))

try {
    New-Item -ItemType Directory -Path $temporaryRoot -Force | Out-Null
    if ([string]::IsNullOrWhiteSpace($ArchivePath)) {
        $baseUrl = if ($ReleaseTag -eq "latest") {
            "https://github.com/sstxww/codex-cli-zh-cn/releases/latest/download"
        }
        else {
            "https://github.com/sstxww/codex-cli-zh-cn/releases/download/$ReleaseTag"
        }
        $ArchivePath = Join-Path $temporaryRoot $assetName
        if ($PSCmdlet.ShouldProcess("$baseUrl/$assetName", "Download localized Codex archive")) {
            Invoke-WebRequest -UseBasicParsing -Uri "$baseUrl/$assetName" -OutFile $ArchivePath
        }
        if (-not (Test-Path -LiteralPath $ArchivePath -PathType Leaf)) {
            throw "Release archive was not downloaded."
        }
    }
    else {
        $ArchivePath = [System.IO.Path]::GetFullPath([Environment]::ExpandEnvironmentVariables($ArchivePath))
        if (-not (Test-Path -LiteralPath $ArchivePath -PathType Leaf)) {
            throw "Local archive was not found: $ArchivePath"
        }
    }

    $extractDir = Join-Path $temporaryRoot "extracted"
    Expand-Archive -LiteralPath $ArchivePath -DestinationPath $extractDir -Force
    $executable = Join-Path $extractDir "codex-zh.exe"
    $launcher = Join-Path $extractDir "codex-zh.cmd"
    foreach ($required in @($executable, $launcher)) {
        if (-not (Test-Path -LiteralPath $required -PathType Leaf)) {
            throw "Archive is incomplete: missing $required"
        }
    }

    if ($PSCmdlet.ShouldProcess($InstallDir, "Install codex-zh without replacing official codex")) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        Copy-Item -LiteralPath $executable -Destination (Join-Path $InstallDir "codex-zh.exe") -Force
        Copy-Item -LiteralPath $launcher -Destination (Join-Path $InstallDir "codex-zh.cmd") -Force
    }

    if ($AddToPath) {
        $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
        $entries = @($userPath -split ";" | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
        $present = $false
        foreach ($entry in $entries) {
            if ([string]::Equals($entry.TrimEnd('\'), $InstallDir.TrimEnd('\'), [System.StringComparison]::OrdinalIgnoreCase)) {
                $present = $true
                break
            }
        }
        if (-not $present -and $PSCmdlet.ShouldProcess("User PATH", "Append $InstallDir")) {
            $newPath = if ([string]::IsNullOrWhiteSpace($userPath)) { $InstallDir } else { "$userPath;$InstallDir" }
            [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
            Write-Host "Added to user PATH. Open a new terminal before using codex-zh."
        }
        elseif ($present) {
            Write-Host "User PATH already contains: $InstallDir"
        }
    }

    Write-Host "Installed codex-zh: $InstallDir"
    Write-Host "Official codex and all CODEX_HOME directories were left unchanged."
}
finally {
    if (Test-Path -LiteralPath $temporaryRoot) {
        Remove-Item -LiteralPath $temporaryRoot -Recurse -Force
    }
}
