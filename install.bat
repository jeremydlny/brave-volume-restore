@echo off
setlocal enabledelayedexpansion
title Brave Volume Restore - Installation

echo.
echo === Brave Volume Restore - Setup ===
echo.

:: Verifier Rust
cargo --version >nul 2>&1
if errorlevel 1 (
    echo [ERREUR] Rust non trouve. Installe-le depuis https://rustup.rs
    pause & exit /b 1
)
echo [OK] Rust detecte.

:: Compiler
echo [INFO] Compilation en cours...
cargo build --release
if errorlevel 1 (
    echo [ERREUR] Compilation echouee.
    pause & exit /b 1
)
echo [OK] Compilation reussie.

:: Copier le binaire
set "INSTALL_DIR=%APPDATA%\BraveVolumeRestore"
set "EXE_SRC=%~dp0target\release\brave-volume-restore.exe"
set "EXE_DST=%INSTALL_DIR%\brave-volume-restore.exe"

if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"
copy /y "%EXE_SRC%" "%EXE_DST%" >nul
echo [OK] Binaire copie : %EXE_DST%

:: Demarrage automatique via le registre (pas besoin d'admin)
reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v "BraveVolumeRestore" /t REG_SZ /d "\"%EXE_DST%\"" /f >nul 2>&1
if errorlevel 1 (
    echo [ERREUR] Impossible d'ecrire dans le registre.
    pause & exit /b 1
)
echo [OK] Demarrage automatique au logon configure.

:: Demarrer le daemon maintenant
start "" "%EXE_DST%"
echo [OK] Daemon demarre.

echo.
echo === Installation terminee ! ===
echo.
echo  Le daemon tourne en fond et memorise automatiquement
echo  le volume que tu regles dans le mixer Windows.
echo  Au prochain demarrage de Brave, il sera restaure.
echo.
echo  Premier lancement : volume par defaut a 50%%.
echo.
pause
