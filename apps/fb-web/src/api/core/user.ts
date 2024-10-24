import type { UserInfo } from '@vben/types';

import { invoke } from '@tauri-apps/api/tauri';

import { requestClient } from '#/api/request';

/**
 * 获取用户信息
 */
export async function getUserInfoApi() {
  return window.__TAURI__
    ? invoke('get_user_info_cmd', {}).then((message: any) => {
        return message.result as UserInfo;
      })
    : requestClient.get<UserInfo>('/user/info');
}
