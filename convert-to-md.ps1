# Converts all HTML files in www.scs.stanford.edu\arm64 to Markdown in www.scs.stanford.edu\md
$Pandoc = "C:\Program Files\Pandoc\pandoc.exe"
$SourceDir = Join-Path $PSScriptRoot "www.scs.stanford.edu\arm64"
$OutputDir = Join-Path $PSScriptRoot "www.scs.stanford.edu\md"

New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

$files = Get-ChildItem -LiteralPath $SourceDir -Filter *.html
$total = $files.Count
$i = 0
foreach ($f in $files) {
    $i++
    $out = Join-Path $OutputDir ($f.BaseName + ".md")
    & $Pandoc -f html -t gfm --wrap=none -o $out $f.FullName
    if ($i % 100 -eq 0) { Write-Host "$i / $total converted..." }
}
Write-Host "Done: $i of $total files converted to $OutputDir"
