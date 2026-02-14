use quick_xml::events::Event;
use quick_xml::Reader;

use super::types::{CcStatus, FileTransfer, Project, TaskResult};

/// Extract text content of an XML element, advancing the reader past its end tag.
fn read_text(reader: &mut Reader<&[u8]>) -> String {
    let mut buf = Vec::new();
    let mut text = String::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => {
                text = e.unescape().unwrap_or_default().to_string();
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
}
