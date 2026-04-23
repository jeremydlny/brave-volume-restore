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
3. Te demande ton volume cible (ex: `30` pour 30%)
4. Crée une tâche planifiée pour démarrer le daemon au logon
5. Lance le daemon immédiatement

## Changer le volume cible

```bat
set_volume.bat 25
```

Le daemon lit le fichier de config à chaque tick, donc la valeur sera
prise en compte dès la prochaine ouverture de Brave.

## Comment ça marche

```
[Daemon Rust] ──poll 500ms──▶ Windows Audio Session Manager
                                    │
                        brave.exe apparaît dans le mixer
                                    │
                         SetMasterVolume(0.30) ◀── volume.cfg
```

Le daemon tourne en arrière-plan sans fenêtre, sans icône dans la barre des tâches.
Il utilise ~1MB de RAM et 0% de CPU en dehors du poll toutes les 500ms.

## Fichiers

```
brave-volume-restore/
├── src/main.rs         ← code Rust (~100 lignes)
├── Cargo.toml          ← dépendance unique : crate `windows`
├── install.bat         ← compile + installe
└── set_volume.bat      ← change le volume cible
```

Config : `%APPDATA%\BraveVolumeRestore\volume.cfg` (float 0.0–1.0)

## Désinstaller

```bat
schtasks /delete /tn "BraveVolumeRestore" /f
taskkill /im brave-volume-restore.exe /f
rmdir /s /q "%APPDATA%\BraveVolumeRestore"
```
