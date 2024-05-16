export interface Response<T> {
  code: number;
  type: string;
  message: string;
  result: T;
}

export interface ModelItem {
  name: string;
  model: string;
  modified_at: string;
  size: bigint;
  digest: string;
}

export interface ModelItems {
  models: ModelItem[];
}
