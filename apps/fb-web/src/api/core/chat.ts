import { useAccessStore } from '@vben/stores';

import { invoke } from '@tauri-apps/api/tauri';
import { v4 as uuidv4 } from 'uuid';

export interface ChatMessage {
  role: string;
  content: string;
  error?: boolean;
  loading: boolean;
}

// export interface Model {
//   name: string;
//   model: string;
//   modified_at: string;
//   size: number;
//   digest: string;
// }
//
// export interface ModelData {
//   models: Model[];
// }

export interface ChatInfo {
  id: string;
  name: string;
  created_at: string;
  oldName?: string;
  isEdit: boolean;
}

// export async function getModels() {
//   const accessStore = useAccessStore();
//   return window.__TAURI__
//     ? invoke('route_cmd', {
//         command: 'chat_get_models',
//         accessToken: accessStore.accessToken,
//         args: {},
//       }).then((msg: any) => {
//         return msg.result;
//       })
//     : new Promise((resolve) => {
//         const response: ModelData = {
//           models: [],
//         };
//         resolve(response);
//       });
// }

export async function getChats(params: { wid: string }) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'chat_list',
        accessToken: accessStore.accessToken,
        args: {
          ...params,
        },
      }).then((msg: any) => {
        return msg.result as ChatInfo[];
      })
    : new Promise<ChatInfo[]>((resolve) => {
        const response: ChatInfo[] = [];
        resolve(response);
      });
}

export async function updateChatName(params: { id: string; name: string }) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'chat_update_name',
        accessToken: accessStore.accessToken,
        args: {
          ...params,
        },
      }).then((msg: any) => {
        return msg.result as ChatInfo;
      })
    : new Promise<ChatInfo>((resolve) => {
        // todo use http client replace this
        resolve({
          ...params,
        } as ChatInfo);
      });
}

export async function createChat(params: { name: string; wid: string }) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'chat_create',
        accessToken: accessStore.accessToken,
        args: {
          ...params,
        },
      }).then((msg: any) => {
        return msg.result as ChatInfo;
      })
    : new Promise<ChatInfo>((resolve: any) => {
        // todo use http client replace this
        resolve({
          id: '',
          name: '',
          created_at: '',
        } as ChatInfo);
      });
}
export async function deleteChat(params: { id: string }) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'chat_delete',
        accessToken: accessStore.accessToken,
        args: {
          ...params,
        },
      }).then(() => {
        return {};
      })
    : new Promise<any>((resolve) => {
        // todo use http client replace this
        resolve({});
      });
}

export async function getChatMessages(params: { id: string }) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'chat_message_list',
        accessToken: accessStore.accessToken,
        args: { ...params },
      }).then((msg: any) => {
        return msg.result as ChatMessage[];
      })
    : new Promise<ChatMessage[]>((resolve) => {
        const response: ChatMessage[] = [];
        resolve(response);
      });
}

// todo support thinking style

export async function generateChatMessageWithStream(params: {
  id: string;
  modelId: string;
  onProgress: (data: any, status: number) => void;
  parentMessageId?: number;
  prompt: string;
  sourceId: string;
}) {
  const accessStore = useAccessStore();
  if (window.__TAURI__) {
    const requestId = uuidv4().toString();
    // @ts-ignore event & ResponseEvent exist
    window.__TAURI__.event.listen(requestId, (e: ResponseEvent) => {
      const { chunk, status } = e?.payload || {};
      params.onProgress(chunk, status);
    });
    return invoke('route_cmd', {
      command: 'chat_message_request',
      accessToken: accessStore.accessToken,
      args: {
        id: params.id,
        prompt: params.prompt,
        modelId: params.modelId,
        sourceId: params.sourceId,
        requestId,
      },
    }).then((res: any) => {
      return res.result;
    });
  }
  return new Promise((resolve: any) => {
    resolve({});
  });
}

export async function regenerateChatMessageWithStream(params: {
  id: string;
  index: number;
  modelId: string;
  onProgress: (data: any, status: number) => void;
  sourceId: string;
}) {
  // { id: string; index: number; modelId: string; sourceId: string; onProgress: (message: null | string, _: number) => void;
  // { id: string; index: number; messageId: number; modelId: string; onProgress: (data: any, status: number) => void; sourceId: string; }
  const accessStore = useAccessStore();
  if (window.__TAURI__) {
    const requestId = uuidv4().toString();
    // @ts-ignore event & ResponseEvent exist
    window.__TAURI__.event.listen(requestId, (e: ResponseEvent) => {
      const { chunk, status } = e?.payload || {};
      // if (chunk) {
      //   message.info(`get stream chunk:${chunk}`);
      // } else {
      //   message.info(`get stream status:${status}`);
      // }
      params.onProgress(chunk, status);
    });
    return invoke('route_cmd', {
      command: 'chat_message_regenerate',
      accessToken: accessStore.accessToken,
      args: {
        id: params.id,
        index: params.index,
        modelId: params.modelId,
        sourceId: params.sourceId,
        requestId,
      },
    }).then((res: any) => {
      return res.result;
    });
  }
  return new Promise((resolve: any) => {
    resolve({});
  });
}

export async function editChatMessageWithStream(params: {
  id: string;
  index: number;
  modelId: string;
  onProgress: (data: any, status: number) => void;
  prompt: string;
  sourceId: string;
}) {
  const accessStore = useAccessStore();
  if (window.__TAURI__) {
    const requestId = uuidv4().toString();
    // @ts-ignore event & ResponseEvent exist
    window.__TAURI__.event.listen(requestId, (e: ResponseEvent) => {
      const { chunk, status } = e?.payload || {};
      params.onProgress(chunk, status);
    });
    return invoke('route_cmd', {
      command: 'chat_message_edit',
      accessToken: accessStore.accessToken,
      args: {
        id: params.id,
        index: params.index,
        prompt: params.prompt,
        modelId: params.modelId,
        sourceId: params.sourceId,
        requestId,
      },
    }).then((res: any) => {
      return res.result;
    });
  }
  return new Promise((resolve: any) => {
    resolve({});
  });
}
