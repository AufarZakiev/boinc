mod rpc;

use rpc::{
    AccountOut, AcctMgrInfo, AcctMgrRpcReply, CcConfig, CcStatus, ConnectionState, DiskUsage,
    FileTransfer, GlobalPreferences, HostInfo, Message, NewerVersionInfo, Notice, Project,
    ProjectAttachReply, ProjectConfig, ProjectListEntry, ProjectStatistics, ProxyInfo, RpcClient,
    TaskResult,
};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

struct AppState {
    client: Arc<Mutex<Option<RpcClient>>>,
}

/// Helper macro to reduce boilerplate for commands that just get the client and call a method.
macro_rules! with_client {
    ($state:expr) => {{
        let guard = $state.client.lock().await;
        guard.as_ref().ok_or("Not connected".to_string())?.clone()
    }};
}

// We need RpcClient to be cloneable for the macro. Since it's behind Arc<Mutex>,
// we just use the guard pattern directly.

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

// ── Statistics ──────────────────────────────────────────────────

#[tauri::command]
async fn get_statistics(
    state: State<'_, AppState>,
) -> Result<Vec<ProjectStatistics>, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_statistics().await
}

// ── Messages ────────────────────────────────────────────────────

#[tauri::command]
async fn get_messages(
    state: State<'_, AppState>,
    seqno: i32,
) -> Result<Vec<Message>, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_messages(seqno).await
}

// ── Notices ─────────────────────────────────────────────────────

#[tauri::command]
async fn get_notices(
    state: State<'_, AppState>,
    seqno: i32,
) -> Result<Vec<Notice>, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_notices(seqno).await
}

// ── Disk usage ──────────────────────────────────────────────────

#[tauri::command]
async fn get_disk_usage(state: State<'_, AppState>) -> Result<DiskUsage, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_disk_usage().await
}

// ── Preferences ─────────────────────────────────────────────────

#[tauri::command]
async fn get_preferences(
    state: State<'_, AppState>,
) -> Result<GlobalPreferences, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_global_prefs_override().await
}

#[tauri::command]
async fn set_preferences(
    state: State<'_, AppState>,
    prefs: GlobalPreferences,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.set_global_prefs_override(&prefs).await
}

// ── Host info ───────────────────────────────────────────────────

#[tauri::command]
async fn get_host_info(state: State<'_, AppState>) -> Result<HostInfo, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_host_info().await
}

// ── Project attach ──────────────────────────────────────────────

#[tauri::command]
async fn get_all_projects_list(
    state: State<'_, AppState>,
) -> Result<Vec<ProjectListEntry>, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_all_projects_list().await
}

#[tauri::command]
async fn lookup_account(
    state: State<'_, AppState>,
    url: String,
    email: String,
    password: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.lookup_account(&url, &email, &password).await
}

#[tauri::command]
async fn lookup_account_poll(
    state: State<'_, AppState>,
) -> Result<AccountOut, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.lookup_account_poll().await
}

#[tauri::command]
async fn project_attach(
    state: State<'_, AppState>,
    url: String,
    authenticator: String,
    name: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.project_attach(&url, &authenticator, &name).await
}

#[tauri::command]
async fn project_attach_poll(
    state: State<'_, AppState>,
) -> Result<ProjectAttachReply, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.project_attach_poll().await
}

// ── Project config (Phase 4) ────────────────────────────────────

#[tauri::command]
async fn get_project_config(
    state: State<'_, AppState>,
    url: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_project_config(&url).await
}

#[tauri::command]
async fn get_project_config_poll(
    state: State<'_, AppState>,
) -> Result<ProjectConfig, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_project_config_poll().await
}

#[tauri::command]
async fn create_account(
    state: State<'_, AppState>,
    url: String,
    email: String,
    passwd_hash: String,
    user_name: String,
    team_name: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client
        .create_account(&url, &email, &passwd_hash, &user_name, &team_name)
        .await
}

#[tauri::command]
async fn create_account_poll(
    state: State<'_, AppState>,
) -> Result<AccountOut, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.create_account_poll().await
}

#[tauri::command]
fn compute_passwd_hash(email: String, password: String) -> String {
    use md5::{Digest, Md5};
    let mut hasher = Md5::new();
    hasher.update(password.as_bytes());
    hasher.update(email.to_lowercase().as_bytes());
    hex::encode(hasher.finalize())
}

// ── Account manager (Phase 4) ──────────────────────────────────

#[tauri::command]
async fn acct_mgr_info(
    state: State<'_, AppState>,
) -> Result<AcctMgrInfo, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.acct_mgr_info().await
}

#[tauri::command]
async fn acct_mgr_rpc(
    state: State<'_, AppState>,
    url: String,
    name: String,
    password: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.acct_mgr_rpc(&url, &name, &password).await
}

#[tauri::command]
async fn acct_mgr_rpc_poll(
    state: State<'_, AppState>,
) -> Result<AcctMgrRpcReply, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.acct_mgr_rpc_poll().await
}

// ── Proxy settings (Phase 5) ────────────────────────────────────

#[tauri::command]
async fn get_proxy_settings(
    state: State<'_, AppState>,
) -> Result<ProxyInfo, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_proxy_settings().await
}

#[tauri::command]
async fn set_proxy_settings(
    state: State<'_, AppState>,
    proxy: ProxyInfo,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.set_proxy_settings(&proxy).await
}

// ── CC Config (Phase 5) ─────────────────────────────────────────

#[tauri::command]
async fn get_cc_config(
    state: State<'_, AppState>,
) -> Result<CcConfig, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_cc_config().await
}

#[tauri::command]
async fn set_cc_config(
    state: State<'_, AppState>,
    config: CcConfig,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.set_cc_config(&config).await
}

// ── Version check (Phase 6) ─────────────────────────────────────

#[tauri::command]
async fn get_newer_version(
    state: State<'_, AppState>,
) -> Result<NewerVersionInfo, String> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().ok_or("Not connected")?;
    client.get_newer_version().await
}

// ── Graphics launcher (Phase 7) ─────────────────────────────────

#[tauri::command]
fn launch_graphics(path: String) -> Result<(), String> {
    std::process::Command::new(&path)
        .spawn()
        .map_err(|e| format!("Failed to launch graphics: {e}"))?;
    Ok(())
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
            get_statistics,
            get_messages,
            get_notices,
            get_disk_usage,
            get_preferences,
            set_preferences,
            get_host_info,
            get_all_projects_list,
            lookup_account,
            lookup_account_poll,
            project_attach,
            project_attach_poll,
            // Phase 4
            get_project_config,
            get_project_config_poll,
            create_account,
            create_account_poll,
            compute_passwd_hash,
            acct_mgr_info,
            acct_mgr_rpc,
            acct_mgr_rpc_poll,
            // Phase 5
            get_proxy_settings,
            set_proxy_settings,
            get_cc_config,
            set_cc_config,
            // Phase 6
            get_newer_version,
            // Phase 7
            launch_graphics,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
