; ============================================
; WMS 仓库管理系统 - Inno Setup 安装脚本
; 生成 Windows 安装程序（.exe）
; ============================================

[Setup]
; --- 应用基本信息 ---
AppName=WMS仓库管理系统
AppVersion=1.0.0
AppPublisher=WMS Team
AppPublisherURL=https://github.com/wms
AppSupportURL=https://github.com/wms/support
AppComments=仓库管理系统 - Rust + Axum 实现

; --- 安装目录 ---
DefaultDirName={autopf}\WMS
DefaultGroupName=WMS仓库管理系统

; --- 输出配置 ---
OutputDir=output
OutputBaseFilename=wms-setup-1.0.0
; SetupIconFile=static\favicon.ico
Compression=lzma2/ultra64
SolidCompression=yes

; --- 权限与兼容性 ---
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog commandline
MinVersion=10.0
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

; --- 安装界面 ---
WizardStyle=modern
UninstallDisplayIcon={app}\rwms.exe

; --- 许可协议（可选） ---
; LicenseFile=LICENSE.txt

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "创建桌面快捷方式"; GroupDescription: "附加选项:"; Flags: unchecked

[Files]
; 主程序
Source: "target\release\rwms.exe"; DestDir: "{app}"; Flags: ignoreversion
; 配置文件
Source: "application.yml"; DestDir: "{app}"; Flags: ignoreversion confirmoverwrite
; 前端静态文件
Source: "static\*.html"; DestDir: "{app}\static"; Flags: ignoreversion
; 数据库（如果已存在）
Source: "wms.db"; DestDir: "{app}"; Flags: ignoreversion skipifsourcedoesntexist

[Icons]
Name: "{group}\WMS仓库管理系统"; Filename: "{app}\rwms.exe"
Name: "{group}\卸载WMS"; Filename: "{uninstallexe}"
Name: "{autodesktop}\WMS仓库管理系统"; Filename: "{app}\rwms.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\rwms.exe"; Description: "启动WMS仓库管理系统"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
Type: filesandordirs; Name: "{app}\static"
Type: files; Name: "{app}\wms.db"
Type: files; Name: "{app}\application.yml"
