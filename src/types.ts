export interface ProcessInfo {
  pid: number;
  name: string;
  cpu: number;
  memory: number;
  path: string | null;
  ports: number[];
  protected: boolean;
}

export interface KillResult {
  pid: number;
  ok: boolean;
  error: string | null;
}
