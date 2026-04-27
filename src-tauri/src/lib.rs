use core_ai::{NoteBrain, NoteAnalysis};
use tauri::{State, Manager};
use tokio::sync::Mutex;

// This holds your "Private Brain" in memory
struct AppState {
    brain: Mutex<Option<NoteBrain>>,
}

#[tauri::command]
async fn analyze_note_content(content: String, state: State<'_, AppState>) -> Result<NoteAnalysis, String> {
    let mut brain_lock = state.brain.lock().await;
    
    // Lazy-load the model the first time it's needed
    if brain_lock.is_none() {
        // This will download the Llama 3.2 1B model (~1.3GB) on first run
        *brain_lock = Some(NoteBrain::new().await);
    }
    
    let analysis = brain_lock.as_ref().unwrap().analyze_note(&content).await;
    Ok(analysis)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // Register the state so Tauri can manage it
        .setup(|app| {
            app.manage(AppState { brain: Mutex::new(None) });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![analyze_note_content])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}