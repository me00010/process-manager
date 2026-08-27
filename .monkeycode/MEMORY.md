# User Instruction Memory

This file records user instructions, preferences, and teachings for reference in future interactions.

## Format

### User Instruction Entry

[User Instruction Summary]
- Date: [YYYY-MM-DD]
- Context: [Mentioned scenario or time]
- Instructions:
  - [Content of user teaching or instruction, described line by line]

### Project Knowledge Entry

[Project Knowledge Summary]
- Date: [YYYY-MM-DD]
- Context: Discovered by Agent while performing [specific task description]
- Category: [Operations & Deployment|Build Methods|Testing Methods|Troubleshooting & Debugging|Workflow & Collaboration|Environment Configuration]
- Instructions:
  - [Specific knowledge points, described line by line]

## Deduplication Strategy

- Before adding a new entry, check for similar or identical instructions.
- If a duplicate is found, skip the new entry or merge it with the existing one.
- When merging, update the context or date information.

## Entries

[Project Knowledge Summary]
- Date: 2026-08-26
- Context: Discovered by Agent while developing a Tauri 2.x Windows process manager (process-manager) in a Linux environment
- Category: Build Methods & Environment Configuration
- Instructions:
  - Tauri Windows 应用无法在 Linux 上交叉编译出 Windows 安装包；Linux 环境只能完成代码编译与 Rust 单元测试验证，最终 Windows 构建需在 Windows 机器执行 `npm run tauri build`，或使用仓库内 `.github/workflows/build-windows.yml` 在 GitHub Actions 上构建。
  - tauri-build 2.x 设置 Windows 管理员权限的方式：`tauri_build::WindowsAttributes::new().app_manifest("<xml>")` 直接传入 manifest XML 字符串，在 XML 中声明 `<requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>`；`tauri_build::AppManifest` 只用于 ACL 命令权限控制，没有 requested_execution_level 方法。
  - sysinfo 0.30 中 `Process::name()` 返回 `&str`（非 `&OsStr`），`Pid::as_u32()` 与 `Pid::from_u32()` 均可用，`Process::memory()` 返回字节数。
  - Tauri 编译 Linux 需要安装 libwebkit2gtk-4.1-dev、libayatana-appindicator3-dev、librsvg2-dev 等系统库；apt 安装依赖较慢，cargo fetch 可提前下载依赖。
