import { useAccessStore } from '@vben/stores';

import { invoke } from '@tauri-apps/api/tauri';
import { message } from 'ant-design-vue';

import { baseRequestClient, requestClient } from '#/api/request';

export namespace AuthApi {
  /** 登录接口参数 */
  export interface LoginParams {
    password?: string;
    username?: string;
  }

  export interface RegisterParams {
    password?: string;
    username?: string;
    nickname?: string;
  }

  /** 登录接口返回值 */
  export interface LoginResult {
    accessToken: string;
    id: string;
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
    ? invoke('route_cmd', {
        command: 'user_login',
        args: {
          ...data,
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return {
            accessToken: '',
            id: '',
          } as AuthApi.LoginResult;
        }
        return msg.result as AuthApi.LoginResult;
      })
    : requestClient.post<AuthApi.LoginResult>('/auth/login', data);
}

/**
 * 刷新accessToken
 */
export async function refreshTokenApi() {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('refresh_token_cmd', {
        command: 'user_refresh_token',
        args: {
          accessToken: accessStore.accessToken,
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return {
            data: accessStore.accessToken,
            status: 0,
          } as AuthApi.RefreshTokenResult;
        }
        return msg.result as AuthApi.RefreshTokenResult;
      })
    : baseRequestClient.post<AuthApi.RefreshTokenResult>('/auth/refresh', {
        withCredentials: true,
      });
}

/**
 * 退出登录
 */
export async function logoutApi() {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'user_logout',
        accessToken: accessStore.accessToken,
        args: {},
      }).then((message: any) => {
        return message.result;
      })
    : baseRequestClient.post('/auth/logout', {
        withCredentials: true,
      });
}

/**
 * 获取用户权限码
 */
export async function getAccessCodesApi() {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'user_get_access_codes',
        accessToken: accessStore.accessToken,
        args: {},
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return [];
        }
        return msg as string[];
      })
    : requestClient.get<string[]>('/auth/codes');
}

export async function registerApi(data: AuthApi.RegisterParams) {
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'user_register',
        args: {
          ...data,
        },
      }).then((msg: any) => {
        if (msg.code === 0) {
          message.success('register successfully!');
          return msg.result as AuthApi.LoginResult;
        } else {
          message.error(msg.message);
        }
      })
    : requestClient.post<AuthApi.LoginResult>('/auth/register', data);
}
