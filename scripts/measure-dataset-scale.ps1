param(
    [ValidateRange(1, 10000000)][int]$Pages = 10000,
    [Parameter(Mandatory=$true)][string]$OutputDirectory,
    [ValidateSet('http','generated')][string]$Mode = 'http'
)
$ErrorActionPreference = 'Stop'
$taskRoot = Split-Path $PSScriptRoot -Parent
$taskExe = Join-Path $taskRoot 'target\release\examples\dataset_scale.exe'
$taskOutput = [IO.Path]::GetFullPath($OutputDirectory)
if (Test-Path -LiteralPath $taskOutput) { throw 'Use a new output directory for each acceptance run.' }
$taskParent = Split-Path $taskOutput -Parent
New-Item -ItemType Directory -Force -Path $taskParent | Out-Null
$taskTimer = [Diagnostics.Stopwatch]::StartNew()
$taskProcess = Start-Process -FilePath $taskExe -ArgumentList @($Pages, ('"' + $taskOutput + '"'), $Mode) -PassThru -WindowStyle Hidden -RedirectStandardOutput "$taskOutput.stdout.log" -RedirectStandardError "$taskOutput.stderr.log"
$taskPeak = 0L
while (-not $taskProcess.HasExited) {
    $taskProcess.Refresh()
    $taskPeak = [Math]::Max($taskPeak, $taskProcess.PeakWorkingSet64)
    Start-Sleep -Milliseconds 200
}
$taskProcess.WaitForExit()
$taskTimer.Stop()
$taskMeasurement = [ordered]@{
    mode = $Mode
    pages = $Pages
    peak_working_set_bytes = $taskPeak
    wall_seconds = $taskTimer.Elapsed.TotalSeconds
    exit_code = $taskProcess.ExitCode
    logical_processors = [Environment]::ProcessorCount
    os = [Environment]::OSVersion.VersionString
    process_scope = 'dataset generator, local HTTP fixture server (http mode), output verification and restart'
    source_revision = (git -C $taskRoot rev-parse HEAD)
    working_tree_modified = [bool](git -C $taskRoot status --porcelain)
}
$taskMeasurement | ConvertTo-Json | Set-Content -LiteralPath "$taskOutput.measurement.json"
Get-Content -LiteralPath "$taskOutput.stdout.log"
Get-Content -LiteralPath "$taskOutput.stderr.log"
$taskMeasurement | ConvertTo-Json
if ($taskProcess.ExitCode -ne 0) { exit $taskProcess.ExitCode }
