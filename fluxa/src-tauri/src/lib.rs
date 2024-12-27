use flux_core::Flux;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::thread;
use tauri::{AppHandle, Emitter, Manager};

pub type FluxState = Mutex<Flux>;

#[tauri::command]
fn greet(app: AppHandle, name: &str) -> String {
    let state = app.state::<FluxState>();
    let mut flux = state.lock().unwrap();
    flux.memory.set_current_url(name.to_string());
    println!("Hello, {}!", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (tx, rx): (Sender<()>, _) = channel();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let tx_clone = tx.clone();
            app.manage(Mutex::new(Flux::new(tx_clone)));
            thread::spawn(move || {
                for _ in rx {
                    app_handle
                        .emit(
                            "memory-changed",
                            &app_handle.state::<FluxState>().lock().unwrap().memory,
                        )
                        .unwrap();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Erreur lors de l'exécution de l'application Tauri");
}
