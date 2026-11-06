param([Parameter(Mandatory=$true)][string]$OutputFile)
$ErrorActionPreference = 'Stop'
$taskRoot = Split-Path $PSScriptRoot -Parent
$taskExe = Join-Path $taskRoot 'target\release\examples\idle_session.exe'
$taskOutput = [IO.Path]::GetFullPath($OutputFile)
if (Test-Path -LiteralPath $taskOutput) { throw 'Use a new output filename.' }
New-Item -ItemType Directory -Force -Path (Split-Path $taskOutput -Parent) | Out-Null
$taskProcess = Start-Process -FilePath $taskExe -PassThru -WindowStyle Hidden -RedirectStandardOutput "$taskOutput.stdout.log" -RedirectStandardError "$taskOutput.stderr.log"
$taskSamples = [Collections.Generic.List[long]]::new()
while (-not $taskProcess.HasExited) {
    $taskProcess.Refresh()
    if ($taskProcess.WorkingSet64 -gt 0) { $taskSamples.Add($taskProcess.WorkingSet64) }
    Start-Sleep -Milliseconds 100
}
$taskProcess.WaitForExit()
if ($taskProcess.ExitCode -ne 0 -or $taskSamples.Count -lt 10) { throw 'Memory probe failed or too few samples.' }
$taskSorted = @($taskSamples | Sort-Object)
[ordered]@{
    scope = 'One Rust Session; four Tokio workers; no HTTP clients, browser or SDK'
    samples = $taskSamples.Count
    median_working_set_bytes = $taskSorted[[int][Math]::Floor($taskSorted.Count / 2)]
    max_sampled_working_set_bytes = $taskSorted[-1]
    executable_sha256 = (Get-FileHash -LiteralPath $taskExe -Algorithm SHA256).Hash
    source_revision = (git -C $taskRoot rev-parse HEAD)
    working_tree_modified = [bool](git -C $taskRoot status --porcelain)
    os = [Environment]::OSVersion.VersionString
} | ConvertTo-Json | Set-Content -LiteralPath $taskOutput
Get-Content -LiteralPath $taskOutput
