import { invoke } from '@tauri-apps/api/tauri';

export interface DeepChatRequestMessage {
  role: string;
  text: string;
}

export interface DeepChatRequestBody {
  messages: DeepChatRequestMessage[];
  model: string;
  stream: boolean;
}

export interface DeepChatTextResponse {
  text: string;
}

export interface Model {
  name: string;
  model: string;
  modified_at: string;
  size: number;
  digest: string;
}

export interface ModelData {
  models: Model[];
}
/**
 * 获取用户信息
 */
export async function chatRequest(body: DeepChatRequestBody) {
  return window.__TAURI__
    ? invoke('chat_request_cmd', {
        body,
      }).then((message: any) => {
        return message as DeepChatTextResponse;
      })
    : new Promise((resolve) => {
        const response: DeepChatTextResponse = {
          text: 'Sorry, this can used in app mode only.',
        };
        resolve(response);
      });
}

export async function getModels() {
  return window.__TAURI__
    ? invoke('get_models_cmd', {}).then((message: any) => {
        return message as ModelData;
      })
    : new Promise((resolve) => {
        const response: ModelData = {
          models: [],
        };
        resolve(response);
      });
}
