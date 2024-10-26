import type { UserInfo } from '@vben/types';

import { useAccessStore } from '@vben/stores';

import { invoke } from '@tauri-apps/api/tauri';
import { message } from 'ant-design-vue';

import { requestClient } from '#/api/request';

/**
 * 获取用户信息
 */
export async function getUserInfoApi() {
  // todo user not login can not redirect to dashboard
  // 关键问题在于处理guard.ts中的 accessToken，系统根据accessToken判断是否可以go on，
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('intercepted_command', {
        command: 'get_user_info',
        accessToken: accessStore.accessToken,
        args: {
          test: 'hello',
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return {} as UserInfo;
        }
        return msg.result as UserInfo;
      })
    : requestClient.get<UserInfo>('/user/info');
}
