import { useState } from "react";
import { Button, Input, Space, Switch, Tag } from "antd";
import {
  ReloadOutlined,
  SafetyCertificateOutlined,
  SearchOutlined,
} from "@ant-design/icons";

interface Props {
  onSearch: (keyword: string) => void;
  onRefresh: () => void;
  loading: boolean;
  autoRefresh: boolean;
  onAutoRefreshChange: (value: boolean) => void;
  isAdmin: boolean;
}

export default function SearchBar({
  onSearch,
  onRefresh,
  loading,
  autoRefresh,
  onAutoRefreshChange,
  isAdmin,
}: Props) {
  const [input, setInput] = useState("");

  return (
    <Space wrap size="middle">
      <Input
        placeholder="输入端口号、进程名称或 PID"
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onPressEnter={() => onSearch(input.trim())}
        allowClear
        style={{ width: 320 }}
      />
      <Button
        type="primary"
        icon={<SearchOutlined />}
        loading={loading}
        onClick={() => onSearch(input.trim())}
      >
        搜索
      </Button>
      <Button icon={<ReloadOutlined />} loading={loading} onClick={onRefresh}>
        刷新
      </Button>
      <Space size={4}>
        <span style={{ color: "#666" }}>自动刷新</span>
        <Switch
          checked={autoRefresh}
          onChange={onAutoRefreshChange}
          size="small"
        />
      </Space>
      {isAdmin ? (
        <Tag color="green" icon={<SafetyCertificateOutlined />}>
          管理员模式
        </Tag>
      ) : (
        <Tag color="orange" icon={<SafetyCertificateOutlined />}>
          建议以管理员身份运行
        </Tag>
      )}
    </Space>
  );
}
