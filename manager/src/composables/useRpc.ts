import { invoke } from "@tauri-apps/api/core";
import type { TaskResult, Project, CcStatus } from "../types/boinc";

/**
 * Thin wrappers around Tauri invoke() calls to the Rust backend.
 * Each function maps 1:1 to a #[tauri::command] in lib.rs.
 */

export async function connectLocal(dataDir: string): Promise<void> {
  return invoke("connect_local", { dataDir });
}

export async function connect(
  host: string,
  port: number,
  password: string,
): Promise<void> {
  return invoke("connect", { host, port, password });
}

export async function disconnect(): Promise<void> {
  return invoke("disconnect");
}

export async function getConnectionState(): Promise<string> {
  return invoke("get_connection_state");
}

export async function getResults(activeOnly: boolean): Promise<TaskResult[]> {
  return invoke("get_results", { activeOnly });
}

export async function getProjectStatus(): Promise<Project[]> {
  return invoke("get_project_status");
}

export async function getCcStatus(): Promise<CcStatus> {
  return invoke("get_cc_status");
}
