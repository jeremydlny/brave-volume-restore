# Brave Volume Restore

Daemon Rust léger qui surveille le mixer audio Windows et remet Brave
à ton volume préféré dès qu'il s'ouvre.

## Pré-requis

- [Rust](https://rustup.rs) installé (pour la compilation)
- Windows 10/11

## Installation

```
double-clic sur install.bat
```

Le script :
1. Compile le binaire en release (`~500KB`, zéro dépendance externe)
2. Copie l'exe dans `%APPDATA%\BraveVolumeRestore\`
3. Configure le démarrage automatique au logon via le registre Windows
4. Lance le daemon immédiatement

## Comment ça marche

```
[Daemon Rust] ──poll 500ms──▶ Windows Audio Session Manager
                                    │
                     brave.exe apparaît dans le mixer
                                    │
                    SetMasterVolume(volume.cfg) ──────────────┐
                                                              │
                     tu changes le volume manuellement        │
                                    │                         │
                         sauvegarde dans volume.cfg ──────────┘
```

- Brave s'ouvre → le daemon restaure le dernier volume connu (50% par défaut)
- Tu changes le volume dans le mixer → le daemon le détecte et le sauvegarde
- Tu fermes et rouvres Brave → repart exactement où t'en étais

Le daemon tourne en arrière-plan sans fenêtre, sans icône dans la barre des tâches.
Il utilise ~1MB de RAM et 0% de CPU en dehors du poll toutes les 500ms.

## Fichiers

```
brave-volume-restore/
├── src/main.rs         ← code Rust (~120 lignes)
├── Cargo.toml          ← dépendance unique : crate `windows`
└── install.bat         ← compile + installe
```

Config : `%APPDATA%\BraveVolumeRestore\volume.cfg` (float 0.0–1.0)

## Désinstaller

```bat
reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v "BraveVolumeRestore" /f
taskkill /im brave-volume-restore.exe /f
rmdir /s /q "%APPDATA%\BraveVolumeRestore"
```