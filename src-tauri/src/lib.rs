mod config;
mod highlight;
mod input;
mod locate;
mod overlay;
mod selection;
mod sound;
mod styles;
mod window;

use config::Config;
use highlight::Undo;
use serde::Serialize;
use sound::Sound;
use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex, RwLock};
use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter, Manager, RunEvent, WebviewUrl, WebviewWindowBuilder, Wry};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

const COLOR_LABELS: [&str; 5] = ["Amarillo", "Rosa", "Verde", "Azul", "Lila"];
const COLOR_KEYS: [Code; 5] = [Code::Digit1, Code::Digit2, Code::Digit3, Code::Digit4, Code::Digit5];

struct AppState {
    config: Arc<RwLock<Config>>,
    sound: Sound,
    undo: Undo,
}

/// Tray entries that mirror settings, kept to update their checks.
struct TrayItems {
    enabled: CheckMenuItem<Wry>,
    sound: CheckMenuItem<Wry>,
    colors: Vec<CheckMenuItem<Wry>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Snapshot {
    config: Config,
    viewers: Vec<styles::Viewer>,
    detected_roots: Vec<PathBuf>,
    autostart: bool,
    css: &'static str,
}

fn update_config(app: &AppHandle, f: impl FnOnce(&mut Config)) {
    let state = app.state::<AppState>();
    let cfg = {
        let mut cfg = state.config.write().expect("config lock");
        f(&mut cfg);
        cfg.clone()
    };
    let _ = cfg.save();
    sync_tray(app, &cfg);
    let _ = app.emit("config", &cfg);
}

fn sync_tray(app: &AppHandle, cfg: &Config) {
    let items = app.state::<TrayItems>();
    let _ = items.enabled.set_checked(cfg.enabled);
    let _ = items.sound.set_checked(cfg.sound);
    for (item, color) in items.colors.iter().zip(mdmark::COLORS) {
        let _ = item.set_checked(cfg.color == color);
    }
}

#[tauri::command]
fn snapshot(app: AppHandle, state: tauri::State<AppState>) -> Snapshot {
    let config = state.config.read().expect("config lock").clone();
    Snapshot {
        detected_roots: locate::roots(&[]),
        viewers: styles::viewers(),
        autostart: app.autolaunch().is_enabled().unwrap_or(false),
        css: styles::CSS,
        config,
    }
}

#[tauri::command]
fn save_config(app: AppHandle, config: Config) {
    update_config(&app, |c| *c = config);
}

#[tauri::command]
fn install_style(kind: String, path: PathBuf) -> Result<(), String> {
    styles::install(&kind, &path)
}

#[tauri::command]
fn copy_css() -> Result<(), String> {
    arboard::Clipboard::new()
        .and_then(|mut c| c.set_text(styles::CSS))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn test_sound(state: tauri::State<AppState>, volume: f32) {
    state.sound.demo(volume);
}

#[tauri::command]
fn set_autostart(app: AppHandle, on: bool) -> Result<(), String> {
    let launcher = app.autolaunch();
    if on { launcher.enable() } else { launcher.disable() }.map_err(|e| e.to_string())
}

fn undo_last(app: &AppHandle) {
    let state = app.state::<AppState>();
    let last = state.undo.lock().expect("undo lock").take();
    match last {
        Some((path, source)) => {
            if let Err(e) = highlight::write_atomic(&path, &source) {
                highlight::notify(app, &format!("No se pudo deshacer: {e}"));
            }
        }
        None => highlight::notify(app, "No hay nada que deshacer."),
    }
}

fn open_settings(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("settings") {
        let _ = w.unminimize();
        let _ = w.show();
        let _ = w.set_focus();
        return;
    }
    let _ = WebviewWindowBuilder::new(app, "settings", WebviewUrl::App("index.html".into()))
        .title("Subrayador")
        .inner_size(480.0, 720.0)
        .min_inner_size(400.0, 520.0)
        .build();
}

fn build_tray(app: &AppHandle, cfg: &Config) -> tauri::Result<()> {
    let enabled = CheckMenuItem::with_id(app, "enabled", "Activado", true, cfg.enabled, None::<&str>)?;
    let colors = mdmark::COLORS
        .iter()
        .zip(COLOR_LABELS)
        .enumerate()
        .map(|(i, (id, label))| {
            let accel = format!("Ctrl+Alt+{}", i + 1);
            CheckMenuItem::with_id(app, format!("color:{id}"), label, true, cfg.color == *id, Some(accel))
        })
        .collect::<tauri::Result<Vec<_>>>()?;
    let color_refs: Vec<&dyn tauri::menu::IsMenuItem<Wry>> =
        colors.iter().map(|c| c as &dyn tauri::menu::IsMenuItem<Wry>).collect();
    let color_menu = Submenu::with_items(app, "Rotulador", true, &color_refs)?;
    let sound = CheckMenuItem::with_id(app, "sound", "Sonido", true, cfg.sound, None::<&str>)?;
    let undo = MenuItem::with_id(app, "undo", "Deshacer último", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Ajustes…", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Salir", true, None::<&str>)?;
    let sep = || PredefinedMenuItem::separator(app);
    let menu = Menu::with_items(
        app,
        &[&enabled, &color_menu, &sound, &sep()?, &undo, &settings, &sep()?, &quit],
    )?;

    TrayIconBuilder::with_id("main")
        .icon(app.default_window_icon().cloned().expect("bundle icon"))
        .tooltip("Subrayador")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "enabled" => update_config(app, |c| c.enabled = !c.enabled),
            "sound" => update_config(app, |c| c.sound = !c.sound),
            "undo" => undo_last(app),
            "settings" => open_settings(app),
            "quit" => app.exit(0),
            id => {
                if let Some(color) = id.strip_prefix("color:") {
                    let color = color.to_string();
                    update_config(app, |c| c.color = color);
                }
            }
        })
        .build(app)?;

    app.manage(TrayItems { enabled, sound, colors });
    Ok(())
}

pub fn run() {
    let config = Arc::new(RwLock::new(Config::load()));

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| open_settings(app)))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() != ShortcutState::Pressed {
                        return;
                    }
                    let mods = Modifiers::CONTROL | Modifiers::ALT;
                    if let Some(i) = COLOR_KEYS.iter().position(|k| shortcut.matches(mods, *k)) {
                        update_config(app, |c| c.color = mdmark::COLORS[i].to_string());
                    }
                })
                .build(),
        )
        .setup(move |app| {
            let handle = app.handle().clone();
            let cfg = config.read().expect("config lock").clone();

            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            build_tray(&handle, &cfg)?;
            for key in COLOR_KEYS {
                let _ = app
                    .global_shortcut()
                    .register(Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), key));
            }
            if let Err(e) = overlay::create(&handle) {
                eprintln!("overlay: {e}");
            }

            let sound = Sound::spawn();
            let undo: Undo = Arc::new(Mutex::new(None));
            let (tx, rx) = mpsc::channel();
            input::spawn(config.clone(), tx);
            highlight::spawn(handle.clone(), config.clone(), sound.clone(), undo.clone(), rx);
            app.manage(AppState { config: config.clone(), sound, undo });

            // First run: show the settings so the user can point at their notes.
            if locate::roots(&cfg.roots).is_empty() {
                open_settings(&handle);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            snapshot,
            save_config,
            install_style,
            copy_css,
            test_sound,
            set_autostart
        ])
        .build(tauri::generate_context!())
        .expect("error while building the app")
        .run(|_, event| {
            // Closing the settings window must not quit the tray app.
            if let RunEvent::ExitRequested { api, code: None, .. } = event {
                api.prevent_exit();
            }
        });
}
