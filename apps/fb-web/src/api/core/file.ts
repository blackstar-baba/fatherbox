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
  size: number;
  create_time: number;
  update_time: number;
}

export interface FileCreateBody {
  name: string;
  pid: string;
  content?: string;
  path?: string;
  type: string;
}

export interface FileCopyBody {
  name: string;
  pid: string;
  fromId: string;
}

export interface FileUpdateContentBody {
  id: string;
  content: string;
}

export interface FileUpdateNameBody {
  id: string;
  name: string;
}

export interface FileUpdateBody {
  id: string;
  name: string;
  pid: string;
}

export interface FileSearchBody {
  pageSize: number;
  pageNum: number;
  pid: string;
  type: string;
  name: string;
}

export async function getAllWorkspaceFiles(type?: string): Promise<File[]> {
  const accessStore = useAccessStore();
  const workspaceStore = useWorkspaceStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'file_get_all_workspace_files',
        accessToken: accessStore.accessToken,
        args: {
          wid: workspaceStore.getId(),
          type,
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return [];
        }
        return msg.result as File[];
      })
    : Promise.all([
        {
          id: '1-1',
          name: 'test-dir',
          pid: workspaceStore.getId(),
          type: 'dir',
        },
        {
          id: '1-2',
          name: 'test-file',
          pid: workspaceStore.getId(),
          type: 'file',
        },
        {
          id: '1-3',
          name: 'test-file-2',
          pid: '1-1',
          type: 'file',
        },
      ] as File[]);
}

export async function getWorkspaceFilesByPid(pid: string, type?: string) {
  const accessStore = useAccessStore();
  const workspaceStore = useWorkspaceStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'file_get_workspace_files_by_id',
        accessToken: accessStore.accessToken,
        args: {
          wid: workspaceStore.getId(),
          pid,
          type,
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return [];
        }
        return msg.result as File[];
      })
    : Promise.all([
        {
          id: '1-1',
          name: 'test-dir',
          pid: workspaceStore.getId(),
          type: 'dir',
          size: 0,
        },
        {
          id: '1-2',
          name: 'test-file',
          pid: workspaceStore.getId(),
          type: 'file',
          size: 123_131,
        },
        {
          id: '1-3',
          name: 'test-file-2',
          pid: '1-1',
          type: 'file',
          size: 11_232,
        },
      ] as File[]);
}

export async function getWorkspaceFilesByPage(body: FileSearchBody) {
  const accessStore = useAccessStore();
  const workspaceStore = useWorkspaceStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'file_get_workspace_files_by_page',
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
        return msg.result;
      })
    : new Promise((resolve) => {
        resolve({
          total: 3,
          items: [
            {
              id: '1-1',
              name: 'test-dir',
              pid: workspaceStore.getId(),
              createTime: 1_734_533_523,
              updateTime: 1_734_533_523,
              type: 'dir',
              size: 0,
            },
            {
              id: '1-2',
              name: 'test-file',
              pid: workspaceStore.getId(),
              createTime: 1_734_533_524,
              updateTime: 1_734_533_525,
              type: 'file',
              size: 123_131,
            },
          ],
        });
      });
}

export async function getFiles() {
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
    : Promise.all([
        {
          id: '1-1',
          name: 'test-dir',
          pid: workspaceStore.getId(),
          type: 'dir',
        },
        {
          id: '1-2',
          name: 'test-file',
          pid: workspaceStore.getId(),
          type: 'file',
        },
        {
          id: '1-3',
          name: 'test-file-2',
          pid: '1-1',
          type: 'file',
        },
      ] as File[]);
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

export async function copyFile(body: FileCopyBody) {
  const accessStore = useAccessStore();
  const workspaceStore = useWorkspaceStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'file_copy',
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

export async function updateFileContent(body: FileUpdateContentBody) {
  const accessStore = useAccessStore();
  const workspaceStore = useWorkspaceStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'file_update_content',
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

export async function updateFileName(body: FileUpdateNameBody) {
  const accessStore = useAccessStore();
  const workspaceStore = useWorkspaceStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'file_update_name',
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
