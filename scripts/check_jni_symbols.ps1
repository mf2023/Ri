# JNI Symbol Completeness Check
# Compares native method declarations in Java sources against JNI implementations in Rust sources.
# Usage: powershell -File scripts/check_jni_symbols.ps1
# Exits with code 1 (and prints MISSING lines) if any declared native method lacks a Rust JNI implementation.

$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

$repoRoot = Split-Path -Parent $PSScriptRoot
$javaRoot = Join-Path $repoRoot 'java\src\main\java\com\dunimd\ri'
$rustJavaDir = Join-Path $repoRoot 'src\java\classes'

if (-not (Test-Path $javaRoot)) {
    Write-Error "Java source root not found: $javaRoot"
    exit 1
}
if (-not (Test-Path $rustJavaDir)) {
    Write-Error "Rust JNI source root not found: $rustJavaDir"
    exit 1
}

# --- Collect implemented JNI symbols from Rust sources ---
# Matches: fn Java_com_dunimd_ri_XYZ_method0(  (also handles generic lifetimes like <'local>)
$rustSymbols = New-Object 'System.Collections.Generic.HashSet[string]'
$rustFiles = Get-ChildItem -LiteralPath $rustJavaDir -Filter '*.rs' -Recurse
foreach ($file in $rustFiles) {
    $content = Get-Content -LiteralPath $file.FullName -Raw
    foreach ($m in [regex]::Matches($content, 'fn\s+(Java_[A-Za-z0-9_]+)\s*[<(]')) {
        [void]$rustSymbols.Add($m.Groups[1].Value)
    }
}
Write-Host "Rust JNI implementations: $($rustSymbols.Count)"

# --- Collect declared native methods from Java sources ---
# Derives the JNI symbol name from the package declaration and the native method name.
$javaSymbols = New-Object 'System.Collections.Generic.HashSet[string]'
$javaFiles = Get-ChildItem -LiteralPath $javaRoot -Filter '*.java' -Recurse
foreach ($file in $javaFiles) {
    $content = Get-Content -LiteralPath $file.FullName -Raw
    $pkgMatch = [regex]::Match($content, '(?m)^\s*package\s+([\w.]+)\s*;')
    if (-not $pkgMatch.Success) { continue }
    $pkgPrefix = 'Java_' + ($pkgMatch.Groups[1].Value -replace '\.', '_')
    foreach ($m in [regex]::Matches($content, 'native\s+[\w\[\].<>]+\s+(\w+0?)\s*\(')) {
        $name = $m.Groups[1].Value
        # Skip false positives from constructor-like matches (e.g. "RiLogger(" parsed as type)
        if ($name -notmatch '^[_a-zA-z][_a-zA-Z0-9]*$') { continue }
        # Native method names may end with digit suffix (0); strip trailing digits for class-method join
        [void]$javaSymbols.Add("$pkgPrefix::$name")
    }
}
Write-Host "Java native declarations: $($javaSymbols.Count)"

# --- Compare ---
# Java native method "foo0" in class "Bar" maps to JNI symbol Java_<pkg>_Bar_foo0.
# We need the class name, so re-scan per file with class context.
$missing = New-Object 'System.Collections.Generic.List[string]'
foreach ($file in $javaFiles) {
    $content = Get-Content -LiteralPath $file.FullName -Raw
    $pkgMatch = [regex]::Match($content, '(?m)^\s*package\s+([\w.]+)\s*;')
    if (-not $pkgMatch.Success) { continue }
    $pkgPrefix = 'Java_' + ($pkgMatch.Groups[1].Value -replace '\.', '_')
    foreach ($classMatch in [regex]::Matches($content, '(?m)^\s*(?:public|protected|private)?\s*(?:final\s+|abstract\s+|static\s+)*class\s+(\w+)')) {
        $className = $classMatch.Groups[1].Value
        $classStart = $classMatch.Index
        $classBody = $content.Substring($classStart)
        foreach ($m in [regex]::Matches($classBody, 'native\s+[\w\[\].<>]+\s+(\w+)\s*\(')) {
            $methodName = $m.Groups[1].Value
            $symbol = "$pkgPrefix`_$className`_$methodName"
            if (-not $rustSymbols.Contains($symbol)) {
                $relPath = $file.FullName.Substring($repoRoot.Length + 1)
                $missing.Add("$relPath : $className.$methodName -> expected $symbol")
            }
        }
    }
}

if ($missing.Count -gt 0) {
    Write-Host ""
    Write-Host "MISSING: $($missing.Count) JNI implementation(s):" -ForegroundColor Red
    foreach ($line in $missing) { Write-Host "  $line" }
    exit 1
}

Write-Host ""
Write-Host "OK: all Java native declarations have matching Rust JNI implementations." -ForegroundColor Green
exit 0
