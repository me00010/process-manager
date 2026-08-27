# 进程管理器 (ProcessManager)

一个基于 Tauri 2.x 的 Windows 桌面小工具：按**端口号**、**进程名称**或 **PID** 搜索系统进程，查看进程详细信息（CPU、内存、端口、可执行文件路径），并支持选中结束进程。

## 功能特性

- 按端口号 / 进程名称 / PID 模糊搜索，三合一输入框
- 进程列表展示：PID、名称、CPU、内存、监听端口、可执行文件路径
- 系统关键进程（`System`、`csrss.exe`、`lsass.exe` 等）自动标记"受保护"，不可选中结束
- 支持多选结束进程，带确认对话框与结果反馈
- 自动刷新（默认 3 秒）与手动刷新
- 应用以管理员权限启动，可结束普通权限进程

## 技术栈

- 桌面框架：Tauri 2.x
- 前端：React 18 + Vite + Ant Design 5
- 后端：Rust（`sysinfo` + PowerShell 端口查询）

## 环境要求

- [Rust](https://www.rust-lang.org/tools/install)（stable，安装时选择默认工具链）
- [Node.js](https://nodejs.org/) >= 18
- Windows 10/11 x64（目标运行平台）

## 本地开发

```bash
# 1. 安装前端依赖
npm install

# 2. 启动开发模式（带热更新的桌面窗口）
npm run tauri dev
```

## 构建安装包

```bash
npm install
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`：

- `nsis/ProcessManager_x.x.x_x64-setup.exe`：NSIS 安装程序
- `msi/*.msi`：MSI 安装包

## 自动构建

仓库包含 GitHub Actions 工作流 `.github/workflows/build-windows.yml`：

- 推送 `v*` 标签时自动在 Windows 上构建并创建 Release
- 也可在 Actions 页面手动触发（`workflow_dispatch`）

## 使用说明

1. 应用默认以管理员权限启动（会触发 UAC 确认）。
2. 搜索框支持三种输入：
   - 端口号（如 `8080`）：列出监听该端口的进程
   - 进程名（如 `node`）：名称模糊匹配
   - PID（如 `1234`）：精确匹配
3. 勾选进程后点击"结束选中进程"，确认后执行结束操作。
4. 受保护的系统关键进程会置灰不可勾选。

## 目录结构

```
├── src/                  # React 前端
│   ├── components/       # 搜索栏、进程表格组件
│   ├── api.ts            # Tauri invoke 封装
│   └── App.tsx           # 主界面逻辑
├── src-tauri/            # Rust 后端
│   ├── src/
│   │   ├── commands.rs   # Tauri command 入口
│   │   ├── process.rs    # 进程枚举与搜索过滤
│   │   ├── ports.rs      # 端口→PID 映射
│   │   ├── killer.rs     # 结束进程与受保护进程判定
│   │   └── models.rs     # 数据结构
│   └── tauri.conf.json   # Tauri 配置
└── .github/workflows/    # Windows 构建工作流
```

## 说明

- 端口映射通过 PowerShell `Get-NetTCPConnection` / `Get-NetUDPEndpoint` 获取，需要 PowerShell 可用。
- 结束进程失败时界面会展示原因；若因权限不足失败，请确认应用已以管理员身份运行。
