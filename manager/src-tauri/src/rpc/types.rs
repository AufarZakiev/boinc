use serde::Serialize;

/// Matches BOINC's RESULT struct — represents a task/work unit result.
#[derive(Debug, Clone, Serialize, Default)]
pub struct TaskResult {
    pub name: String,
    pub wu_name: String,
    pub project_url: String,
    pub report_deadline: f64,
    pub received_time: f64,
    pub elapsed_time: f64,
    pub estimated_cpu_time_remaining: f64,
    pub fraction_done: f64,
    pub state: i32,
    pub scheduler_state: i32,
    pub active_task_state: i32,
    pub active_task: bool,
    pub suspended_via_gui: bool,
    pub project_suspended_via_gui: bool,
    pub ready_to_report: bool,
    pub got_server_ack: bool,
    pub plan_class: String,
    pub resources: String,
}

/// Matches BOINC's PROJECT struct (subset needed for display).
#[derive(Debug, Clone, Serialize, Default)]
pub struct Project {
    pub master_url: String,
    pub project_name: String,
    pub user_name: String,
    pub team_name: String,
    pub user_total_credit: f64,
    pub user_expavg_credit: f64,
    pub host_total_credit: f64,
    pub host_expavg_credit: f64,
    pub suspended_via_gui: bool,
    pub dont_request_more_work: bool,
}

/// Matches BOINC's CC_STATUS struct.
#[derive(Debug, Clone, Serialize, Default)]
pub struct CcStatus {
    pub task_mode: i32,
    pub task_mode_perm: i32,
    pub task_mode_delay: f64,
    pub gpu_mode: i32,
    pub gpu_mode_perm: i32,
    pub gpu_mode_delay: f64,
    pub network_mode: i32,
    pub network_mode_perm: i32,
    pub network_mode_delay: f64,
    pub network_status: i32,
}

/// State of the RPC connection.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    AuthError,
    Error(String),
}
