import { invoke } from '@tauri-apps/api/tauri';

import { requestClient } from '#/api/request';

export namespace AuthApi {
  /** 登录接口参数 */
  export interface LoginParams {
    password: string;
    username: string;
  }

  /** 登录接口返回值 */
  export interface LoginResult {
    accessToken: string;
    desc: string;
    realName: string;
    refreshToken: string;
    userId: string;
    username: string;
  }
}

/**
 * 登录
 */
export async function loginApi(data: AuthApi.LoginParams) {
  return window.__TAURI__
    ? invoke('user_login_cmd', {
        password: data.password,
        username: data.username,
      }).then((message: any) => {
        return message as AuthApi.LoginResult;
      })
    : requestClient.post<AuthApi.LoginResult>('/auth/login', data);
}

/**
 * 获取用户权限码
 */
export async function getAccessCodesApi() {
  return requestClient.get<string[]>('/auth/codes');
}
