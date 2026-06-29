@echo off
setlocal

set "APP_NAME=sensor-studio-core"
set "DIST_DIR=dist"

for /f "tokens=2 delims=@" %%v in ('cargo pkgid 2^>nul') do (
    set "VERSION=%%v"
)

if "%VERSION%"=="" (
    echo [ERROR] failed to read version from Cargo.toml
    exit /b 1
)

echo [INFO] version: %VERSION%

cargo build --release
if errorlevel 1 exit /b 1

if not exist "%DIST_DIR%" (
    mkdir "%DIST_DIR%"
    if errorlevel 1 exit /b 1
)

if not exist "target\release\%APP_NAME%.exe" (
    echo [ERROR] build output not found: target\release\%APP_NAME%.exe
    exit /b 1
)

copy /Y "target\release\%APP_NAME%.exe" "%DIST_DIR%\%APP_NAME%-v%VERSION%-windows-x86_64.exe" >nul
if errorlevel 1 exit /b 1

echo [INFO] artifacts created:
echo   - %DIST_DIR%\%APP_NAME%-v%VERSION%-windows-x86_64.exe

endlocal