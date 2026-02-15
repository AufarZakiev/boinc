/** Matches Rust TaskResult / BOINC RESULT struct. */
export interface TaskResult {
  name: string;
  wu_name: string;
  project_url: string;
  report_deadline: number;
  received_time: number;
  elapsed_time: number;
  estimated_cpu_time_remaining: number;
  fraction_done: number;
  state: number;
  scheduler_state: number;
  active_task_state: number;
  active_task: boolean;
  suspended_via_gui: boolean;
  project_suspended_via_gui: boolean;
  ready_to_report: boolean;
  got_server_ack: boolean;
  plan_class: string;
  resources: string;
  // Extended fields (Phase 2)
  version_num: number;
  slot: number;
  pid: number;
  checkpoint_cpu_time: number;
  current_cpu_time: number;
  progress_rate: number;
  working_set_size_smoothed: number;
  swap_size: number;
  slot_path: string;
  graphics_exec_path: string;
  web_graphics_url: string;
  remote_desktop_addr: string;
}

/** A GUI URL associated with a project. */
export interface GuiUrl {
  name: string;
  description: string;
  url: string;
}

/** Matches Rust Project / BOINC PROJECT struct. */
export interface Project {
  master_url: string;
  project_name: string;
  user_name: string;
  team_name: string;
  user_total_credit: number;
  user_expavg_credit: number;
  host_total_credit: number;
  host_expavg_credit: number;
  suspended_via_gui: boolean;
  dont_request_more_work: boolean;
  // Extended fields (Phase 2)
  resource_share: number;
  hostid: number;
  disk_usage: number;
  nrpc_failures: number;
  min_rpc_time: number;
  download_backoff: number;
  upload_backoff: number;
  sched_priority: number;
  duration_correction_factor: number;
  last_rpc_time: number;
  njobs_success: number;
  njobs_error: number;
  venue: string;
  gui_urls: GuiUrl[];
}

/** Matches Rust CcStatus / BOINC CC_STATUS struct. */
export interface CcStatus {
  task_mode: number;
  task_mode_perm: number;
  task_mode_delay: number;
  gpu_mode: number;
  gpu_mode_perm: number;
  gpu_mode_delay: number;
  network_mode: number;
  network_mode_perm: number;
  network_mode_delay: number;
  network_status: number;
  // Suspend reasons (Phase 3)
  task_suspend_reason: number;
  gpu_suspend_reason: number;
  network_suspend_reason: number;
}

/** Matches Rust FileTransfer / BOINC FILE_TRANSFER struct. */
export interface FileTransfer {
  project_url: string;
  project_name: string;
  name: string;
  nbytes: number;
  status: number;
  bytes_xferred: number;
  xfer_speed: number;
  is_upload: boolean;
}

/** BOINC run mode codes (from common_defs.h). */
export const RUN_MODE = {
  ALWAYS: 1,
  AUTO: 2,
  NEVER: 3,
} as const;

export type RunMode = (typeof RUN_MODE)[keyof typeof RUN_MODE];

export type ConnectionState =
  | "Disconnected"
  | "Connecting"
  | "Connected"
  | "AuthError"
  | { Error: string };

/** BOINC result state codes (from common_defs.h). */
export const RESULT_STATE = {
  NEW: 0,
  FILES_DOWNLOADING: 1,
  FILES_DOWNLOADED: 2,
  COMPUTE_ERROR: 3,
  FILES_UPLOADING: 4,
  FILES_UPLOADED: 5,
  ABORTED: 6,
  UPLOAD_FAILED: 7,
} as const;

/** BOINC active task state codes. */
export const ACTIVE_TASK_STATE = {
  UNINITIALIZED: 0,
  EXECUTING: 1,
  SUSPENDED: 9,
  ABORT_PENDING: 5,
  QUIT_PENDING: 8,
  COPY_PENDING: 10,
} as const;

/** BOINC scheduler state codes. */
export const SCHEDULER_STATE = {
  UNINITIALIZED: 0,
  PREEMPTED: 1,
  SCHEDULED: 2,
} as const;

/** Suspend reason bitmask values (from common_defs.h). */
export const SUSPEND_REASON = {
  BATTERIES: 1,
  USER_ACTIVE: 2,
  USER_REQ: 4,
  TIME_OF_DAY: 8,
  BENCHMARKS: 16,
  DISK_SIZE: 32,
  CPU_THROTTLE: 64,
  NO_RECENT_INPUT: 128,
  INITIAL_DELAY: 256,
  EXCLUSIVE_APP: 512,
  CPU_USAGE: 1024,
  NETWORK_QUOTA: 2048,
  OS: 4096,
} as const;

/** A single day's statistics for a project. */
export interface DailyStats {
  day: number;
  user_total_credit: number;
  user_expavg_credit: number;
  host_total_credit: number;
  host_expavg_credit: number;
}

/** Statistics for a single project. */
export interface ProjectStatistics {
  master_url: string;
  daily_statistics: DailyStats[];
}

