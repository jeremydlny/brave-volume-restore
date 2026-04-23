# Brave Volume Restore

Brave remet son volume à 100% dans le mixer Windows à chaque ouverture.
Ce daemon Rust tourne en fond et le restaure automatiquement à ta dernière valeur.

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
Brave s'ouvre   →  Daemon restaure le volume depuis volume.cfg
Volume changé   →  Daemon détecte le changement et sauvegarde dans volume.cfg
Brave se ferme  →  Daemon continue de tourner en arrière-plan
```

Le daemon tourne sans fenêtre, sans icône dans la barre des tâches.
Il utilise ~1MB de RAM et 0% de CPU en dehors du poll toutes les 500ms.

## Fichiers

```
brave-volume-restore/
├── src/main.rs         ← code Rust (~120 lignes)
├── Cargo.toml          ← dépendance unique : crate `windows`
├── install.bat         ← compile + installe
└── uninstall.bat       ← désinstalle proprement
```

Config : `%APPDATA%\BraveVolumeRestore\volume.cfg` (float 0.0–1.0)

## Désinstaller

```
double-clic sur uninstall.bat
```
