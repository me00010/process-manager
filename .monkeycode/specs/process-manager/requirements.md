# Requirements Document

## Introduction

一个基于 Tauri 的 Windows 桌面小工具（进程管理器），支持按端口号、进程名称或 PID 搜索系统进程，展示进程详细信息，并允许用户结束选中的进程。

## Technical Stack

- 桌面框架: Tauri 2.x
- 前端: React 18 + Vite + Ant Design 5
- 后端语言: Rust（Tauri commands）
- 目标平台: Windows 10/11 x64

## Glossary

- **系统 (System)**: 本进程管理器应用
- **进程 (Process)**: Windows 操作系统中运行的程序实例
- **端口 (Port)**: TCP/UDP 网络监听端口号
- **PID**: 进程标识符（Process Identifier）
- **受保护进程**: 标记为不可结束的系统关键进程（如 `System`、`Idle`、`csrss.exe`、`smss.exe` 等）

## Requirements

### Requirement 1: 进程搜索

**User Story:** AS 用户, I want 通过端口、名称或 PID 搜索进程, so that 快速定位目标进程

#### Acceptance Criteria

1. WHEN 用户在搜索框输入端口号并触发搜索，系统 SHALL 返回监听该端口（含 TCP 与 UDP）的进程列表
2. WHEN 用户在搜索框输入进程名称并触发搜索，系统 SHALL 返回名称模糊匹配的进程列表
3. WHEN 用户在搜索框输入 PID 并触发搜索，系统 SHALL 返回对应 PID 的进程
4. WHEN 搜索输入为空，系统 SHALL 返回全部进程列表
5. WHEN 搜索结果为 0 条，系统 SHALL 显示空结果提示

### Requirement 2: 进程信息展示

**User Story:** AS 用户, I want 查看进程详细信息, so that 确认进程身份后再决定是否结束

#### Acceptance Criteria

1. WHEN 进程列表加载成功，系统 SHALL 展示每条进程的 PID、名称、CPU 占用、内存占用、关联端口与可执行文件路径
2. WHEN 某进程占用端口，系统 SHALL 在列表展示其占用的端口号
3. WHEN 进程无关联端口，系统 SHALL 以空值展示端口字段

### Requirement 3: 结束进程

**User Story:** AS 用户, I want 结束选中的进程, so that 释放端口或资源

#### Acceptance Criteria

1. WHEN 用户选中一个或多个进程并点击结束按钮，系统 SHALL 弹出确认对话框
2. WHEN 用户确认结束，系统 SHALL 结束选中的进程并刷新进程列表
3. IF 进程结束失败，系统 SHALL 展示失败原因（如权限不足、进程已退出）
4. IF 用户选中受保护进程，系统 SHALL 阻止结束操作并提示该进程受保护
5. WHEN 用户取消确认对话框，系统 SHALL 不执行任何结束操作

### Requirement 4: 进程列表刷新

**User Story:** AS 用户, I want 手动或自动刷新进程列表, so that 看到最新进程状态

#### Acceptance Criteria

1. WHEN 用户点击刷新按钮，系统 SHALL 重新获取进程列表
2. WHILE 应用运行，系统 SHALL 提供可配置的自动刷新间隔（默认 3 秒）
3. WHEN 自动刷新进行中，用户当前的搜索关键字 SHALL 保持不变

### Requirement 5: 权限处理

**User Story:** AS 用户, I want 结束需要较高权限的进程, so that 能结束任意目标进程

#### Acceptance Criteria

1. WHEN 结束进程因权限不足失败，系统 SHALL 提示用户以管理员身份重新运行应用
2. IF 当前应用无管理员权限，系统 SHALL 在界面上提示"建议以管理员身份运行"
