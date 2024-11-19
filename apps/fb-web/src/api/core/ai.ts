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

export interface ChatInfo {
  id: string;
  name: string;
}
/**
 * 获取用户信息
 */
export async function chatRequest(body: DeepChatRequestBody) {
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'model_chat_request',
        args: {
          ...body,
        },
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
    ? invoke('route_cmd', {
        command: 'model_get_models',
        args: {},
      }).then((message: any) => {
        return message as ModelData;
      })
    : new Promise((resolve) => {
        const response: ModelData = {
          models: [],
        };
        resolve(response);
      });
}

export async function getChats() {
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'model_get_chats',
        args: {},
      }).then((message: any) => {
        return message as ChatInfo[];
      })
    : new Promise((resolve) => {
        const response: ChatInfo[] = [];
        resolve(response);
      });
}

export async function getChatHistoryMessages(id: string) {
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'model_get_chat_history_messages',
        args: { id },
      }).then((message: any) => {
        return message as DeepChatRequestMessage[];
      })
    : new Promise((resolve) => {
        const response: DeepChatRequestMessage[] = [];
        resolve(response);
      });
}
