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
}

/** Matches Rust Project / BOINC PROJECT struct (subset). */
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
