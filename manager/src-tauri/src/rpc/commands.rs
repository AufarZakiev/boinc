use super::connection::RpcClient;
use super::types::{
    AccountOut, CcStatus, DiskUsage, FileTransfer, GlobalPreferences, HostInfo, Message, Notice,
    Project, ProjectAttachReply, ProjectListEntry, ProjectStatistics, TaskResult,
};
use super::xml_parse;

/// High-level RPC commands that return typed data.
impl RpcClient {
    // ── Read operations ──────────────────────────────────────────────

    /// Get all results (tasks). If `active_only` is true, only active tasks are returned.
    pub async fn get_results(&self, active_only: bool) -> Result<Vec<TaskResult>, String> {
        let req = if active_only {
            "<get_results>\n<active_only>1</active_only>\n</get_results>"
        } else {
            "<get_results/>"
        };
        let xml = self.rpc_call(req).await?;
        Ok(xml_parse::parse_results(&xml))
    }

    /// Get all attached projects.
    pub async fn get_project_status(&self) -> Result<Vec<Project>, String> {
        let xml = self.rpc_call("<get_project_status/>").await?;
        Ok(xml_parse::parse_projects(&xml))
    }

    /// Get the client's current status (run modes, network status).
    pub async fn get_cc_status(&self) -> Result<CcStatus, String> {
        let xml = self.rpc_call("<get_cc_status/>").await?;
        Ok(xml_parse::parse_cc_status(&xml))
    }

    /// Get all active file transfers.
    pub async fn get_file_transfers(&self) -> Result<Vec<FileTransfer>, String> {
        let xml = self.rpc_call("<get_file_transfers/>").await?;
        Ok(xml_parse::parse_file_transfers(&xml))
    }

    // ── Task (result) operations ─────────────────────────────────────

    /// Send a task operation identified by project URL and result name.
    async fn result_op(&self, op: &str, project_url: &str, name: &str) -> Result<(), String> {
        let req = format!(
            "<{op}>\n<project_url>{project_url}</project_url>\n<name>{name}</name>\n</{op}>"
        );
        let xml = self.rpc_call(&req).await?;
        xml_parse::parse_success(&xml)
    }

    pub async fn suspend_result(&self, project_url: &str, name: &str) -> Result<(), String> {
        self.result_op("suspend_result", project_url, name).await
    }

    pub async fn resume_result(&self, project_url: &str, name: &str) -> Result<(), String> {
        self.result_op("resume_result", project_url, name).await
    }

    pub async fn abort_result(&self, project_url: &str, name: &str) -> Result<(), String> {
        self.result_op("abort_result", project_url, name).await
    }

    // ── Project operations ───────────────────────────────────────────

    /// Send a project operation identified by master URL.
    async fn project_op(&self, op: &str, project_url: &str) -> Result<(), String> {
        let req = format!(
            "<{op}>\n<project_url>{project_url}</project_url>\n</{op}>"
        );
        let xml = self.rpc_call(&req).await?;
        xml_parse::parse_success(&xml)
    }

    pub async fn project_suspend(&self, project_url: &str) -> Result<(), String> {
        self.project_op("project_suspend", project_url).await
    }

    pub async fn project_resume(&self, project_url: &str) -> Result<(), String> {
        self.project_op("project_resume", project_url).await
    }

    pub async fn project_update(&self, project_url: &str) -> Result<(), String> {
        self.project_op("project_update", project_url).await
    }

    pub async fn project_nomorework(&self, project_url: &str) -> Result<(), String> {
        self.project_op("project_nomorework", project_url).await
    }

    pub async fn project_allowmorework(&self, project_url: &str) -> Result<(), String> {
        self.project_op("project_allowmorework", project_url).await
    }

    pub async fn project_reset(&self, project_url: &str) -> Result<(), String> {
        self.project_op("project_reset", project_url).await
    }

    pub async fn project_detach(&self, project_url: &str) -> Result<(), String> {
        self.project_op("project_detach", project_url).await
    }

    // ── Mode controls ────────────────────────────────────────────────

    /// Set a run mode. `mode`: 1=always, 2=auto, 3=never. `duration`: seconds (0=permanent).
    async fn set_mode(&self, tag: &str, mode: i32, duration: f64) -> Result<(), String> {
        let req = format!(
            "<set_{tag}>\n<{tag}>{mode}</{tag}>\n<duration>{duration}</duration>\n</set_{tag}>"
        );
        let xml = self.rpc_call(&req).await?;
        xml_parse::parse_success(&xml)
    }

    pub async fn set_run_mode(&self, mode: i32, duration: f64) -> Result<(), String> {
        self.set_mode("run_mode", mode, duration).await
    }

    pub async fn set_gpu_mode(&self, mode: i32, duration: f64) -> Result<(), String> {
        self.set_mode("gpu_mode", mode, duration).await
    }

    pub async fn set_network_mode(&self, mode: i32, duration: f64) -> Result<(), String> {
        self.set_mode("network_mode", mode, duration).await
    }

    // ── Transfer operations ──────────────────────────────────────────