/** A BOINC message (event log entry). */
export interface Message {
  project: string;
  priority: number;
  seqno: number;
  body: string;
  timestamp: number;
}

/** Message priority levels. */
export const MSG_PRIORITY = {
  INFO: 1,
  USER_ALERT: 2,
  INTERNAL_ERROR: 3,
} as const;

/** A BOINC notice. */
export interface Notice {
  seqno: number;
  title: string;
  description: string;
  create_time: number;
  project_name: string;
  link: string;
  category: string;
  is_private: boolean;
}

/** Disk usage for a single project. */
export interface DiskUsageProject {
  master_url: string;
  disk_usage: number;
}

/** Overall disk usage summary. */
export interface DiskUsage {
  projects: DiskUsageProject[];
  d_total: number;
  d_free: number;
  d_boinc: number;
  d_allowed: number;
}

/** Per-day-of-week preferences. */
export interface DayOfWeekPrefs {
  day_of_week: number;
  start_hour: number;
  end_hour: number;
  net_start_hour: number;
  net_end_hour: number;
}

/** Global preferences. */
export interface GlobalPreferences {
  run_on_batteries: boolean;
  run_if_user_active: boolean;
  idle_time_to_run: number;
  max_ncpus_pct: number;
  cpu_usage_limit: number;
  ram_max_used_busy_frac: number;
  ram_max_used_idle_frac: number;
  max_bytes_sec_down: number;
  max_bytes_sec_up: number;
  daily_xfer_limit_mb: number;
  disk_max_used_gb: number;
  disk_max_used_pct: number;
  disk_min_free_gb: number;
  work_buf_min_days: number;
  cpu_scheduling_period_minutes: number;
  start_hour: number;
  end_hour: number;
  net_start_hour: number;
  net_end_hour: number;
  // Enhanced preferences (Phase 5)
  suspend_if_no_recent_input: number;
  suspend_cpu_usage: number;
  leave_apps_in_memory: boolean;
  work_buf_additional_days: number;
  day_prefs: DayOfWeekPrefs[];
}

/** Host information. */
export interface HostInfo {
  domain_name: string;
  ip_addr: string;
  p_ncpus: number;
  p_vendor: string;
  p_model: string;
  p_fpops: number;
  p_iops: number;
  m_nbytes: number;
  m_cache: number;
  m_swap: number;
  d_total: number;
  d_free: number;
  os_name: string;
  os_version: string;
  product_name: string;
  virtualbox_version: string;
}

/** Entry in the all-projects list. */
export interface ProjectListEntry {
  name: string;
  url: string;
  general_area: string;
  specific_area: string;
  description: string;
  home: string;
  platforms: string[];
}

/** Result of an account lookup. */
export interface AccountOut {
  error_num: number;
  authenticator: string;
  error_msg: string;
}

/** Result of a project attach operation. */
export interface ProjectAttachReply {
  error_num: number;
  messages: string[];
}

/** Project configuration (Phase 4). */
export interface ProjectConfig {
  error_num: number;
  name: string;
  master_url: string;
  min_passwd_length: number;
  account_creation_disabled: boolean;
  client_account_creation_disabled: boolean;
  uses_username: boolean;
  terms_of_use: string;
  terms_of_use_is_html: boolean;
  ldap_auth: boolean;
  platforms: string[];
  sched_stopped: boolean;
  web_stopped: boolean;
}

/** Account manager info (Phase 4). */
export interface AcctMgrInfo {
  acct_mgr_name: string;
  acct_mgr_url: string;
  have_credentials: boolean;
}

/** Account manager RPC reply (Phase 4). */
export interface AcctMgrRpcReply {
  error_num: number;
  messages: string[];
}

/** Proxy settings (Phase 5). */
export interface ProxyInfo {
  use_http_proxy: boolean;
  http_server_name: string;
  http_server_port: number;
  http_user_name: string;
  http_user_passwd: string;
  use_http_auth: boolean;
  use_socks_proxy: boolean;
  socks_server_name: string;
  socks_server_port: number;
  socks5_user_name: string;
  socks5_user_passwd: string;
  socks5_remote_dns: boolean;
  noproxy_hosts: string;
}

/** Log flags (Phase 5). */
export interface LogFlags {
  task: boolean;
  file_xfer: boolean;
  sched_ops: boolean;
  cpu_sched: boolean;
  network_xfer: boolean;
  mem_usage: boolean;
  disk_usage: boolean;
  http_debug: boolean;
  state_debug: boolean;
  statefile_debug: boolean;
}

/** CC Config (Phase 5). */
export interface CcConfig {
  exclusive_apps: string[];
  exclusive_gpu_apps: string[];
  log_flags: LogFlags;
  max_file_xfers: number;
  max_ncpus: number;
  report_results_immediately: boolean;
}

/** Newer version info (Phase 6). */
export interface NewerVersionInfo {
  newer_version: string;
  download_url: string;
}
