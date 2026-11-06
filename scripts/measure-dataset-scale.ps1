param(
    [ValidateRange(1, 10000000)][int]$Pages = 10000,
    [Parameter(Mandatory=$true)][string]$OutputDirectory,
    [ValidateSet('http','generated','http-fetch')][string]$Mode = 'http',
    [ValidateRange(1, 1000)][int]$RowsPerPage = 1,
    [ValidateRange(0, 86400)][int]$MaxWallSeconds = 0
)
$ErrorActionPreference = 'Stop'
$taskRoot = Split-Path $PSScriptRoot -Parent
$taskExe = Join-Path $taskRoot 'target\release\examples\dataset_scale.exe'
$taskOutput = [IO.Path]::GetFullPath($OutputDirectory)
if (Test-Path -LiteralPath $taskOutput) { throw 'Use a new output directory for each acceptance run.' }
$taskParent = Split-Path $taskOutput -Parent
New-Item -ItemType Directory -Force -Path $taskParent | Out-Null
$taskBinaryHash = (Get-FileHash -LiteralPath $taskExe -Algorithm SHA256).Hash
$taskStartedUtc = [DateTime]::UtcNow.ToString('o')
$taskTimer = [Diagnostics.Stopwatch]::StartNew()
$taskProcess = Start-Process -FilePath $taskExe -ArgumentList @($Pages, ('"' + $taskOutput + '"'), $Mode, $RowsPerPage) -PassThru -WindowStyle Hidden -RedirectStandardOutput "$taskOutput.stdout.log" -RedirectStandardError "$taskOutput.stderr.log"
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
    rows_per_page = $RowsPerPage
    executable_sha256 = $taskBinaryHash
    started_at_utc = $taskStartedUtc
    peak_working_set_bytes = $taskPeak
    wall_seconds = $taskTimer.Elapsed.TotalSeconds
    max_wall_seconds = $MaxWallSeconds
    exit_code = $taskProcess.ExitCode
    logical_processors = [Environment]::ProcessorCount
    os = [Environment]::OSVersion.VersionString
    process_scope = if ($Mode -eq 'http-fetch') { 'session HTTP dispatch, loopback fixture server, full body verification' } else { 'dataset generator, local HTTP fixture server (http mode), output verification and restart' }
    source_revision = (git -C $taskRoot rev-parse HEAD)
    working_tree_modified = [bool](git -C $taskRoot status --porcelain)
}
$taskMeasurement | ConvertTo-Json | Set-Content -LiteralPath "$taskOutput.measurement.json"
Get-Content -LiteralPath "$taskOutput.stdout.log"
Get-Content -LiteralPath "$taskOutput.stderr.log"
$taskMeasurement | ConvertTo-Json
if ($taskProcess.ExitCode -ne 0) { throw "Scale probe failed with exit code $($taskProcess.ExitCode). See $taskOutput.stderr.log" }
$taskAcceptance = Get-Content -LiteralPath (Join-Path $taskOutput 'acceptance.json') -Raw | ConvertFrom-Json
if ($taskAcceptance.mode -ne $Mode -or $taskAcceptance.pages -ne $Pages -or $taskAcceptance.rows_per_page -ne $RowsPerPage) {
    throw 'Acceptance workload differs from the requested inputs. Rebuild the release probe.'
}
if ($Mode -eq 'http-fetch') {
    if ($taskAcceptance.verified_body_bytes -le 0 -or $taskAcceptance.requests_per_second -le 0) { throw 'Missing verified HTTP throughput.' }
} elseif ($taskAcceptance.verified_records -ne ([long]$Pages * $RowsPerPage) -or
          $taskAcceptance.report.complete -ne $true -or $taskAcceptance.report.pages_failed -ne 0 -or
          $taskAcceptance.report.records_rejected -ne 0 -or $taskAcceptance.resume_verified -ne $true) {
    throw 'Missing verified records or successful restart.'
}
if ($MaxWallSeconds -gt 0 -and $taskTimer.Elapsed.TotalSeconds -ge $MaxWallSeconds) {
    throw "Verified output exceeded the wall-time budget of $MaxWallSeconds seconds."
}
