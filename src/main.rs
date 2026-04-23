#![windows_subsystem = "windows"]

use std::{path::PathBuf, thread, time::Duration};
use windows::{
    core::*,
    Win32::{
        Foundation::CloseHandle,
        Media::Audio::{
            eConsole, eRender, IAudioSessionControl2, IAudioSessionEnumerator,
            IAudioSessionManager2, IMMDeviceEnumerator, ISimpleAudioVolume, MMDeviceEnumerator,
        },
        System::{
            Com::{CoInitializeEx, CoCreateInstance, CLSCTX_ALL, COINIT_APARTMENTTHREADED},
            Threading::{
                OpenProcess, QueryFullProcessImageNameW,
                PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
            },
        },
    },
};

// ── Config ────────────────────────────────────────────────────────────────────

fn config_path() -> PathBuf {
    let base = std::env::var("APPDATA").unwrap_or_else(|_| ".".into());
    let dir = PathBuf::from(base).join("BraveVolumeRestore");
    let _ = std::fs::create_dir_all(&dir);
    dir.join("volume.cfg")
}

fn load_volume() -> f32 {
    std::fs::read_to_string(config_path())
        .ok()
        .and_then(|s| s.trim().parse::<f32>().ok())
        .map(|v| v.clamp(0.0, 1.0))
        .unwrap_or(0.5)
}

fn save_volume(vol: f32) {
    let _ = std::fs::write(config_path(), format!("{:.4}", vol));
}

// ── Windows Audio API ─────────────────────────────────────────────────────────

unsafe fn process_name(pid: u32) -> Option<String> {
    let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
    let mut buf = [0u16; 260];
    let mut size = buf.len() as u32;
    let _ = QueryFullProcessImageNameW(
        handle,
        PROCESS_NAME_WIN32,
        PWSTR(buf.as_mut_ptr()),
        &mut size,
    );
    let _ = CloseHandle(handle);
    if size == 0 { return None; }
    Some(String::from_utf16_lossy(&buf[..size as usize]))
}

unsafe fn brave_sessions() -> Result<Vec<ISimpleAudioVolume>> {
    let enumerator: IMMDeviceEnumerator =
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
    let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
    let manager: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, None)?;
    let sessions: IAudioSessionEnumerator = manager.GetSessionEnumerator()?;
    let count = sessions.GetCount()?;

    let mut result = Vec::new();
    for i in 0..count {
        let ctrl = match sessions.GetSession(i) { Ok(c) => c, Err(_) => continue };
        let ctrl2: IAudioSessionControl2 = match ctrl.cast() { Ok(c) => c, Err(_) => continue };
        let pid = match ctrl2.GetProcessId() { Ok(p) if p != 0 => p, _ => continue };
        let name = process_name(pid).unwrap_or_default().to_lowercase();
        if name.ends_with("brave.exe") {
            if let Ok(vol_iface) = ctrl2.cast::<ISimpleAudioVolume>() {
                result.push(vol_iface);
            }
        }
    }
    Ok(result)
}

unsafe fn brave_present() -> bool {
    brave_sessions().map(|s| !s.is_empty()).unwrap_or(false)
}

unsafe fn apply_volume(level: f32) {
    if let Ok(sessions) = brave_sessions() {
        for s in &sessions {
            let _ = s.SetMasterVolume(level, std::ptr::null());
        }
    }
}

/// Lit le volume actuel de Brave dans le mixer (premiere session trouvee).
unsafe fn get_brave_volume() -> Option<f32> {
    let sessions = brave_sessions().ok()?;
    let s = sessions.first()?;
    s.GetMasterVolume().ok()
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // --set 30 : sauvegarde 30% et quitte (utilise par set_volume.bat)
    if let Some(pos) = args.iter().position(|a| a == "--set") {
        if let Some(raw) = args.get(pos + 1) {
            if let Ok(pct) = raw.trim().trim_end_matches('%').parse::<f32>() {
                save_volume((pct / 100.0).clamp(0.0, 1.0));
            }
        }
        return;
    }

    unsafe { let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED); }

    let mut was_present = false;
    // Volume qu'on vient d'appliquer nous-memes (pour ne pas le re-sauvegarder)
    let mut last_applied: Option<f32> = None;

    loop {
        let present = unsafe { brave_present() };

        if present && !was_present {
            // Brave vient d'apparaitre -> restaure la derniere valeur sauvegardee
            let vol = load_volume();
            unsafe { apply_volume(vol) };
            last_applied = Some(vol);

        } else if present {
            // Brave tourne -> surveille si l'utilisateur a change le volume manuellement
            if let Some(current) = unsafe { get_brave_volume() } {
                let saved = load_volume();
                // Le volume a change ET c'est pas nous qui venons de l'appliquer
                let we_set_this = last_applied.map(|v| (v - current).abs() < 0.02).unwrap_or(false);
                if !we_set_this && (current - saved).abs() > 0.02 {
                    // L'utilisateur a change le volume -> on sauvegarde
                    save_volume(current);
                    last_applied = Some(current);
                }
            }
        }

        if !present {
            last_applied = None;
        }

        was_present = present;
        thread::sleep(Duration::from_millis(500));
    }
}
