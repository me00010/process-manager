import { Space, Table, Tag, Tooltip } from "antd";
import type { ColumnsType } from "antd/es/table";
import type { ProcessInfo } from "../types";

export function formatMemory(bytes: number): string {
  if (bytes >= 1024 * 1024 * 1024) {
    return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
  }
  if (bytes >= 1024 * 1024) {
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }
  if (bytes >= 1024) {
    return `${(bytes / 1024).toFixed(1)} KB`;
  }
  return `${bytes} B`;
}

interface Props {
  processes: ProcessInfo[];
  loading: boolean;
  selectedRowKeys: React.Key[];
  onSelectionChange: (keys: React.Key[]) => void;
}

const columns: ColumnsType<ProcessInfo> = [
  {
    title: "PID",
    dataIndex: "pid",
    width: 90,
    sorter: (a, b) => a.pid - b.pid,
  },
  {
    title: "名称",
    dataIndex: "name",
    width: 220,
    ellipsis: true,
    render: (name: string, record: ProcessInfo) => (
      <Space size={4}>
        <span>{name}</span>
        {record.protected && <Tag color="red">受保护</Tag>}
      </Space>
    ),
  },
  {
    title: "CPU",
    dataIndex: "cpu",
    width: 100,
    sorter: (a, b) => a.cpu - b.cpu,
    render: (v: number) => `${v.toFixed(1)}%`,
  },
  {
    title: "内存",
    dataIndex: "memory",
    width: 120,
    sorter: (a, b) => a.memory - b.memory,
    render: formatMemory,
  },
  {
    title: "端口",
    dataIndex: "ports",
    width: 180,
    render: (ports: number[]) =>
      ports.length ? (
        <Space size={4} wrap>
          {ports.map((p) => (
            <Tag key={p}>{p}</Tag>
          ))}
        </Space>
      ) : (
        <span style={{ color: "#999" }}>-</span>
      ),
  },
  {
    title: "可执行文件路径",
    dataIndex: "path",
    ellipsis: true,
    render: (path: string | null) =>
      path ? (
        <Tooltip title={path}>
          <span>{path}</span>
        </Tooltip>
      ) : (
        <span style={{ color: "#999" }}>-</span>
      ),
  },
];

export default function ProcessTable({
  processes,
  loading,
  selectedRowKeys,
  onSelectionChange,
}: Props) {
  return (
    <Table<ProcessInfo>
      rowKey="pid"
      size="small"
      columns={columns}
      dataSource={processes}
      loading={loading}
      scroll={{ y: "calc(100vh - 220px)" }}
      rowSelection={{
        selectedRowKeys,
        onChange: onSelectionChange,
        getCheckboxProps: (record: ProcessInfo) => ({
          disabled: record.protected,
        }),
      }}
      pagination={{ pageSize: 50, showSizeChanger: true, showTotal: (t) => `共 ${t} 条` }}
    />
  );
}
