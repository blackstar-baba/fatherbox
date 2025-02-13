import { useAccessStore } from '@vben/stores';

import { invoke } from '@tauri-apps/api/tauri';

export interface ChatMessage {
  role: string;
  content: string;
  error?: boolean;
  loading: boolean;
  time?: string;
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
  created_at: string;
  oldName?: string;
  isEdit: boolean;
}

export interface MessageResponse {
  id: string;
  index: number;
  text?: string;
  error?: string;
}

export async function getModels() {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'model_get_models',
        accessToken: accessStore.accessToken,
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
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'chat_list',
        accessToken: accessStore.accessToken,
        args: {},
      }).then((message: any) => {
        return message as ChatInfo[];
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
      }).then((message: any) => {
        return message as ChatInfo;
      })
    : new Promise<ChatInfo>((resolve) => {
        // todo use http client replace this
        resolve({
          ...params,
        } as ChatInfo);
      });
}

export async function createChat(params: { name: string }) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'chat_create',
        accessToken: accessStore.accessToken,
        args: {
          ...params,
        },
      }).then((message: any) => {
        return message as ChatInfo;
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
      }).then((message: any) => {
        return message as ChatMessage[];
      })
    : new Promise<ChatMessage[]>((resolve) => {
        const response: ChatMessage[] = [];
        resolve(response);
      });
}

// todo
// 思考过程

export async function fetchChatAPIProcessNew(params: {
  id: string;
  model: string;
  onDownloadProgress?: (data: any) => void;
  parentMessageId?: number;
  prompt: string;
}) {
  const accessStore = useAccessStore();
  invoke('route_cmd', {
    command: 'chat_message_request',
    accessToken: accessStore.accessToken,
    args: {
      id: params.id,
      prompt: params.prompt,
      model: params.model,
      stream: false,
    },
  }).then((response: any) => {
    if (params.onDownloadProgress) {
      params.onDownloadProgress(response.text ?? response.error);
    }
  });
}

export async function regenerateMessage(params: {
  id: string;
  index: number;
  model: string;
  onDownloadProgress?: (data: any) => void;
}) {
  const accessStore = useAccessStore();
  invoke('route_cmd', {
    command: 'chat_message_regenerate',
    accessToken: accessStore.accessToken,
    args: {
      id: params.id,
      index: params.index,
      model: params.model,
      stream: false,
    },
  }).then((response: any) => {
    if (params.onDownloadProgress) {
      params.onDownloadProgress(response.text ?? response.error);
    }
  });
}

export async function editMessage(params: {
  id: string;
  index: number;
  model: string;
  onDownloadProgress?: (data: any) => void;
  prompt: string;
}) {
  const accessStore = useAccessStore();
  invoke('route_cmd', {
    command: 'chat_message_edit',
    accessToken: accessStore.accessToken,
    args: {
      id: params.id,
      index: params.index,
      prompt: params.prompt,
      model: params.model,
      stream: false,
    },
  }).then((response: any) => {
    if (params.onDownloadProgress) {
      params.onDownloadProgress(response.text ?? response.error);
    }
  });
}
