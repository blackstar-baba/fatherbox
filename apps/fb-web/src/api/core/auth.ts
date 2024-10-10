import { invoke } from '@tauri-apps/api/tauri';

import { baseRequestClient, requestClient } from '#/api/request';

export namespace AuthApi {
  /** 登录接口参数 */
  export interface LoginParams {
    password: string;
    username: string;
  }

  /** 登录接口返回值 */
  export interface LoginResult {
    accessToken: string;
  }

  export interface RefreshTokenResult {
    data: string;
    status: number;
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
 * 刷新accessToken
 */
export async function refreshTokenApi() {
  return window.__TAURI__
    ? invoke('refresh_token_cmd', {}).then((message: any) => {
        return message as AuthApi.RefreshTokenResult;
      })
    : baseRequestClient.post<AuthApi.RefreshTokenResult>('/auth/refresh', {
        withCredentials: true,
      });
}

/**
 * 退出登录
 */
export async function logoutApi() {
  // todo
  return window.__TAURI__
    ? new Promise((resolve) => {
        resolve('');
      })
    : baseRequestClient.post('/auth/logout', {
        withCredentials: true,
      });
}

/**
 * 获取用户权限码
 */
export async function getAccessCodesApi() {
  return window.__TAURI__
    ? invoke('get_access_codes_cmd', {}).then((message: any) => {
        return message as string[];
      })
    : requestClient.get<string[]>('/auth/codes');
}
