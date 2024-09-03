import { invoke } from '@tauri-apps/api/tauri';

import { requestClient } from '#/api/request';

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
/**
 * 获取用户信息
 */
export async function chatRequest(body: DeepChatRequestBody) {
  return window.__TAURI__
    ? invoke('chat_request_cmd', {
        body: {
          messages: body.messages,
          model: '',
          stream: false,
        },
      }).then((message: any) => {
        return message as DeepChatTextResponse;
      })
    : requestClient.get<DeepChatTextResponse>('/local/chat');
}
