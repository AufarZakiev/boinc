use quick_xml::events::Event;
use quick_xml::Reader;

use super::types::{
    AccountOut, CcStatus, DailyStats, DiskUsage, DiskUsageProject, FileTransfer, GlobalPreferences,
    HostInfo, Message, Notice, Project, ProjectAttachReply, ProjectListEntry, ProjectStatistics,
    TaskResult,
};

/// Extract text content of an XML element, advancing the reader past its end tag.
/// Handles both regular text and CDATA sections.
fn read_text(reader: &mut Reader<&[u8]>) -> String {
    let mut buf = Vec::new();
    let mut text = String::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => {
                text.push_str(&e.unescape().unwrap_or_default());
            }
            Ok(Event::CData(e)) => {
                if let Ok(s) = std::str::from_utf8(&e) {
                    text.push_str(s);
                }
            }
            Ok(Event::End(_)) | Ok(Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }
    text
}

/// Parse the `<results>` section from a `get_results` or `get_state` response.
pub fn parse_results(xml: &str) -> Vec<TaskResult> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut results = Vec::new();
    let mut in_result = false;
    let mut in_active_task = false;
    let mut current = TaskResult::default();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "result" => {
                        in_result = true;
                        current = TaskResult::default();
                    }
                    "active_task" if in_result => {
                        in_active_task = true;
                        current.active_task = true;
                    }
                    _ if in_result => {
                        let text = read_text(&mut reader);
                        if in_active_task {
                            match tag.as_str() {
                                "active_task_state" => {
                                    current.active_task_state =
                                        text.parse().unwrap_or(0);
                                }
                                "scheduler_state" => {
                                    current.scheduler_state =
                                        text.parse().unwrap_or(0);
                                }
                                "elapsed_time" => {
                                    current.elapsed_time =
                                        text.parse().unwrap_or(0.0);
                                }
                                "fraction_done" => {
                                    current.fraction_done =
                                        text.parse().unwrap_or(0.0);
                                }
                                _ => {}
                            }
                        } else {
                            match tag.as_str() {
                                "name" => current.name = text,
                                "wu_name" => current.wu_name = text,
                                "project_url" => current.project_url = text,
                                "report_deadline" => {
                                    current.report_deadline =
                                        text.parse().unwrap_or(0.0);
                                }
                                "received_time" => {
                                    current.received_time =
                                        text.parse().unwrap_or(0.0);
                                }
                                "estimated_cpu_time_remaining" => {
                                    current.estimated_cpu_time_remaining =
                                        text.parse().unwrap_or(0.0);
                                }
                                "state" => {
                                    current.state = text.parse().unwrap_or(0);
                                }
                                "scheduler_state" => {
                                    current.scheduler_state =
                                        text.parse().unwrap_or(0);
                                }
                                "suspended_via_gui" => {
                                    current.suspended_via_gui = true;
                                }
                                "project_suspended_via_gui" => {
                                    current.project_suspended_via_gui = true;
                                }
                                "ready_to_report" => {
                                    current.ready_to_report = true;
                                }
                                "got_server_ack" => {
                                    current.got_server_ack = true;
                                }
                                "plan_class" => current.plan_class = text,
                                "resources" => current.resources = text,
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(ref e)) if in_result => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "active_task" => current.active_task = true,
                    "suspended_via_gui" => current.suspended_via_gui = true,
                    "project_suspended_via_gui" => {
                        current.project_suspended_via_gui = true;
                    }
                    "ready_to_report" => current.ready_to_report = true,
                    "got_server_ack" => current.got_server_ack = true,
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "result" => {
                        in_result = false;
                        in_active_task = false;
                        results.push(current.clone());
                    }
                    "active_task" => {
                        in_active_task = false;
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    results
}

/// Parse `<projects>` section from a `get_state` or `get_project_status` response.
pub fn parse_projects(xml: &str) -> Vec<Project> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut projects = Vec::new();
    let mut in_project = false;
    let mut current = Project::default();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "project" => {
                        in_project = true;
                        current = Project::default();
                    }
                    _ if in_project => {
                        let text = read_text(&mut reader);
                        match tag.as_str() {
                            "master_url" => current.master_url = text,
                            "project_name" => current.project_name = text,
                            "user_name" => current.user_name = text,
                            "team_name" => current.team_name = text,
                            "user_total_credit" => {
                                current.user_total_credit =
                                    text.parse().unwrap_or(0.0);
                            }
                            "user_expavg_credit" => {
                                current.user_expavg_credit =
                                    text.parse().unwrap_or(0.0);
                            }
                            "host_total_credit" => {
                                current.host_total_credit =
                                    text.parse().unwrap_or(0.0);
                            }
                            "host_expavg_credit" => {
                                current.host_expavg_credit =
                                    text.parse().unwrap_or(0.0);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(ref e)) if in_project => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "suspended_via_gui" => current.suspended_via_gui = true,
                    "dont_request_more_work" => {
                        current.dont_request_more_work = true;
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if tag == "project" {
                    in_project = false;
                    projects.push(current.clone());
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    projects
}

/// Parse `<cc_status>` from a `get_cc_status` response.
pub fn parse_cc_status(xml: &str) -> CcStatus {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut status = CcStatus::default();
    let mut in_status = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "cc_status" => {
                        in_status = true;
                    }
                    _ if in_status => {
                        let text = read_text(&mut reader);
                        match tag.as_str() {
                            "task_mode" => {
                                status.task_mode = text.parse().unwrap_or(0);
                            }
                            "task_mode_perm" => {
                                status.task_mode_perm = text.parse().unwrap_or(0);
                            }
                            "task_mode_delay" => {
                                status.task_mode_delay =
                                    text.parse().unwrap_or(0.0);
                            }
                            "gpu_mode" => {
                                status.gpu_mode = text.parse().unwrap_or(0);
                            }
                            "gpu_mode_perm" => {
                                status.gpu_mode_perm = text.parse().unwrap_or(0);
                            }
                            "gpu_mode_delay" => {
                                status.gpu_mode_delay =
                                    text.parse().unwrap_or(0.0);
                            }
                            "network_mode" => {
                                status.network_mode = text.parse().unwrap_or(0);
                            }
                            "network_mode_perm" => {
                                status.network_mode_perm =
                                    text.parse().unwrap_or(0);
                            }
                            "network_mode_delay" => {
                                status.network_mode_delay =
                                    text.parse().unwrap_or(0.0);
                            }
                            "network_status" => {
                                status.network_status = text.parse().unwrap_or(0);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if tag == "cc_status" {
                    in_status = false;
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    status
}

/// Check whether a response contains `<success/>`, returning an error if not.
pub fn parse_success(xml: &str) -> Result<(), String> {
    if xml.contains("<success/>") {
        Ok(())
    } else if xml.contains("<error>") {
        // Extract the error message
        let mut reader = Reader::from_str(xml);
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    if tag == "error" {
                        let text = read_text(&mut reader);
                        return Err(text);
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
            buf.clear();
        }
        Err("Unknown error".to_string())
    } else {
        Err("Unexpected response".to_string())
    }
}

/// Parse `<file_transfers>` from a `get_file_transfers` response.
pub fn parse_file_transfers(xml: &str) -> Vec<FileTransfer> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut transfers = Vec::new();
    let mut in_transfer = false;
    let mut in_file_xfer = false;
    let mut current = FileTransfer::default();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "file_transfer" => {
                        in_transfer = true;
                        current = FileTransfer::default();
                    }
                    "file_xfer" if in_transfer => {
                        in_file_xfer = true;
                    }
                    _ if in_transfer => {
                        let text = read_text(&mut reader);
                        if in_file_xfer {
                            match tag.as_str() {
                                "bytes_xferred" => {
                                    current.bytes_xferred =
                                        text.parse().unwrap_or(0.0);
                                }
                                "xfer_speed" => {
                                    current.xfer_speed =
                                        text.parse().unwrap_or(0.0);
                                }
                                _ => {}
                            }
                        } else {
                            match tag.as_str() {
                                "project_url" => current.project_url = text,
                                "project_name" => current.project_name = text,
                                "name" => current.name = text,
                                "nbytes" => {
                                    current.nbytes =
                                        text.parse().unwrap_or(0.0);
                                }
                                "status" => {
                                    current.status =
                                        text.parse().unwrap_or(0);
                                }
                                "is_upload" => {
                                    current.is_upload = true;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(ref e)) if in_transfer => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if tag == "is_upload" {
                    current.is_upload = true;
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "file_transfer" => {
                        in_transfer = false;
                        in_file_xfer = false;
                        transfers.push(current.clone());
                    }
                    "file_xfer" => {
                        in_file_xfer = false;
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    transfers
}

/// Parse `<statistics>` from a `get_statistics` response.
pub fn parse_statistics(xml: &str) -> Vec<ProjectStatistics> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut result = Vec::new();
    let mut in_project_statistics = false;
    let mut in_daily_statistics = false;
    let mut current_project = ProjectStatistics::default();
    let mut current_day = DailyStats::default();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "project_statistics" => {
                        in_project_statistics = true;
                        current_project = ProjectStatistics::default();
                    }
                    "daily_statistics" if in_project_statistics => {
                        in_daily_statistics = true;
                        current_day = DailyStats::default();
                    }
                    _ if in_daily_statistics => {
                        let text = read_text(&mut reader);
                        match tag.as_str() {
                            "day" => current_day.day = text.parse().unwrap_or(0.0),
                            "user_total_credit" => {
                                current_day.user_total_credit = text.parse().unwrap_or(0.0)
                            }
                            "user_expavg_credit" => {
                                current_day.user_expavg_credit = text.parse().unwrap_or(0.0)
                            }
                            "host_total_credit" => {
                                current_day.host_total_credit = text.parse().unwrap_or(0.0)
                            }
                            "host_expavg_credit" => {
                                current_day.host_expavg_credit = text.parse().unwrap_or(0.0)
                            }
                            _ => {}
                        }
                    }
                    _ if in_project_statistics => {
                        let text = read_text(&mut reader);
                        if tag == "master_url" {
                            current_project.master_url = text;
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "project_statistics" => {
                        in_project_statistics = false;
                        result.push(current_project.clone());
                    }
                    "daily_statistics" => {
                        in_daily_statistics = false;
                        current_project.daily_statistics.push(current_day.clone());
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    result
}

/// Parse `<msgs>` from a `get_messages` response.
pub fn parse_messages(xml: &str) -> Vec<Message> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut messages = Vec::new();
    let mut in_msg = false;
    let mut current = Message::default();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "msg" => {
                        in_msg = true;
                        current = Message::default();
                    }
                    _ if in_msg => {
                        let text = read_text(&mut reader);
                        match tag.as_str() {
                            "project" => current.project = text,
                            "pri" => current.priority = text.parse().unwrap_or(0),
                            "seqno" => current.seqno = text.parse().unwrap_or(0),
                            "body" => current.body = text.trim().to_string(),
                            "time" => current.timestamp = text.parse().unwrap_or(0.0),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if tag == "msg" {
                    in_msg = false;
                    messages.push(current.clone());
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    messages
}

/// Parse `<notices>` from a `get_notices` response.
pub fn parse_notices(xml: &str) -> Vec<Notice> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut notices = Vec::new();
    let mut in_notice = false;
    let mut current = Notice::default();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "notice" => {
                        in_notice = true;
                        current = Notice::default();
                    }
                    _ if in_notice => {
                        let text = read_text(&mut reader);
                        match tag.as_str() {
                            "seqno" => current.seqno = text.parse().unwrap_or(0),
                            "title" => current.title = text,
                            "description" => current.description = text,
                            "create_time" => {
                                current.create_time = text.parse().unwrap_or(0.0)
                            }
                            "project_name" => current.project_name = text,
                            "link" => current.link = text,
                            "category" => current.category = text,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(ref e)) if in_notice => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if tag == "is_private" {
                    current.is_private = true;
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if tag == "notice" {
                    in_notice = false;
                    notices.push(current.clone());
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    notices
}

/// Parse `<disk_usage_summary>` from a `get_disk_usage` response.
pub fn parse_disk_usage(xml: &str) -> DiskUsage {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut usage = DiskUsage::default();
    let mut in_summary = false;
    let mut in_project = false;
    let mut current_project = DiskUsageProject::default();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "disk_usage_summary" => {
                        in_summary = true;
                    }
                    "project" if in_summary => {
                        in_project = true;
                        current_project = DiskUsageProject::default();
                    }
                    _ if in_project => {
                        let text = read_text(&mut reader);
                        match tag.as_str() {
                            "master_url" => current_project.master_url = text,
                            "disk_usage" => {
                                current_project.disk_usage = text.parse().unwrap_or(0.0)
                            }
                            _ => {}
                        }
                    }
                    _ if in_summary => {
                        let text = read_text(&mut reader);
                        match tag.as_str() {
                            "d_total" => usage.d_total = text.parse().unwrap_or(0.0),
                            "d_free" => usage.d_free = text.parse().unwrap_or(0.0),
                            "d_boinc" => usage.d_boinc = text.parse().unwrap_or(0.0),
                            "d_allowed" => usage.d_allowed = text.parse().unwrap_or(0.0),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "project" if in_summary => {
                        in_project = false;
                        usage.projects.push(current_project.clone());
                    }
                    "disk_usage_summary" => {
                        in_summary = false;
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    usage
}

/// Parse `<global_preferences>` from a `get_global_prefs_override` response.
pub fn parse_global_preferences(xml: &str) -> GlobalPreferences {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut prefs = GlobalPreferences::default();
    let mut in_prefs = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "global_preferences" => {
                        in_prefs = true;
                    }
                    _ if in_prefs => {
                        let text = read_text(&mut reader);
                        match tag.as_str() {
                            "run_on_batteries" => {
                                prefs.run_on_batteries = text.parse::<i32>().unwrap_or(0) != 0
                            }
                            "run_if_user_active" => {
                                prefs.run_if_user_active = text.parse::<i32>().unwrap_or(0) != 0
                            }
                            "idle_time_to_run" => {
                                prefs.idle_time_to_run = text.parse().unwrap_or(0.0)
                            }
                            "max_ncpus_pct" => {
                                prefs.max_ncpus_pct = text.parse().unwrap_or(0.0)
                            }
                            "cpu_usage_limit" => {
                                prefs.cpu_usage_limit = text.parse().unwrap_or(0.0)
                            }
                            "ram_max_used_busy_frac" => {
                                prefs.ram_max_used_busy_frac = text.parse().unwrap_or(0.0)
                            }
                            "ram_max_used_idle_frac" => {
                                prefs.ram_max_used_idle_frac = text.parse().unwrap_or(0.0)
                            }
                            "max_bytes_sec_down" => {
                                prefs.max_bytes_sec_down = text.parse().unwrap_or(0.0)
                            }
                            "max_bytes_sec_up" => {
                                prefs.max_bytes_sec_up = text.parse().unwrap_or(0.0)
                            }
                            "daily_xfer_limit_mb" => {
                                prefs.daily_xfer_limit_mb = text.parse().unwrap_or(0.0)
                            }
                            "disk_max_used_gb" => {
                                prefs.disk_max_used_gb = text.parse().unwrap_or(0.0)
                            }
                            "disk_max_used_pct" => {
                                prefs.disk_max_used_pct = text.parse().unwrap_or(0.0)
                            }
                            "disk_min_free_gb" => {
                                prefs.disk_min_free_gb = text.parse().unwrap_or(0.0)
                            }
                            "work_buf_min_days" => {
                                prefs.work_buf_min_days = text.parse().unwrap_or(0.0)
                            }
                            "cpu_scheduling_period_minutes" => {
                                prefs.cpu_scheduling_period_minutes =
                                    text.parse().unwrap_or(0.0)
                            }
                            "start_hour" => {
                                prefs.start_hour = text.parse().unwrap_or(0.0)
                            }
                            "end_hour" => {
                                prefs.end_hour = text.parse().unwrap_or(0.0)
                            }
                            "net_start_hour" => {
                                prefs.net_start_hour = text.parse().unwrap_or(0.0)
                            }
                            "net_end_hour" => {
                                prefs.net_end_hour = text.parse().unwrap_or(0.0)
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(ref e)) if in_prefs => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "run_on_batteries" => prefs.run_on_batteries = true,
                    "run_if_user_active" => prefs.run_if_user_active = true,
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if tag == "global_preferences" {
                    in_prefs = false;
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    prefs
}

/// Serialize global preferences to XML for `set_global_prefs_override`.
pub fn serialize_global_preferences(prefs: &GlobalPreferences) -> String {
    format!(
        "<global_preferences>\n\
         <run_on_batteries>{}</run_on_batteries>\n\
         <run_if_user_active>{}</run_if_user_active>\n\
         <idle_time_to_run>{}</idle_time_to_run>\n\
         <max_ncpus_pct>{}</max_ncpus_pct>\n\
         <cpu_usage_limit>{}</cpu_usage_limit>\n\
         <ram_max_used_busy_frac>{}</ram_max_used_busy_frac>\n\
         <ram_max_used_idle_frac>{}</ram_max_used_idle_frac>\n\
         <max_bytes_sec_down>{}</max_bytes_sec_down>\n\
         <max_bytes_sec_up>{}</max_bytes_sec_up>\n\
         <daily_xfer_limit_mb>{}</daily_xfer_limit_mb>\n\
         <disk_max_used_gb>{}</disk_max_used_gb>\n\
         <disk_max_used_pct>{}</disk_max_used_pct>\n\
         <disk_min_free_gb>{}</disk_min_free_gb>\n\
         <work_buf_min_days>{}</work_buf_min_days>\n\
         <cpu_scheduling_period_minutes>{}</cpu_scheduling_period_minutes>\n\
         <start_hour>{}</start_hour>\n\
         <end_hour>{}</end_hour>\n\
         <net_start_hour>{}</net_start_hour>\n\
         <net_end_hour>{}</net_end_hour>\n\
         </global_preferences>",
        if prefs.run_on_batteries { 1 } else { 0 },
        if prefs.run_if_user_active { 1 } else { 0 },
        prefs.idle_time_to_run,
        prefs.max_ncpus_pct,
        prefs.cpu_usage_limit,
        prefs.ram_max_used_busy_frac,
        prefs.ram_max_used_idle_frac,
        prefs.max_bytes_sec_down,
        prefs.max_bytes_sec_up,
        prefs.daily_xfer_limit_mb,
        prefs.disk_max_used_gb,
        prefs.disk_max_used_pct,
        prefs.disk_min_free_gb,
        prefs.work_buf_min_days,
        prefs.cpu_scheduling_period_minutes,
        prefs.start_hour,
        prefs.end_hour,
        prefs.net_start_hour,
        prefs.net_end_hour,
    )
}

/// Parse `<host_info>` from a `get_host_info` response.
pub fn parse_host_info(xml: &str) -> HostInfo {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut info = HostInfo::default();
    let mut in_host_info = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "host_info" => {
                        in_host_info = true;
                    }
                    _ if in_host_info => {
                        let text = read_text(&mut reader);
                        match tag.as_str() {
                            "domain_name" => info.domain_name = text,
                            "ip_addr" => info.ip_addr = text,
                            "p_ncpus" => info.p_ncpus = text.parse().unwrap_or(0),
                            "p_vendor" => info.p_vendor = text,
                            "p_model" => info.p_model = text,
                            "p_fpops" => info.p_fpops = text.parse().unwrap_or(0.0),
                            "p_iops" => info.p_iops = text.parse().unwrap_or(0.0),
                            "m_nbytes" => info.m_nbytes = text.parse().unwrap_or(0.0),
                            "m_cache" => info.m_cache = text.parse().unwrap_or(0.0),
                            "m_swap" => info.m_swap = text.parse().unwrap_or(0.0),
                            "d_total" => info.d_total = text.parse().unwrap_or(0.0),
                            "d_free" => info.d_free = text.parse().unwrap_or(0.0),
                            "os_name" => info.os_name = text,
                            "os_version" => info.os_version = text,
                            "product_name" => info.product_name = text,
                            "virtualbox_version" => info.virtualbox_version = text,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if tag == "host_info" {
                    in_host_info = false;
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    info
}

/// Parse `<projects>` from a `get_all_projects_list` response.
pub fn parse_all_projects_list(xml: &str) -> Vec<ProjectListEntry> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut entries = Vec::new();
    let mut in_project = false;
    let mut in_platforms = false;
    let mut current = ProjectListEntry::default();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "project" => {
                        in_project = true;
                        current = ProjectListEntry::default();
                    }
                    "platforms" if in_project => {
                        in_platforms = true;
                    }
                    "name" if in_platforms => {
                        let text = read_text(&mut reader);
                        current.platforms.push(text);
                    }
                    _ if in_project && !in_platforms => {
                        let text = read_text(&mut reader);
                        match tag.as_str() {
                            "name" => current.name = text,
                            "url" => current.url = text,
                            "general_area" => current.general_area = text,
                            "specific_area" => current.specific_area = text,
                            "description" => current.description = text,
                            "home" => current.home = text,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "project" => {
                        in_project = false;
                        in_platforms = false;
                        entries.push(current.clone());
                    }
                    "platforms" => {
                        in_platforms = false;
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    entries
}

/// Parse account lookup poll result.
pub fn parse_account_out(xml: &str) -> AccountOut {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut out = AccountOut::default();
    let mut in_account_out = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "account_out" => {
                        in_account_out = true;
                    }
                    _ if in_account_out => {
                        let text = read_text(&mut reader);
                        match tag.as_str() {
                            "error_num" => out.error_num = text.parse().unwrap_or(0),
                            "authenticator" => out.authenticator = text,
                            "error_msg" => out.error_msg = text,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if tag == "account_out" {
                    in_account_out = false;
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    out
}

/// Parse project attach poll result.
pub fn parse_project_attach_reply(xml: &str) -> ProjectAttachReply {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut reply = ProjectAttachReply::default();
    let mut in_reply = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "project_attach_reply" => {
                        in_reply = true;
                    }
                    _ if in_reply => {
                        let text = read_text(&mut reader);
                        match tag.as_str() {
                            "error_num" => reply.error_num = text.parse().unwrap_or(0),
                            "message" => reply.messages.push(text),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if tag == "project_attach_reply" {
                    in_reply = false;
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    reply
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_results() {
        let xml = r#"
<boinc_gui_rpc_reply>
<results>
<result>
    <name>task_12345_0</name>
    <wu_name>task_12345</wu_name>
    <project_url>https://example.com/project/</project_url>
    <report_deadline>1700000000.000000</report_deadline>
    <received_time>1699900000.000000</received_time>
    <estimated_cpu_time_remaining>3600.500000</estimated_cpu_time_remaining>
    <state>2</state>
    <plan_class>sse2</plan_class>
    <resources>1 CPU</resources>
    <active_task>
        <active_task_state>1</active_task_state>
        <scheduler_state>2</scheduler_state>
        <elapsed_time>1200.300000</elapsed_time>
        <fraction_done>0.456000</fraction_done>
    </active_task>
</result>
<result>
    <name>task_67890_0</name>
    <wu_name>task_67890</wu_name>
    <project_url>https://example.com/project2/</project_url>
    <state>5</state>
    <ready_to_report/>
</result>
</results>
</boinc_gui_rpc_reply>"#;

        let results = parse_results(xml);
        assert_eq!(results.len(), 2);

        let r0 = &results[0];
        assert_eq!(r0.name, "task_12345_0");
        assert_eq!(r0.wu_name, "task_12345");
        assert_eq!(r0.project_url, "https://example.com/project/");
        assert!((r0.report_deadline - 1700000000.0).abs() < 1.0);
        assert!((r0.estimated_cpu_time_remaining - 3600.5).abs() < 0.1);
        assert_eq!(r0.state, 2);
        assert_eq!(r0.scheduler_state, 2);
        assert_eq!(r0.plan_class, "sse2");
        assert_eq!(r0.resources, "1 CPU");
        assert!(r0.active_task);
        assert_eq!(r0.active_task_state, 1);
        assert!((r0.elapsed_time - 1200.3).abs() < 0.1);
        assert!((r0.fraction_done - 0.456).abs() < 0.001);

        let r1 = &results[1];
        assert_eq!(r1.name, "task_67890_0");
        assert_eq!(r1.state, 5);
        assert!(r1.ready_to_report);
        assert!(!r1.active_task);
    }

    #[test]
    fn test_parse_projects() {
        let xml = r#"
<boinc_gui_rpc_reply>
<projects>
<project>
    <master_url>https://example.com/project/</master_url>
    <project_name>Example Project</project_name>
    <user_name>testuser</user_name>
    <team_name>Test Team</team_name>
    <user_total_credit>12345.670000</user_total_credit>
    <user_expavg_credit>100.500000</user_expavg_credit>
    <host_total_credit>5000.000000</host_total_credit>
    <host_expavg_credit>50.250000</host_expavg_credit>
    <suspended_via_gui/>
</project>
</projects>
</boinc_gui_rpc_reply>"#;

        let projects = parse_projects(xml);
        assert_eq!(projects.len(), 1);

        let p = &projects[0];
        assert_eq!(p.project_name, "Example Project");
        assert_eq!(p.user_name, "testuser");
        assert!((p.user_total_credit - 12345.67).abs() < 0.01);
        assert!(p.suspended_via_gui);
        assert!(!p.dont_request_more_work);
    }

    #[test]
    fn test_parse_cc_status() {
        let xml = r#"
<boinc_gui_rpc_reply>
<cc_status>
    <task_mode>2</task_mode>
    <task_mode_perm>2</task_mode_perm>
    <task_mode_delay>0.000000</task_mode_delay>
    <gpu_mode>2</gpu_mode>
    <gpu_mode_perm>2</gpu_mode_perm>
    <gpu_mode_delay>0.000000</gpu_mode_delay>
    <network_mode>2</network_mode>
    <network_mode_perm>2</network_mode_perm>
    <network_mode_delay>0.000000</network_mode_delay>
    <network_status>0</network_status>
</cc_status>
</boinc_gui_rpc_reply>"#;

        let status = parse_cc_status(xml);
        assert_eq!(status.task_mode, 2);
        assert_eq!(status.gpu_mode, 2);
        assert_eq!(status.network_mode, 2);
        assert_eq!(status.network_status, 0);
    }

    #[test]
    fn test_parse_scheduler_state_inside_active_task() {
        // Regression: scheduler_state is sent inside <active_task> by the
        // BOINC client. Previously it was only parsed at the result level,
        // so running tasks showed "Waiting to run" instead of "Running".
        let xml = r#"
<boinc_gui_rpc_reply>
<results>
<result>
    <name>running_task_0</name>
    <wu_name>running_task</wu_name>
    <project_url>https://example.com/</project_url>
    <state>2</state>
    <active_task>
        <active_task_state>1</active_task_state>
        <scheduler_state>2</scheduler_state>
        <elapsed_time>500.000000</elapsed_time>
        <fraction_done>0.500000</fraction_done>
    </active_task>
</result>
</results>
</boinc_gui_rpc_reply>"#;

        let results = parse_results(xml);
        assert_eq!(results.len(), 1);

        let r = &results[0];
        assert!(r.active_task);
        assert_eq!(r.active_task_state, 1); // EXECUTING
        assert_eq!(r.scheduler_state, 2); // SCHEDULED — the key assertion
    }

    #[test]
    fn test_parse_empty_results() {
        let xml = r#"
<boinc_gui_rpc_reply>
<results>
</results>
</boinc_gui_rpc_reply>"#;
        let results = parse_results(xml);
        assert!(results.is_empty());
    }

    #[test]
    fn test_parse_success() {
        let xml = "<boinc_gui_rpc_reply>\n<success/>\n</boinc_gui_rpc_reply>";
        assert!(parse_success(xml).is_ok());
    }

    #[test]
    fn test_parse_success_error() {
        let xml = "<boinc_gui_rpc_reply>\n<error>not found</error>\n</boinc_gui_rpc_reply>";
        let err = parse_success(xml).unwrap_err();
        assert_eq!(err, "not found");
    }

    #[test]
    fn test_parse_success_unexpected() {
        let xml = "<boinc_gui_rpc_reply>\n<something_else/>\n</boinc_gui_rpc_reply>";
        assert!(parse_success(xml).is_err());
    }

    #[test]
    fn test_parse_file_transfers() {
        let xml = r#"
<boinc_gui_rpc_reply>
<file_transfers>
<file_transfer>
    <project_url>https://example.com/project/</project_url>
    <project_name>Example Project</project_name>
    <name>input_data_001.zip</name>
    <nbytes>1048576.000000</nbytes>
    <status>0</status>
    <file_xfer>
        <bytes_xferred>524288.000000</bytes_xferred>
        <xfer_speed>65536.000000</xfer_speed>
    </file_xfer>
</file_transfer>
<file_transfer>
    <project_url>https://example.com/project/</project_url>
    <project_name>Example Project</project_name>
    <name>output_result_001.zip</name>
    <nbytes>2097152.000000</nbytes>
    <status>0</status>
    <is_upload/>
    <file_xfer>
        <bytes_xferred>1048576.000000</bytes_xferred>
        <xfer_speed>131072.000000</xfer_speed>
    </file_xfer>
</file_transfer>
</file_transfers>
</boinc_gui_rpc_reply>"#;

        let transfers = parse_file_transfers(xml);
        assert_eq!(transfers.len(), 2);

        let t0 = &transfers[0];
        assert_eq!(t0.name, "input_data_001.zip");
        assert_eq!(t0.project_name, "Example Project");
        assert!((t0.nbytes - 1048576.0).abs() < 1.0);
        assert!((t0.bytes_xferred - 524288.0).abs() < 1.0);
        assert!((t0.xfer_speed - 65536.0).abs() < 1.0);
        assert!(!t0.is_upload);

        let t1 = &transfers[1];
        assert_eq!(t1.name, "output_result_001.zip");
        assert!(t1.is_upload);
        assert!((t1.bytes_xferred - 1048576.0).abs() < 1.0);
    }

    #[test]
    fn test_parse_empty_file_transfers() {
        let xml = r#"
<boinc_gui_rpc_reply>
<file_transfers>
</file_transfers>
</boinc_gui_rpc_reply>"#;
        let transfers = parse_file_transfers(xml);
        assert!(transfers.is_empty());
    }

    #[test]
    fn test_parse_statistics() {
        let xml = r#"
<boinc_gui_rpc_reply>
<statistics>
<project_statistics>
    <master_url>https://example.com/</master_url>
    <daily_statistics>
        <day>19800.000000</day>
        <user_total_credit>1000.000000</user_total_credit>
        <user_expavg_credit>50.000000</user_expavg_credit>
        <host_total_credit>500.000000</host_total_credit>
        <host_expavg_credit>25.000000</host_expavg_credit>
    </daily_statistics>
    <daily_statistics>
        <day>19801.000000</day>
        <user_total_credit>1100.000000</user_total_credit>
        <user_expavg_credit>55.000000</user_expavg_credit>
        <host_total_credit>550.000000</host_total_credit>
        <host_expavg_credit>27.500000</host_expavg_credit>
    </daily_statistics>
</project_statistics>
</statistics>
</boinc_gui_rpc_reply>"#;

        let stats = parse_statistics(xml);
        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0].master_url, "https://example.com/");
        assert_eq!(stats[0].daily_statistics.len(), 2);
        assert!((stats[0].daily_statistics[0].day - 19800.0).abs() < 0.1);
        assert!((stats[0].daily_statistics[0].user_total_credit - 1000.0).abs() < 0.1);
        assert!((stats[0].daily_statistics[1].host_expavg_credit - 27.5).abs() < 0.1);
    }

    #[test]
    fn test_parse_messages() {
        let xml = r#"
<boinc_gui_rpc_reply>
<msgs>
<msg>
    <project>Example Project</project>
    <pri>1</pri>
    <seqno>42</seqno>
    <body>
Computation started
</body>
    <time>1700000000</time>
</msg>
<msg>
    <project></project>
    <pri>3</pri>
    <seqno>43</seqno>
    <body>
Internal error occurred
</body>
    <time>1700000001</time>
</msg>
</msgs>
</boinc_gui_rpc_reply>"#;

        let messages = parse_messages(xml);
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].project, "Example Project");
        assert_eq!(messages[0].priority, 1);
        assert_eq!(messages[0].seqno, 42);
        assert_eq!(messages[0].body, "Computation started");
        assert_eq!(messages[1].priority, 3);
        assert_eq!(messages[1].seqno, 43);
    }

    #[test]
    fn test_parse_notices() {
        let xml = r#"
<boinc_gui_rpc_reply>
<notices>
<notice>
    <seqno>1</seqno>
    <title>Welcome</title>
    <description><![CDATA[<b>Hello</b> world]]></description>
    <create_time>1700000000</create_time>
    <project_name>Example</project_name>
    <link>https://example.com</link>
    <category>client</category>
    <is_private/>
</notice>
</notices>
</boinc_gui_rpc_reply>"#;

        let notices = parse_notices(xml);
        assert_eq!(notices.len(), 1);
        assert_eq!(notices[0].seqno, 1);
        assert_eq!(notices[0].title, "Welcome");
        assert_eq!(notices[0].description, "<b>Hello</b> world");
        assert!(notices[0].is_private);
        assert_eq!(notices[0].link, "https://example.com");
    }

    #[test]
    fn test_parse_disk_usage() {
        let xml = r#"
<boinc_gui_rpc_reply>
<disk_usage_summary>
    <project>
        <master_url>https://example.com/</master_url>
        <disk_usage>1073741824.000000</disk_usage>
    </project>
    <d_total>500000000000.000000</d_total>
    <d_free>250000000000.000000</d_free>
    <d_boinc>2147483648.000000</d_boinc>
    <d_allowed>50000000000.000000</d_allowed>
</disk_usage_summary>
</boinc_gui_rpc_reply>"#;

        let usage = parse_disk_usage(xml);
        assert_eq!(usage.projects.len(), 1);
        assert_eq!(usage.projects[0].master_url, "https://example.com/");
        assert!((usage.projects[0].disk_usage - 1073741824.0).abs() < 1.0);
        assert!((usage.d_total - 500000000000.0).abs() < 1.0);
        assert!((usage.d_free - 250000000000.0).abs() < 1.0);
    }

    #[test]
    fn test_parse_global_preferences() {
        let xml = r#"
<boinc_gui_rpc_reply>
<global_preferences>
    <run_on_batteries>0</run_on_batteries>
    <run_if_user_active>1</run_if_user_active>
    <idle_time_to_run>3.000000</idle_time_to_run>
    <max_ncpus_pct>100.000000</max_ncpus_pct>
    <cpu_usage_limit>100.000000</cpu_usage_limit>
    <ram_max_used_busy_frac>0.500000</ram_max_used_busy_frac>
    <ram_max_used_idle_frac>0.900000</ram_max_used_idle_frac>
    <disk_max_used_pct>90.000000</disk_max_used_pct>
    <disk_min_free_gb>0.100000</disk_min_free_gb>
    <work_buf_min_days>0.100000</work_buf_min_days>
</global_preferences>
</boinc_gui_rpc_reply>"#;

        let prefs = parse_global_preferences(xml);
        assert!(!prefs.run_on_batteries);
        assert!(prefs.run_if_user_active);
        assert!((prefs.idle_time_to_run - 3.0).abs() < 0.01);
        assert!((prefs.max_ncpus_pct - 100.0).abs() < 0.01);
        assert!((prefs.ram_max_used_busy_frac - 0.5).abs() < 0.01);
        assert!((prefs.disk_max_used_pct - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_serialize_global_preferences() {
        let prefs = GlobalPreferences {
            run_on_batteries: false,
            run_if_user_active: true,
            cpu_usage_limit: 80.0,
            ..Default::default()
        };
        let xml = serialize_global_preferences(&prefs);
        assert!(xml.contains("<run_on_batteries>0</run_on_batteries>"));
        assert!(xml.contains("<run_if_user_active>1</run_if_user_active>"));
        assert!(xml.contains("<cpu_usage_limit>80</cpu_usage_limit>"));
    }

    #[test]
    fn test_parse_host_info() {
        let xml = r#"
<boinc_gui_rpc_reply>
<host_info>
    <domain_name>myhost</domain_name>
    <ip_addr>192.168.1.100</ip_addr>
    <p_ncpus>8</p_ncpus>
    <p_vendor>GenuineIntel</p_vendor>
    <p_model>Intel Core i7</p_model>
    <p_fpops>2000000000.000000</p_fpops>
    <m_nbytes>17179869184.000000</m_nbytes>
    <os_name>Microsoft Windows</os_name>
    <os_version>10.0</os_version>
</host_info>
</boinc_gui_rpc_reply>"#;

        let info = parse_host_info(xml);
        assert_eq!(info.domain_name, "myhost");
        assert_eq!(info.ip_addr, "192.168.1.100");
        assert_eq!(info.p_ncpus, 8);
        assert_eq!(info.p_vendor, "GenuineIntel");
        assert_eq!(info.os_name, "Microsoft Windows");
    }

    #[test]
    fn test_parse_all_projects_list() {
        let xml = r#"
<boinc_gui_rpc_reply>
<projects>
<project>
    <name>SETI@home</name>
    <url>https://setiathome.berkeley.edu/</url>
    <general_area>Physical Science</general_area>
    <specific_area>Astronomy</specific_area>
    <description>Search for ET</description>
    <home>UC Berkeley</home>
    <platforms>
        <name>windows_x86_64</name>
        <name>x86_64-pc-linux-gnu</name>
    </platforms>
</project>
</projects>
</boinc_gui_rpc_reply>"#;

        let entries = parse_all_projects_list(xml);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "SETI@home");
        assert_eq!(entries[0].url, "https://setiathome.berkeley.edu/");
        assert_eq!(entries[0].platforms.len(), 2);
        assert_eq!(entries[0].platforms[0], "windows_x86_64");
    }

    #[test]
    fn test_parse_account_out() {
        let xml = r#"
<boinc_gui_rpc_reply>
<account_out>
    <error_num>0</error_num>
    <authenticator>abc123def456</authenticator>
</account_out>
</boinc_gui_rpc_reply>"#;

        let out = parse_account_out(xml);
        assert_eq!(out.error_num, 0);
        assert_eq!(out.authenticator, "abc123def456");
        assert_eq!(out.error_msg, "");
    }

    #[test]
    fn test_parse_project_attach_reply() {
        let xml = r#"
<boinc_gui_rpc_reply>
<project_attach_reply>
    <error_num>0</error_num>
    <message>Success</message>
</project_attach_reply>
</boinc_gui_rpc_reply>"#;

        let reply = parse_project_attach_reply(xml);
        assert_eq!(reply.error_num, 0);
        assert_eq!(reply.messages.len(), 1);
        assert_eq!(reply.messages[0], "Success");
    }

    #[test]
    fn test_read_text_cdata() {
        let xml = "<desc><![CDATA[<b>Hello</b>]]></desc>";
        let mut reader = Reader::from_str(xml);
        let mut buf = Vec::new();
        // Skip the start tag
        let _ = reader.read_event_into(&mut buf);
        buf.clear();
        let text = read_text(&mut reader);
        assert_eq!(text, "<b>Hello</b>");
    }
}
