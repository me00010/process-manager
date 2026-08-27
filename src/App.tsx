import { useCallback, useEffect, useRef, useState } from "react";
import { App as AntApp, Button, Modal, Typography } from "antd";
import { DeleteOutlined } from "@ant-design/icons";
import { checkIsAdmin, killProcesses, listProcesses } from "./api";
import ProcessTable from "./components/ProcessTable";
import SearchBar from "./components/SearchBar";
import type { KillResult, ProcessInfo } from "./types";

const AUTO_REFRESH_INTERVAL_MS = 3000;

export default function App() {
  const { message } = AntApp.useApp();
  const [processes, setProcesses] = useState<ProcessInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [keyword, setKeyword] = useState("");
  const [selectedRowKeys, setSelectedRowKeys] = useState<React.Key[]>([]);
  const [autoRefresh, setAutoRefresh] = useState(true);
  const [isAdmin, setIsAdmin] = useState(false);
  const [killing, setKilling] = useState(false);
  const timerRef = useRef<number | null>(null);

  const load = useCallback(
    async (kw: string, silent = false) => {
      if (!silent) setLoading(true);
      try {
        const data = await listProcesses(kw || undefined);
        setProcesses(data);
        setSelectedRowKeys((keys) =>
          keys.filter((k) => data.some((p) => p.pid === k))
        );
      } catch (err) {
        message.error(`获取进程列表失败: ${String(err)}`);
      } finally {
        if (!silent) setLoading(false);
      }
    },
    [message]
  );

  useEffect(() => {
    load("");
    checkIsAdmin()
      .then(setIsAdmin)
      .catch(() => setIsAdmin(false));
  }, [load]);

  useEffect(() => {
    if (!autoRefresh) return;
    timerRef.current = window.setInterval(() => {
      load(keyword, true);
    }, AUTO_REFRESH_INTERVAL_MS);
    return () => {
      if (timerRef.current) window.clearInterval(timerRef.current);
    };
  }, [autoRefresh, keyword, load]);

  const handleSearch = (kw: string) => {
    setKeyword(kw);
    load(kw);
  };

  const handleKill = () => {
    const selected = processes.filter((p) =>
      selectedRowKeys.includes(p.pid)
    );
    const protectedSelected = selected.filter((p) => p.protected);
    const normal = selected.filter((p) => !p.protected);
    if (protectedSelected.length > 0) {
      Modal.warning({
        title: "包含受保护进程",
        content: `${protectedSelected
          .map((p) => `${p.name} (PID ${p.pid})`)
          .join("、")} 为系统关键进程，无法结束，已自动跳过。`,
      });
    }
    if (normal.length === 0) return;

    Modal.confirm({
      title: "确认结束进程",
      content: `确定要结束选中的 ${normal.length} 个进程吗？${normal.length === 1 ? "" : "结束这些进程可能导致相关程序崩溃或数据丢失。"}`,
      okText: "结束",
      okButtonProps: { danger: true },
      cancelText: "取消",
      onOk: async () => {
        setKilling(true);
        try {
          const results: KillResult[] = await killProcesses(
            normal.map((p) => p.pid)
          );
          const failed = results.filter((r) => !r.ok);
          if (failed.length === 0) {
            message.success(
              `已成功结束 ${results.length} 个进程`
            );
          } else {
            message.warning(
              `${results.length - failed.length} 个进程结束成功，${failed.length} 个失败：${failed
                .map((f) => `${f.pid}(${f.error || "未知错误"})`)
                .join("、")}`
            );
          }
          await load(keyword, true);
        } catch (err) {
          message.error(`结束进程失败: ${String(err)}`);
        } finally {
          setKilling(false);
        }
      },
    });
  };

  return (
    <div style={{ padding: 16 }}>
      <Typography.Title level={4} style={{ marginTop: 0 }}>
        进程管理器
      </Typography.Title>
      <div style={{ marginBottom: 12 }}>
        <SearchBar
          onSearch={handleSearch}
          onRefresh={() => load(keyword)}
          loading={loading}
          autoRefresh={autoRefresh}
          onAutoRefreshChange={setAutoRefresh}
          isAdmin={isAdmin}
        />
      </div>
      <div style={{ marginBottom: 8 }}>
        <Button
          type="primary"
          danger
          icon={<DeleteOutlined />}
          disabled={selectedRowKeys.length === 0}
          loading={killing}
          onClick={handleKill}
        >
          结束选中进程 ({selectedRowKeys.length})
        </Button>
      </div>
      <ProcessTable
        processes={processes}
        loading={loading}
        selectedRowKeys={selectedRowKeys}
        onSelectionChange={setSelectedRowKeys}
      />
    </div>
  );
}
