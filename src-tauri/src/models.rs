use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu: f32,
    pub memory: u64,
    pub path: Option<String>,
    pub ports: Vec<u16>,
    pub protected: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct KillResult {
    pub pid: u32,
    pub ok: bool,
    pub error: Option<String>,
}
