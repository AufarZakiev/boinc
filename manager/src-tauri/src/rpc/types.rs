use serde::{Deserialize, Serialize};

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

/// Matches BOINC's FILE_TRANSFER struct.
#[derive(Debug, Clone, Serialize, Default)]
pub struct FileTransfer {
    pub project_url: String,
    pub project_name: String,
    pub name: String,
    pub nbytes: f64,
    pub status: i32,
    pub bytes_xferred: f64,
    pub xfer_speed: f64,
    pub is_upload: bool,
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

/// A single day's statistics for a project.
#[derive(Debug, Clone, Serialize, Default)]
pub struct DailyStats {
    pub day: f64,
    pub user_total_credit: f64,
    pub user_expavg_credit: f64,
    pub host_total_credit: f64,
    pub host_expavg_credit: f64,
}

/// Statistics for a single project, containing daily data points.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ProjectStatistics {
    pub master_url: String,
    pub daily_statistics: Vec<DailyStats>,
}

/// A BOINC message (event log entry).
#[derive(Debug, Clone, Serialize, Default)]
pub struct Message {
    pub project: String,
    pub priority: i32,
    pub seqno: i32,
    pub body: String,
    pub timestamp: f64,
}

/// A BOINC notice.
#[derive(Debug, Clone, Serialize, Default)]
pub struct Notice {
    pub seqno: i32,
    pub title: String,
    pub description: String,
    pub create_time: f64,
    pub project_name: String,
    pub link: String,
    pub category: String,
    pub is_private: bool,
}

/// Disk usage for a single project.
#[derive(Debug, Clone, Serialize, Default)]
pub struct DiskUsageProject {
    pub master_url: String,
    pub disk_usage: f64,
}

/// Overall disk usage summary.
#[derive(Debug, Clone, Serialize, Default)]
pub struct DiskUsage {
    pub projects: Vec<DiskUsageProject>,
    pub d_total: f64,
    pub d_free: f64,
    pub d_boinc: f64,
    pub d_allowed: f64,
}

/// Global preferences (computing/network/storage settings).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalPreferences {
    pub run_on_batteries: bool,
    pub run_if_user_active: bool,
    pub idle_time_to_run: f64,
    pub max_ncpus_pct: f64,
    pub cpu_usage_limit: f64,
    pub ram_max_used_busy_frac: f64,
    pub ram_max_used_idle_frac: f64,
    pub max_bytes_sec_down: f64,
    pub max_bytes_sec_up: f64,
    pub daily_xfer_limit_mb: f64,
    pub disk_max_used_gb: f64,
    pub disk_max_used_pct: f64,
    pub disk_min_free_gb: f64,
    pub work_buf_min_days: f64,
    pub cpu_scheduling_period_minutes: f64,
    pub start_hour: f64,
    pub end_hour: f64,
    pub net_start_hour: f64,
    pub net_end_hour: f64,
}

/// Host information.
#[derive(Debug, Clone, Serialize, Default)]
pub struct HostInfo {
    pub domain_name: String,
    pub ip_addr: String,
    pub p_ncpus: i32,
    pub p_vendor: String,
    pub p_model: String,
    pub p_fpops: f64,
    pub p_iops: f64,
    pub m_nbytes: f64,
    pub m_cache: f64,
    pub m_swap: f64,
    pub d_total: f64,
    pub d_free: f64,
    pub os_name: String,
    pub os_version: String,
    pub product_name: String,
    pub virtualbox_version: String,
}

/// Entry in the all-projects list.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ProjectListEntry {
    pub name: String,
    pub url: String,
    pub general_area: String,
    pub specific_area: String,
    pub description: String,
    pub home: String,
    pub platforms: Vec<String>,
}

/// Result of an account lookup (authenticator or error).
#[derive(Debug, Clone, Serialize, Default)]
pub struct AccountOut {
    pub error_num: i32,
    pub authenticator: String,
    pub error_msg: String,
}

/// Result of a project attach operation.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ProjectAttachReply {
    pub error_num: i32,
    pub messages: Vec<String>,
}
