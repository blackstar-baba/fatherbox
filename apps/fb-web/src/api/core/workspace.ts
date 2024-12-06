import type { WorkspaceInfo } from '#/types';

import { useAccessStore } from '@vben/stores';

import { invoke } from '@tauri-apps/api/tauri';
import { message } from 'ant-design-vue';

/**
 * get workspace list
 */
export async function getAllWorkspaceApi() {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'workspace_list',
        accessToken: accessStore.accessToken,
        args: {},
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return [];
        }
        return msg.result as WorkspaceInfo[];
      })
    : Promise.all([
        {
          id: '1',
          name: 'test',
        } as WorkspaceInfo,
        {
          id: '2',
          name: 'test2',
        } as WorkspaceInfo,
      ]);
}
