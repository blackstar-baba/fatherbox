import { useAccessStore } from '@vben/stores';

import { readTextFile } from '@tauri-apps/api/fs';
import { invoke } from '@tauri-apps/api/tauri';
import { message } from 'ant-design-vue';

import { useWorkspaceStore } from '#/store';

export interface File {
  id: string;
  name: string;
  pid: string;
  type: string;
}

export interface FileCreateBody {
  name: string;
  pid: string;
  content?: string;
  path?: string;
  type: string;
}

export interface FileUpdateBody {
  id: string;
  content: string;
}

export async function getAllWorkspaceFiles() {
  const accessStore = useAccessStore();
  const workspaceStore = useWorkspaceStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'file_get_all_workspace_files',
        accessToken: accessStore.accessToken,
        args: {
          wid: workspaceStore.getId(),
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return [];
        }
        return msg.result as File[];
      })
    : Promise.all([]);
}

export async function getFile(pid: string) {
  const accessStore = useAccessStore();
  const workspaceStore = useWorkspaceStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'file_get',
        accessToken: accessStore.accessToken,
        args: {
          wid: workspaceStore.getId(),
          pid,
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return [];
        }
        return msg.result as File;
      })
    : new Promise((resolve) => {
        resolve({});
      });
}
export async function createFile(body: FileCreateBody) {
  const accessStore = useAccessStore();
  const workspaceStore = useWorkspaceStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'file_create',
        accessToken: accessStore.accessToken,
        args: {
          wid: workspaceStore.getId(),
          ...body,
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return {};
        }
        return msg.result as File;
      })
    : new Promise((resolve) => {
        resolve({});
      });
}

export async function updateFile(body: FileUpdateBody) {
  const accessStore = useAccessStore();
  const workspaceStore = useWorkspaceStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'file_update',
        accessToken: accessStore.accessToken,
        args: {
          wid: workspaceStore.getId(),
          ...body,
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return [];
        }
        return msg.result as File;
      })
    : new Promise((resolve) => {
        resolve({});
      });
}
export async function deleteFile(id: string) {
  const accessStore = useAccessStore();
  const workspaceStore = useWorkspaceStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'file_delete',
        accessToken: accessStore.accessToken,
        args: {
          wid: workspaceStore.getId(),
          id,
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return [];
        }
        return msg.result;
      })
    : new Promise((resolve) => {
        resolve({});
      });
}

export async function getFileContent(id: string) {
  const accessStore = useAccessStore();
  const workspaceStore = useWorkspaceStore();
  if (window.__TAURI__) {
    const filePath = await invoke('route_cmd', {
      command: 'file_get_path',
      accessToken: accessStore.accessToken,
      args: {
        wid: workspaceStore.getId(),
        id,
      },
    }).then((msg: any) => {
      if (msg.code !== 0) {
        message.error(msg.message);
        return '';
      }
      return msg.result as string;
    });
    if (filePath.length > 0) {
      return readTextFile(filePath);
    }
    return new Promise((resolve) => {
      resolve('');
    });
  }
  return new Promise((resolve) => {
    resolve('');
  });
}
