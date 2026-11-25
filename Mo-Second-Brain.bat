@echo off
rem Bam dup de mo Second Brain (Windows). Lan dau se tu tai chuong trinh (~5 MB).
chcp 65001 >nul
cd /d "%~dp0"
if not exist "bin\secondbrain.exe" (
  echo Dang tai chuong trinh Second Brain lan dau...
  if not exist bin mkdir bin
  powershell -NoProfile -ExecutionPolicy Bypass -Command "$ProgressPreference='SilentlyContinue'; Invoke-WebRequest -UseBasicParsing -Uri 'https://github.com/cxtungai/SecondBrain/releases/download/ban-moi-nhat/secondbrain-windows-x64.exe' -OutFile 'bin\secondbrain.exe'"
)
if not exist "bin\secondbrain.exe" (
  echo Khong tai duoc chuong trinh. Kiem tra mang, hoac nho Claude: "cai chuong trinh Second Brain giup toi".
  pause
  exit /b 1
)
"bin\secondbrain.exe" mo
if errorlevel 1 pause
