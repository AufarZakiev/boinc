mod rpc;

use rpc::{CcStatus, ConnectionState, FileTransfer, Project, RpcClient, TaskResult};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

struct AppState {
    client: Arc<Mutex<Option<RpcClient>>>,
}

#[tauri::command]
async fn connect(
    state: State<'_, AppState>,
    host: String,
    port: u16,
    password: String,
) -> Result<(), String> {
    let client = RpcClient::new(&host, port);
    client.connect(&password).await?;
    let mut guard = state.client.lock().await;
    *guard = Some(client);
    Ok(())
}

#[tauri::command]
async fn connect_local(
    state: State<'_, AppState>,
    data_dir: String,
) -> Result<(), String> {
    let password = rpc::auth::read_password_from_file(&data_dir).unwrap_or_default();
    let client = RpcClient::localhost();
    client.connect(&password).await?;
    let mut guard = state.client.lock().await;
    *guard = Some(client);
    Ok(())
}

#[tauri::command]
async fn disconnect(state: State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.client.lock().await;
    if let Some(client) = guard.take() {
        client.disconnect().await;
    }
    Ok(())
}

#[tauri::command]
async fn get_connection_state(state: State<'_, AppState>) -> Result<ConnectionState, String> {
    let guard = state.client.lock().await;
    match guard.as_ref() {
        Some(client) => Ok(client.connection_state().await),
        None => Ok(ConnectionState::Disconnected),
    }
}

#[tauri::command]
async fn get_results(
    state: State<'_, AppState>,
    active_only: bool,
) -> Result<Vec<TaskResult>, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_results(active_only).await
}

#[tauri::command]
async fn get_project_status(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_project_status().await
}

#[tauri::command]
async fn get_cc_status(state: State<'_, AppState>) -> Result<CcStatus, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_cc_status().await
}

#[tauri::command]
async fn get_transfers(state: State<'_, AppState>) -> Result<Vec<FileTransfer>, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_file_transfers().await
}

// ── Task actions ─────────────────────────────────────────────────

#[tauri::command]
async fn suspend_task(
    state: State<'_, AppState>,
    project_url: String,
    name: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.suspend_result(&project_url, &name).await
}

#[tauri::command]
async fn resume_task(
    state: State<'_, AppState>,
    project_url: String,
    name: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.resume_result(&project_url, &name).await
}

#[tauri::command]
async fn abort_task(
    state: State<'_, AppState>,
    project_url: String,
    name: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.abort_result(&project_url, &name).await
}

// ── Project actions ──────────────────────────────────────────────

#[tauri::command]
async fn suspend_project(
    state: State<'_, AppState>,
    project_url: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.project_suspend(&project_url).await
}

#[tauri::command]
async fn resume_project(
    state: State<'_, AppState>,
    project_url: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.project_resume(&project_url).await
}

#[tauri::command]
async fn update_project(
    state: State<'_, AppState>,
    project_url: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.project_update(&project_url).await
}

#[tauri::command]
async fn no_new_tasks_project(
    state: State<'_, AppState>,
    project_url: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.project_nomorework(&project_url).await
}

#[tauri::command]
async fn allow_new_tasks_project(
    state: State<'_, AppState>,
    project_url: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.project_allowmorework(&project_url).await
}

#[tauri::command]
async fn reset_project(
    state: State<'_, AppState>,
    project_url: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.project_reset(&project_url).await
}

#[tauri::command]
async fn detach_project(
    state: State<'_, AppState>,
    project_url: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.project_detach(&project_url).await
}

// ── Mode controls ────────────────────────────────────────────────

#[tauri::command]
async fn set_run_mode(
    state: State<'_, AppState>,
    mode: i32,
    duration: f64,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.set_run_mode(mode, duration).await
}

#[tauri::command]
async fn set_gpu_mode(
    state: State<'_, AppState>,
    mode: i32,
    duration: f64,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.set_gpu_mode(mode, duration).await
}

#[tauri::command]
async fn set_network_mode(
    state: State<'_, AppState>,
    mode: i32,
    duration: f64,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.set_network_mode(mode, duration).await
}

// ── Transfer actions ─────────────────────────────────────────────

#[tauri::command]
async fn retry_transfer(
    state: State<'_, AppState>,
    project_url: String,
    filename: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.retry_file_transfer(&project_url, &filename).await
}

#[tauri::command]
async fn abort_transfer(
    state: State<'_, AppState>,
    project_url: String,
    filename: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.abort_file_transfer(&project_url, &filename).await
}

// ── Other ────────────────────────────────────────────────────────

#[tauri::command]
async fn run_benchmarks(state: State<'_, AppState>) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.run_benchmarks().await
}

#[tauri::command]
async fn retry_pending_transfers(state: State<'_, AppState>) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.network_available().await
}

#[tauri::command]
async fn shutdown_client(state: State<'_, AppState>) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.quit().await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            client: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            connect,
            connect_local,
            disconnect,
            get_connection_state,
            get_results,
            get_project_status,
            get_cc_status,
            get_transfers,
            suspend_task,
            resume_task,
            abort_task,
            suspend_project,
            resume_project,
            update_project,
            no_new_tasks_project,
            allow_new_tasks_project,
            reset_project,
            detach_project,
            set_run_mode,
            set_gpu_mode,
            set_network_mode,
            retry_transfer,
            abort_transfer,
            run_benchmarks,
            retry_pending_transfers,
            shutdown_client,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
