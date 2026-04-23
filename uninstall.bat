@echo off
title Brave Volume Restore - Desinstallation

echo.
echo === Brave Volume Restore - Desinstallation ===
echo.

:: Arreter le daemon
taskkill /im brave-volume-restore.exe /f >nul 2>&1
echo [OK] Daemon arrete.

:: Supprimer la cle registre
reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v "BraveVolumeRestore" /f >nul 2>&1
echo [OK] Demarrage automatique supprime.

:: Supprimer les fichiers
rmdir /s /q "%APPDATA%\BraveVolumeRestore" >nul 2>&1
echo [OK] Fichiers supprimes.

echo.
echo === Desinstallation terminee ! ===
echo.
pause
