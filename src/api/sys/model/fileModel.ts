import { BasicPageParams } from '@/api/model/baseModel';

export interface FileEntry {
  name: string;
  type: string;
  path: string;
  size: number;
  createTime: number;
  modifyTime: number;
  children: FileEntry[];
  parentPath: string;
}

export interface SimpleFileEntry {
  name: string;
  type: string;
  id: string;
  size: number;
  createTime: number;
  modifyTime: number;
  pid: string;
}

export type FileParams = BasicPageParams & {
  pid: string;
};
