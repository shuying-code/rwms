# ============================================
# RWMS仓库管理系统 - PowerShell 自动化构建脚本
# ============================================

Write-Host "============================================" -ForegroundColor Green
Write-Host "RWMS仓库管理系统 - 自动化构建脚本" -ForegroundColor Green
Write-Host "============================================" -ForegroundColor Green

# 步骤1：检查 Rust 环境
Write-Host ""
Write-Host "[STEP 1] 检查 Rust 环境..." -ForegroundColor Cyan

try {
    $rustcVersion = rustc --version
    Write-Host "[INFO] $rustcVersion" -ForegroundColor Green
} catch {
    Write-Host "[ERROR] Rust 环境未安装或未正确配置" -ForegroundColor Red
    Write-Host "请先安装 Rust: https://www.rust-lang.org/" -ForegroundColor Yellow
    Read-Host "按任意键退出"
    exit 1
}

# 步骤2：检查 Inno Setup
Write-Host ""
Write-Host "[STEP 2] 检查 Inno Setup 环境..." -ForegroundColor Cyan

$isccPaths = @(
    "C:\Program Files (x86)\Inno Setup 6\ISCC.exe",
    "C:\Program Files\Inno Setup 6\ISCC.exe"
)

$isccExe = $null
foreach ($path in $isccPaths) {
    if (Test-Path $path) {
        $isccExe = $path
        break
    }
}

if ($isccExe) {
    Write-Host "[INFO] 检查到 Inno Setup: $isccExe" -ForegroundColor Green
    $buildOnly = $false
} else {
    Write-Host "[WARNING] Inno Setup 未安装，将进行普通编译但不打包" -ForegroundColor Yellow
    $buildOnly = $true
}

# 步骤3：清理之前的构建
Write-Host ""
Write-Host "[STEP 3] 清理之前的构建文件..." -ForegroundColor Cyan

if (Test-Path "target\release\rwms.exe") {
    Remove-Item "target\release\rwms.exe" -Force
}

if (Test-Path "target\release\installer") {
    Remove-Item "target\release\installer" -Recurse -Force
}

# 确保必要的目录存在
if (!(Test-Path "logs")) {
    New-Item -ItemType Directory -Path "logs" | Out-Null
}

if (!(Test-Path "database")) {
    New-Item -ItemType Directory -Path "database" | Out-Null
}

# 步骤4：构建 release 版本
Write-Host ""
Write-Host "[STEP 4] 构建 release 版本..." -ForegroundColor Cyan

try {
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        throw "编译失败"
    }
    Write-Host "[SUCCESS] Release 版本构建成功" -ForegroundColor Green
} catch {
    Write-Host "[ERROR] 编译失败！" -ForegroundColor Red
    Read-Host "按任意键退出"
    exit 1
}

# 检查编译产物
if (!(Test-Path "target\release\rwms.exe")) {
    Write-Host "[ERROR] 编译产物不存在" -ForegroundColor Red
    exit 1
}

# 步骤5：生成安装程序（如果Inno Setup可用）
if (-not $buildOnly) {
    Write-Host ""
    Write-Host "[STEP 5] 生成 Windows 安装程序..." -ForegroundColor Cyan
    
    try {
        & $isccExe installer.iss
        if ($LASTEXITCODE -ne 0) {
            throw "Inno Setup 编译失败"
        }
        Write-Host "[SUCCESS] 安装程序生成成功" -ForegroundColor Green
        Write-Host ""
        Write-Host "安装程序位置: target\release\installer\rwms-setup-1.0.0.exe" -ForegroundColor Yellow
    } catch {
        Write-Host "[ERROR] Inno Setup 编译失败！" -ForegroundColor Red
    }
} else {
    Write-Host ""
    Write-Host "可执行文件: target\release\rwms.exe" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "============================================" -ForegroundColor Green
if (-not $buildOnly) {
    Write-Host "构建完成！安装程序已生成。" -ForegroundColor Green
} else {
    Write-Host "构建完成！(未安装 Inno Setup，跳过打包)" -ForegroundColor Green
}
Write-Host "============================================" -ForegroundColor Green

Read-Host "按任意键退出"