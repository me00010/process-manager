import { invoke } from "@tauri-apps/api/core";
import type { KillResult, ProcessInfo } from "./types";

export function listProcesses(keyword?: string): Promise<ProcessInfo[]> {
  return invoke<ProcessInfo[]>("list_processes", { keyword: keyword || null });
}

export function killProcesses(pids: number[]): Promise<KillResult[]> {
  return invoke<KillResult[]>("kill_processes", { pids });
}

export function checkIsAdmin(): Promise<boolean> {
  return invoke<boolean>("is_admin");
}
