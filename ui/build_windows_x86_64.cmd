@echo off
setlocal EnableExtensions EnableDelayedExpansion

set "TARGET_OS=windows"
set "TARGET_ARCH=x86_64"
set "DIST_DIR=dist"
set "BUILD_DIR=build\windows\x64\runner\Release"

cd /d "%~dp0"

if not exist "pubspec.yaml" (
    echo [ERROR] pubspec.yaml not found. Please run this script from the ui project directory.
    exit /b 1
)

where flutter >nul 2>nul
if errorlevel 1 (
    echo [ERROR] flutter command not found.
    exit /b 1
)

for /f "tokens=1,* delims=:" %%A in ('findstr /b /c:"name:" pubspec.yaml') do (
    set "APP_NAME=%%B"
)

for /f "tokens=1,* delims=:" %%A in ('findstr /b /c:"version:" pubspec.yaml') do (
    set "VERSION_RAW=%%B"
)

rem trim leading spaces
for /f "tokens=* delims= " %%A in ("!APP_NAME!") do set "APP_NAME=%%A"
for /f "tokens=* delims= " %%A in ("!VERSION_RAW!") do set "VERSION_RAW=%%A"

rem remove quotes
set "APP_NAME=!APP_NAME:"=!"
set "VERSION_RAW=!VERSION_RAW:"=!"

rem remove build number: 0.1.0+1 -> 0.1.0
for /f "tokens=1 delims=+" %%A in ("!VERSION_RAW!") do (
    set "VERSION=%%A"
)

if "!APP_NAME!"=="" (
    echo [ERROR] failed to read app name from pubspec.yaml
    exit /b 1
)

if "!VERSION!"=="" (
    echo [ERROR] failed to read version from pubspec.yaml
    exit /b 1
)

rem sensor_studio_ui -> sensor-studio-ui
set "PACKAGE_NAME=!APP_NAME:_=-!"

set "ARTIFACT_NAME=!PACKAGE_NAME!-v!VERSION!-!TARGET_OS!-!TARGET_ARCH!"
set "OUTPUT_DIR=!DIST_DIR!\!ARTIFACT_NAME!"

echo [INFO] app name      : !APP_NAME!
echo [INFO] version       : !VERSION_RAW!
echo [INFO] artifact name : !ARTIFACT_NAME!

call flutter pub get
if errorlevel 1 exit /b 1

call flutter build windows --release
if errorlevel 1 exit /b 1

if not exist "!BUILD_DIR!" (
    echo [ERROR] build output not found: !BUILD_DIR!
    exit /b 1
)

if not exist "!DIST_DIR!" (
    mkdir "!DIST_DIR!"
)

if exist "!OUTPUT_DIR!" (
    rmdir /s /q "!OUTPUT_DIR!"
)

mkdir "!OUTPUT_DIR!"

xcopy "!BUILD_DIR!\*" "!OUTPUT_DIR!\" /E /I /Y >nul
if errorlevel 1 (
    echo [ERROR] failed to copy build output.
    exit /b 1
)

(
    echo Application: !APP_NAME!
    echo Version: !VERSION_RAW!
    echo Target: !TARGET_OS!-!TARGET_ARCH!
    echo Built At: %DATE% %TIME%
) > "!OUTPUT_DIR!\RELEASE_INFO.txt"

where tar >nul 2>nul
if errorlevel 1 (
    echo [WARN] tar command not found. zip archive skipped.
    echo [INFO] release bundle created:
    echo        !OUTPUT_DIR!
    exit /b 0
)

echo [INFO] release bundle created:
echo        !OUTPUT_DIR!

endlocal