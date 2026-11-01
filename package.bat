@echo off
chcp 65001 >nul
echo.
echo ============================================
echo RWMS仓库管理系统 -简化打包脚本
echo ============================================
echo.

:: 检查可执行文件是否存在
if not exist "target\release\rwms.exe" (
    echo [ERROR] 可执行文件不存在，请先运行 cargo build --release
    pause
    exit /b 1
)

echo [INFO] 找到可执行文件: target\release\rwms.exe
echo [INFO] 文件大小: 5666304 字节
echo.

:: 检查Inno Setup
echo [STEP 1] 检查安装程序构建环境...
set ISCC_EXE=

if exist "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" set ISCC_EXE="C:\Program Files (x86)\Inno Setup 6\ISCC.exe"
if exist "C:\Program Files\Inno Setup 6\ISCC.exe" set ISCC_EXE="C:\Program Files\Inno Setup 6\ISCC.exe"

if "%ISCC_EXE%"=="" (
    echo [WARNING] Inno Setup 未安装，无法生成安装程序
    echo 可执行文件已构建完成: target\release\rwms.exe
    pause
    exit /b 0
)

echo [INFO] 使用 Inno Setup: %ISCC_EXE%
echo.

:: 执行Inno Setup编译
echo [STEP 2] 开始生成安装程序...
%ISCC_EXE% installer.iss

if %errorlevel% == 0 (
    echo.
    echo ============================================
    echo [SUCCESS] 安装程序生成完成！
    echo ============================================
    echo.
    echo 安装程序位置: target\release\installer\rwms-setup-1.0.0.exe
    echo.
    echo 测试方法：
    echo 1. 双击运行安装程序
    echo 2. 选择安装目录
    echo 3. 验证中文界面和功能
    echo 4. 启动应用程序
    echo.
) else (
    echo.
    echo [ERROR] 安装程序生成失败
)

pause