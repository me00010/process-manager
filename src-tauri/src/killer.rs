use sysinfo::{Pid, System};

use crate::models::KillResult;

const PROTECTED_PROCESSES: &[&str] = &[
    "system",
    "system idle process",
    "registry",
    "memory compression",
    "smss.exe",
    "csrss.exe",
    "wininit.exe",
    "services.exe",
    "lsass.exe",
    "lsaiso.exe",
    "winlogon.exe",
    "dwm.exe",
    "fontdrvhost.exe",
];

pub fn is_protected(name: &str) -> bool {
    let lower = name.trim().to_lowercase();
    PROTECTED_PROCESSES.iter().any(|p| *p == lower)
}

pub fn kill_process(sys: &System, pid: u32) -> KillResult {
    if pid == std::process::id() {
        return KillResult {
            pid,
            ok: false,
            error: Some("不能结束当前应用自身".to_string()),
        };
    }

    match sys.process(Pid::from_u32(pid)) {
        Some(proc) => {
            let name = proc.name().to_string();
            if is_protected(&name) {
                return KillResult {
                    pid,
                    ok: false,
                    error: Some("系统关键进程，禁止结束".to_string()),
                };
            }
            if proc.kill() {
                KillResult {
                    pid,
                    ok: true,
                    error: None,
                }
            } else {
                KillResult {
                    pid,
                    ok: false,
                    error: Some("结束失败，可能权限不足或进程已退出".to_string()),
                }
            }
        }
        None => KillResult {
            pid,
            ok: false,
            error: Some("进程不存在".to_string()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protects_known_system_processes() {
        assert!(is_protected("System"));
        assert!(is_protected("SYSTEM IDLE PROCESS"));
        assert!(is_protected("csrss.exe"));
        assert!(is_protected("lsass.exe"));
        assert!(is_protected("winlogon.exe"));
        assert!(is_protected(" services.exe "));
    }

    #[test]
    fn does_not_protect_normal_processes() {
        assert!(!is_protected("notepad.exe"));
        assert!(!is_protected("node.exe"));
        assert!(!is_protected("code.exe"));
        assert!(!is_protected("SystemSettings.exe"));
        assert!(!is_protected(""));
    }
}
