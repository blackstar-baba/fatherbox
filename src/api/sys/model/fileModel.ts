import { BasicPageParams } from '@/api/model/baseModel';

export interface FileEntry {
  name: string;
  type: string;
  path: string;
  size: number;
  createTime: number;
  updateTime: number;
  children: FileEntry[];
  parentPath: string;
}

export interface SimpleFileEntry {
  name: string;
  type: string;
  id: string;
  size: number;
  createTime: string;
  updateTime: string;
  pid: string;
}

export type FileParams = BasicPageParams & {
  pid: string;
};
