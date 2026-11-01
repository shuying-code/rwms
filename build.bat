@echo off
chcp 65001 >nul
echo ============================================
echo RWMS仓库管理系统 - 自动化构建脚本
echo ============================================

setlocal enabledelayedexpansion

:: 检查 Rust 环境
call rustc --version >nul 2>&1
if !errorlevel! neq 0 (
    echo [ERROR] Rust 环境未安装或未正确配置
    echo 请先安装 Rust: https://www.rust-lang.org/
    pause
    exit /b 1
)

echo [INFO] 检查到 Rust 环境

:: 检查 Inno Setup 是否可用
set ISCC_EXE=""

:: 常见安装路径
if exist "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" (
    set "ISCC_EXE=C:\Program Files (x86)\Inno Setup 6\ISCC.exe"
) else if exist "C:\Program Files\Inno Setup 6\ISCC.exe" (
    set "ISCC_EXE=C:\Program Files\Inno Setup 6\ISCC.exe"
)

if "%ISCC_EXE%"=="" (
    echo [WARNING] Inno Setup 未安装，将进行普通编译但不打包
    set BUILD_ONLY=1
) else (
    echo [INFO] 检查到 Inno Setup: %ISCC_EXE%
    set BUILD_ONLY=0
)

:: 步骤1：清理之前的构建
echo.
echo [STEP 1] 清理之前的构建文件...
if exist "target\release\rwms.exe" (
    del /q "target\release\rwms.exe"
)
if exist "target\release\installer" (
    rmdir /s /q "target\release\installer"
)

:: 步骤2：构建 release 版本
echo.
echo [STEP 2] 构建 release 版本...
call cargo build --release

if %errorlevel% neq 0 (
    echo [ERROR] 编译失败！
    pause
    exit /b 1
)

echo [SUCCESS] Release 版本构建成功

:: 步骤3：准备安装程序文件
echo.
echo [STEP 3] 准备安装程序资源...
if not exist "target\release\rwms.exe" (
    echo [ERROR] 编译产物不存在
    pause
    exit /b 1
)

:: 创建必要的目录
mkdir "logs" 2>nul
mkdir "database" 2>nul

:: 步骤4：生成安装程序
if "%BUILD_ONLY%"=="0" (
    echo.
    echo [STEP 4] 生成 Windows 安装程序...
    "%ISCC_EXE%" installer.iss
    
    if %errorlevel% neq 0 (
        echo [ERROR] Inno Setup 编译失败！
        pause
        exit /b 1
)
    
    echo [SUCCESS] 安装程序生成成功
    echo.
    echo 安装程序位置: target\release\installer\rwms-setup-1.0.0.exe
    echo.
    echo ============================================
    echo 构建完成！
    echo ============================================
) else (
    echo.
    echo ============================================
    echo 构建完成！(未安装 Inno Setup，跳过打包)
    echo 可执行文件: target\release\rwms.exe
    echo ============================================
)

pause
exit /b 0