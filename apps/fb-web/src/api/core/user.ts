import type { UserInfo } from '@vben/types';

import { useAccessStore } from '@vben/stores';

import { invoke } from '@tauri-apps/api/tauri';
import { message } from 'ant-design-vue';

import { requestClient } from '#/api/request';

/**
 * 获取用户信息
 */
export async function getUserInfoApi() {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'user_get_info',
        accessToken: accessStore.accessToken,
        args: {},
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return null;
        }
        return msg.result as UserInfo;
      })
    : requestClient.get<UserInfo>('/user/info');
}
