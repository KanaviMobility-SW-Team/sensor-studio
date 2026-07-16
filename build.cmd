@echo off
setlocal EnableExtensions

set "ROOT_DIR=%~dp0"
set "DIST_DIR=%ROOT_DIR%dist"
set "CORE_DIST_DIR=%ROOT_DIR%core\dist"
set "UI_DIST_DIR=%ROOT_DIR%ui\dist"

echo [INFO] Preparing root dist directory...

if exist "%DIST_DIR%" (
    rmdir /s /q "%DIST_DIR%"
    if errorlevel 1 (
        echo [ERROR] Failed to clean root dist directory.
        exit /b 1
    )
)

mkdir "%DIST_DIR%"
if errorlevel 1 (
    echo [ERROR] Failed to create root dist directory.
    exit /b 1
)

echo [INFO] Building Sensor Studio Core...

pushd "%ROOT_DIR%core"
call build.cmd
set "BUILD_EXIT_CODE=%ERRORLEVEL%"
popd

if not "%BUILD_EXIT_CODE%"=="0" (
    echo [ERROR] Sensor Studio Core build failed.
    exit /b %BUILD_EXIT_CODE%
)

call :move_dist_contents "%CORE_DIST_DIR%" "Core"
if errorlevel 1 exit /b 1

echo [INFO] Building Sensor Studio UI...

pushd "%ROOT_DIR%ui"
call build_windows_x86_64.cmd
set "BUILD_EXIT_CODE=%ERRORLEVEL%"
popd

if not "%BUILD_EXIT_CODE%"=="0" (
    echo [ERROR] Sensor Studio UI build failed.
    exit /b %BUILD_EXIT_CODE%
)

call :move_dist_contents "%UI_DIST_DIR%" "UI"
if errorlevel 1 exit /b 1

echo [INFO] Sensor Studio build completed successfully.
echo [INFO] Artifacts: %DIST_DIR%

endlocal
exit /b 0


:move_dist_contents
set "SOURCE_DIR=%~1"
set "COMPONENT_NAME=%~2"

if not exist "%SOURCE_DIR%\" (
    echo [ERROR] %COMPONENT_NAME% dist directory was not created: %SOURCE_DIR%
    exit /b 1
)

dir /b /a "%SOURCE_DIR%" >nul 2>&1
if errorlevel 1 (
    echo [WARN] No %COMPONENT_NAME% artifacts found in: %SOURCE_DIR%
    exit /b 0
)

echo [INFO] Collecting %COMPONENT_NAME% artifacts...

for /f "delims=" %%F in ('dir /b /a "%SOURCE_DIR%"') do (
    move /Y "%SOURCE_DIR%\%%F" "%DIST_DIR%\" >nul
    if errorlevel 1 (
        echo [ERROR] Failed to move artifact: %%F
        exit /b 1
    )
)

exit /b 0