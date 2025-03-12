import { useAccessStore } from '@vben/stores';

import { invoke } from '@tauri-apps/api/tauri';
import { message } from 'ant-design-vue';

export interface Source {
  id: string;
  name: string;
  buildin: boolean;
  url: string;
  key: string;
  enable: boolean;
  sync: boolean;
}

export interface Model {
  id: string;
  name: string;
  cId: string;
  enable: boolean;
}

export async function listAiSource() {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'ai_source_list',
        accessToken: accessStore.accessToken,
        args: {},
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return [] as Source[];
        }
        return msg.result as Source[];
      })
    : new Promise<Source[]>((resolve: any) => {
        // todo use http client replace this
        resolve([]);
      });
}

export async function listEnableAiSource() {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'ai_source_list_enable',
        accessToken: accessStore.accessToken,
        args: {},
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return [] as Source[];
        }
        return msg.result as Source[];
      })
    : new Promise<Source[]>((resolve: any) => {
        // todo use http client replace this
        resolve([]);
      });
}

export async function createAiSource(params: {
  key: string;
  name: string;
  url: string;
}) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'ai_source_create',
        accessToken: accessStore.accessToken,
        args: {
          ...params,
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return {
            id: '',
            name: '',
            buildin: false,
            url: '',
            key: '',
            enable: false,
            sync: false,
          } as Source;
        }
        return msg.result as Source;
      })
    : new Promise<Source>((resolve: any) => {
        // todo use http client replace this
        resolve({
          id: '',
          name: '',
          buildin: false,
          url: '',
          key: '',
          enable: false,
          sync: false,
        } as Source);
      });
}

export async function updateAiSource(params: {
  id: string;
  key: string;
  name: string;
  url: string;
}) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'ai_source_update',
        accessToken: accessStore.accessToken,
        args: {
          ...params,
        },
      }).then((message: any) => {
        if (message.code !== 0) {
          message.error(message.message);
          return {
            id: '',
            name: '',
            buildin: false,
            url: '',
            key: '',
            enable: false,
            sync: false,
          } as Source;
        }
        return message.result as Source;
      })
    : new Promise<Source>((resolve: any) => {
        // todo use http client replace this
        resolve({
          id: '',
          name: '',
          buildin: false,
          url: '',
          key: '',
          enable: false,
          sync: false,
        } as Source);
      });
}

export async function deleteAiSource(params: { id: string }) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'ai_source_delete',
        accessToken: accessStore.accessToken,
        args: {
          ...params,
        },
      }).then((message: any) => {
        if (message.code !== 0) {
          message.error(message.message);
          return {} as Source;
        }
        return message.result as Source;
      })
    : new Promise<any>((resolve) => {
        // todo use http client replace this
        resolve({} as Source);
      });
}

export async function enableAiSource(params: { enable: boolean; id: string }) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'ai_source_enable',
        accessToken: accessStore.accessToken,
        args: {
          ...params,
        },
      }).then((message: any) => {
        if (message.code !== 0) {
          message.error(message.message);
          return {} as Source;
        }
        return message.result as Source;
      })
    : new Promise<Source>((resolve: any) => {
        // todo use http client replace this
        resolve({} as Source);
      });
}

export async function listAiSourceModels(params: { sourceId: string }) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'ai_model_list',
        accessToken: accessStore.accessToken,
        args: {
          ...params,
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return [] as Model[];
        }
        return msg.result as Model[];
      })
    : new Promise<Model[]>((resolve: any) => {
        // todo use http client replace this
        resolve([]);
      });
}

export async function listEnableAiSourceModels(params: { sourceId: string }) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'ai_model_list_enable',
        accessToken: accessStore.accessToken,
        args: {
          ...params,
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return [] as Model[];
        }
        return msg.result as Model[];
      })
    : new Promise<Model[]>((resolve: any) => {
        // todo use http client replace this
        resolve([]);
      });
}

export async function createAiSourceModel(params: {
  name: string;
  sourceId: string;
}) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'ai_model_create',
        accessToken: accessStore.accessToken,
        args: {
          ...params,
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return {} as Model;
        }
        return msg.result as Model;
      })
    : new Promise<Model>((resolve: any) => {
        // todo use http client replace this
        resolve({} as Model);
      });
}

export async function updateAiSourceModel(params: {
  id: string;
  name: string;
}) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'ai_model_update',
        accessToken: accessStore.accessToken,
        args: {
          ...params,
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return {} as Model;
        }
        return msg.result as Model;
      })
    : new Promise<Model>((resolve: any) => {
        // todo use http client replace this
        resolve({} as Model);
      });
}

export async function deleteAiSourceModel(params: { id: string }) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'ai_model_delete',
        accessToken: accessStore.accessToken,
        args: {
          ...params,
        },
      }).then((msg: any) => {
        if (msg.code !== 0) {
          message.error(msg.message);
          return {} as Model;
        }
        return msg.result as Model;
      })
    : new Promise<Model>((resolve: any) => {
        // todo use http client replace this
        resolve({} as Model);
      });
}

export async function enableAiSourceModel(params: {
  enable: boolean;
  id: string;
}) {
  const accessStore = useAccessStore();
  return window.__TAURI__
    ? invoke('route_cmd', {
        command: 'ai_model_enable',
        accessToken: accessStore.accessToken,
        args: {
          ...params,
        },
      }).then((message: any) => {
        if (message.code !== 0) {
          message.error(message.message);
          return {} as Model;
        }
        return message.result as Model;
      })
    : new Promise<Model>((resolve: any) => {
        // todo use http client replace this
        resolve({} as Model);
      });
}