    /// Send a file transfer operation identified by project URL and filename.
    async fn transfer_op(&self, op: &str, project_url: &str, filename: &str) -> Result<(), String> {
        let req = format!(
            "<{op}>\n<project_url>{project_url}</project_url>\n<filename>{filename}</filename>\n</{op}>"
        );
        let xml = self.rpc_call(&req).await?;
        xml_parse::parse_success(&xml)
    }

    pub async fn retry_file_transfer(&self, project_url: &str, filename: &str) -> Result<(), String> {
        self.transfer_op("retry_file_transfer", project_url, filename).await
    }

    pub async fn abort_file_transfer(&self, project_url: &str, filename: &str) -> Result<(), String> {
        self.transfer_op("abort_file_transfer", project_url, filename).await
    }

    // ── Other operations ─────────────────────────────────────────────

    pub async fn run_benchmarks(&self) -> Result<(), String> {
        let xml = self.rpc_call("<run_benchmarks/>").await?;
        xml_parse::parse_success(&xml)
    }

    pub async fn network_available(&self) -> Result<(), String> {
        let xml = self.rpc_call("<network_available/>").await?;
        xml_parse::parse_success(&xml)
    }

    pub async fn quit(&self) -> Result<(), String> {
        let xml = self.rpc_call("<quit/>").await?;
        xml_parse::parse_success(&xml)
    }

    // ── Statistics ─────────────────────────────────────────────────

    pub async fn get_statistics(&self) -> Result<Vec<ProjectStatistics>, String> {
        let xml = self.rpc_call("<get_statistics/>").await?;
        Ok(xml_parse::parse_statistics(&xml))
    }

    // ── Messages ──────────────────────────────────────────────────

    pub async fn get_messages(&self, seqno: i32) -> Result<Vec<Message>, String> {
        let req = format!(
            "<get_messages>\n<seqno>{seqno}</seqno>\n</get_messages>"
        );
        let xml = self.rpc_call(&req).await?;
        Ok(xml_parse::parse_messages(&xml))
    }

    // ── Notices ───────────────────────────────────────────────────

    pub async fn get_notices(&self, seqno: i32) -> Result<Vec<Notice>, String> {
        let req = format!(
            "<get_notices>\n<seqno>{seqno}</seqno>\n</get_notices>"
        );
        let xml = self.rpc_call(&req).await?;
        Ok(xml_parse::parse_notices(&xml))
    }

    // ── Disk usage ────────────────────────────────────────────────

    pub async fn get_disk_usage(&self) -> Result<DiskUsage, String> {
        let xml = self.rpc_call("<get_disk_usage/>").await?;
        Ok(xml_parse::parse_disk_usage(&xml))
    }

    // ── Preferences ───────────────────────────────────────────────

    pub async fn get_global_prefs_override(&self) -> Result<GlobalPreferences, String> {
        let xml = self.rpc_call("<get_global_prefs_override/>").await?;
        Ok(xml_parse::parse_global_preferences(&xml))
    }

    pub async fn set_global_prefs_override(
        &self,
        prefs: &GlobalPreferences,
    ) -> Result<(), String> {
        let prefs_xml = xml_parse::serialize_global_preferences(prefs);
        let req = format!(
            "<set_global_prefs_override>\n{prefs_xml}\n</set_global_prefs_override>"
        );
        self.rpc_call(&req).await?;
        // Tell the client to re-read the override file
        let xml = self.rpc_call("<read_global_prefs_override/>").await?;
        xml_parse::parse_success(&xml)
    }

    // ── Host info ─────────────────────────────────────────────────

    pub async fn get_host_info(&self) -> Result<HostInfo, String> {
        let xml = self.rpc_call("<get_host_info/>").await?;
        Ok(xml_parse::parse_host_info(&xml))
    }

    // ── Project attach ────────────────────────────────────────────

    pub async fn get_all_projects_list(&self) -> Result<Vec<ProjectListEntry>, String> {
        let xml = self.rpc_call("<get_all_projects_list/>").await?;
        Ok(xml_parse::parse_all_projects_list(&xml))
    }

    pub async fn lookup_account(
        &self,
        url: &str,
        email: &str,
        password: &str,
    ) -> Result<(), String> {
        let req = format!(
            "<lookup_account>\n\
             <url>{url}</url>\n\
             <email_addr>{email}</email_addr>\n\
             <passwd_hash>{password}</passwd_hash>\n\
             </lookup_account>"
        );
        self.rpc_call(&req).await?;
        Ok(())
    }

    pub async fn lookup_account_poll(&self) -> Result<AccountOut, String> {
        let xml = self.rpc_call("<lookup_account_poll/>").await?;
        Ok(xml_parse::parse_account_out(&xml))
    }

    pub async fn project_attach(
        &self,
        url: &str,
        authenticator: &str,
        name: &str,
    ) -> Result<(), String> {
        let req = format!(
            "<project_attach>\n\
             <project_url>{url}</project_url>\n\
             <authenticator>{authenticator}</authenticator>\n\
             <project_name>{name}</project_name>\n\
             </project_attach>"
        );
        self.rpc_call(&req).await?;
        Ok(())
    }

    pub async fn project_attach_poll(&self) -> Result<ProjectAttachReply, String> {
        let xml = self.rpc_call("<project_attach_poll/>").await?;
        Ok(xml_parse::parse_project_attach_reply(&xml))
    }
}
