; ============================================
; WMS 仓库管理系统 - Inno Setup 安装脚本
; 生成 Windows 安装程序 (setup.exe)
; ============================================

#define MyAppName "RWMS仓库管理系统"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "RWMS开发团队"
#define MyAppExeName "rwms.exe"
#define MyAppURL "http://localhost:8080/login.html"
#define MyAppAssocExt ".wms"
#define MyAppAssocKey "RWMS"

[Setup]
; 应用基本信息
AppId={{A5D5E8B2-3C4F-4E8D-9F1A-2B3C4D5E6F7A}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
; 输出文件设置
OutputDir=target\release\installer
OutputBaseFilename=rwms-setup-{#MyAppVersion}
SetupIconFile=
; 压缩和安装选项
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
; 程序相关
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
; 卸载信息
UninstallDisplayIcon={app}\{#MyAppExeName}
UninstallDisplayName={#MyAppName} {#MyAppVersion}

[Languages]
Name: "zh"; MessagesFile: "compiler:Default.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "创建桌面快捷方式"; GroupDescription: "附加图标:"; Flags: unchecked; Languages: zh
Name: "autostartup"; Description: "开机自启动"; GroupDescription: "启动选项:"; Flags: unchecked;

[Files]
; 主程序 exe
Source: "target\release\rwms.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "README.md"; DestDir: "{app}"; Flags: ignoreversion isreadme
Source: "LICENSE"; DestDir: "{app}"; Flags: ignoreversion
; 静态前端页面（HTML/CSS/JS）
Source: "static\*"; DestDir: "{app}\static"; Flags: ignoreversion recursesubdirs createallsubdirs
; 配置文件
Source: "application.yml"; DestDir:"{app}"; Flags: ignoreversion onlyifdoesntexist
; 必要的数据文件目录
Source: "logs"; DestDir: "{app}\logs"; Flags: ignoreversion recursesubdirs createallsubdirs onlyifdoesntexist
Source: "database"; DestDir: "{app}\database"; Flags: ignoreversion recursesubdirs createallsubdirs onlyifdoesntexist

[Icons]
; 开始菜单快捷方式
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\打开管理后台"; Filename: "{app}\{#MyAppExeName}"; Parameters: "--open-browser"
Name: "{group}\卸载 {#MyAppName}"; Filename: "{uninstallexe}"
; 桌面快捷方式（可选）
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon
; 开机自启动（可选）
Name: "{commonstartup}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: autostartup

[Run]
; 安装完成后启动程序
Filename: "{app}\{#MyAppExeName}"; Description: "启动 {#MyAppName}"; Flags: nowait postinstall skipifsilent; Languages: zh
Filename: "{app}\{#MyAppExeName}"; Description: "Launch {#MyAppName}"; Flags: nowait postinstall skipifsilent; Languages: english

[UninstallDelete]
; 卸载时删除数据目录中的日志和数据库（保留用户数据，仅清理程序文件）
Type: filesandordirs; Name: "{app}\static"
Type: files; Name: "{app}\README.md"
Type: files; Name: "{app}\LICENSE"

[Registry]
; 注册表项（用于版本检测等）
Root: HKCU; Subkey: "Software\{#MyAppName}"; ValueType: string; ValueName: "InstallPath"; ValueData: "{app}"
Root: HKCU; Subkey: "Software\{#MyAppName}"; ValueType: string; ValueName: "Version"; ValueData: "{#MyAppVersion}"

[Code]
// 安装前检查：如果程序正在运行则提示关闭
function InitializeSetup(): Boolean;
var ResultCode: Integer;
begin
  // 检查是否已有实例在运行
  if Exec(ExpandConstant('{win}\system32\tasklist.exe'), '/FI "IMAGENAME eq rwms.exe"', '', SW_HIDE, ewWaitUntilTerminated, ResultCode) then begin
    if ResultCode = 0 then begin
      if MsgBox('WMS 服务正在运行，是否继续安装？'#13#10'建议先关闭当前运行的服务。', mbConfirmation, MB_YESNO) = IDNO then begin
        Result := False;
        Exit;
      end;
    end;
  end;
  Result := True;
end;

// 安装完成后提示用户访问地址
procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then begin
    MsgBox('安装完成！'#13#10#13#10'请通过浏览器访问:'#13#10#13#10'http://localhost:8080/login.html'#13#10#13#10'默认管理员账号: admin / admin', mbInformation, MB_OK);
  end;
end;
