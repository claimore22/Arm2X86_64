# Starts the supervisor loop (new window) and runs the main worker (this window).
# Usage (from this script's directory, i.e. the repo root):
#   .\start-supervisor-and-worker.ps1              # supervisor loop + one fresh worker run
#   .\start-supervisor-and-worker.ps1 -SupervisorOnly   # supervisor loop only
param(
  [switch]$SupervisorOnly,
  [string]$Model = "nvidia/nvidia/nemotron-3-ultra-550b-a55b"
)

$ErrorActionPreference = "Stop"
$Root = $PSScriptRoot
Set-Location -LiteralPath $Root

$Opencode = "C:\Users\claimoar\AppData\Local\hermes\node\node_modules\opencode-ai\bin\opencode.exe"
if (-not (Test-Path -LiteralPath $Opencode)) {
  throw "opencode not found at $Opencode"
}

# --- 1. Supervisor loop in its own window (single instance) ---
$already = Get-Process -Name "supervisor" -ErrorAction SilentlyContinue
if ($already) {
  Write-Host "supervisor already running (PID $($already.Id)); skipping new loop."
} else {
  Write-Host "starting supervisor loop in a new window..."
  Start-Process -FilePath "pwsh" `
    -ArgumentList "-NoExit", "-Command", "Set-Location -LiteralPath '$Root'; cargo run --manifest-path supervisor/Cargo.toml -- --loop --execute" `
    -WorkingDirectory $Root
}

if ($SupervisorOnly) {
  Write-Host "supervisor-only mode; not starting worker."
  return
}

# --- 2. Main worker: fresh session (no --continue), foreground ---
$inprog = "unknown"
$m = Select-String -Pattern '^IN_PROGRESS:\s*(.+?)\s*$' -LiteralPath "$Root\AI_PROGRESS.md" | Select-Object -Last 1
if ($m) { $inprog = $m.Matches.Groups[1].Value.Trim() }

Write-Host "starting fresh worker (IN_PROGRESS=$inprog)..."
& $Opencode run -m $Model "Read WORKER_PROMPT.md, AI_RULES.md, AI_PLAN.md, AI_PROGRESS.md, AI_DECISIONS.md. State: IN_PROGRESS=$inprog tests green. Resume the autonomous loop now: write fresh HEARTBEAT + IN_PROGRESS, implement the next instruction, test, update ledger."
