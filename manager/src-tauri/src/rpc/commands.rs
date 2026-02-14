use super::connection::RpcClient;
use super::types::{CcStatus, Project, TaskResult};
use super::xml_parse;

/// High-level RPC commands that return typed data.
impl RpcClient {
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
}
