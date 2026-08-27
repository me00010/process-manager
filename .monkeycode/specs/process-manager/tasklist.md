# Task List: process-manager

## 1. 项目脚手架

- [x] 1.1 初始化 Tauri 2.x + React + Vite 项目结构（package.json、vite 配置、前端入口）
- [x] 1.2 创建 Rust 后端结构（Cargo.toml、src/main.rs、src/lib.rs、tauri.conf.json、capabilities）

## 2. Rust 后端核心

- [x] 2.1 实现 `models.rs`：ProcessInfo / KillResult 数据结构
- [x] 2.2 实现 `process.rs`：基于 sysinfo 枚举进程、按端口/名称/PID 过滤
- [x] 2.3 实现 `ports.rs`：PowerShell 获取端口到 PID 映射
- [x] 2.4 实现 `killer.rs`：结束进程、受保护进程判定
- [x] 2.5 实现 `commands.rs` 并在 lib.rs 注册 Tauri commands

## 3. Rust 单元测试

- [x] 3.1 受保护进程判定测试
- [x] 3.2 端口映射 JSON 解析测试
- [x] 3.3 关键字匹配逻辑测试

## 4. 前端界面

- [x] 4.1 前端脚手架：React + Vite + Ant Design 依赖接入
- [x] 4.2 实现搜索框、刷新/自动刷新控制
- [x] 4.3 实现进程表格（多选、端口/CPU/内存展示、受保护标记）
- [x] 4.4 实现结束进程确认弹窗与结果反馈
- [x] 4.5 实现管理员权限提示

## 5. 配置与工程化

- [x] 5.1 tauri.conf.json 配置（窗口、管理员权限、identifier、icons）
- [x] 5.2 前端代理与 allowedHosts 配置（开发预览环境）
- [x] 5.3 GitHub Actions 工作流（Windows 构建发布）
- [x] 5.4 README 文档（构建与使用说明）

## 6. 验证

- [x] 6.1 cargo check / cargo test 通过
- [x] 6.2 前端类型检查与构建通过
