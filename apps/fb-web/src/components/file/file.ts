import { h } from 'vue';

import {
  CopyOutlined,
  DeleteOutlined,
  EditOutlined,
  PlusOutlined,
} from '@ant-design/icons-vue';
import { Button } from 'ant-design-vue';

import { type File, getAllWorkspaceFiles } from '#/api';
import LocalUpload from '#/components/file/LocalUpload.vue';
import { useWorkspaceStore } from '#/store';

export const FILE_TYPE_DIR = 'dir';
export const FILE_TYPE_FILE = 'file';

const decoder = new TextDecoder('utf8');
const encoder = new TextEncoder();

export interface FileContent {
  id: string;
  name: string;
  type?: string;
  content: Uint8Array;
}

interface TreeItem {
  key: string;
  value: string;
  title: string;
  pkey?: string;
  children?: TreeItem[];
  disabled?: boolean;
}

export const CREATE_FORM_SCHEMA: any[] = [
  {
    component: 'Input',
    componentProps: {
      placeholder: '请输入',
    },
    fieldName: 'name',
    label: 'Name',
    rules: 'required',
  },
  {
    component: 'TreeSelect',
    componentProps: {
      class: 'w-full',
      allowClear: false,
      placeholder: 'Please select',
      showSearch: false,
      treeData: [],
      treeNodeFilterProp: 'label',
    },
    fieldName: 'pid',
    label: 'Parent',
  },
  {
    component: 'RadioGroup',
    componentProps: {
      options: [
        { label: 'Dir', value: FILE_TYPE_DIR },
        { label: 'File', value: FILE_TYPE_FILE },
      ],
    },
    fieldName: 'type',
    label: 'Type',
    defaultValue: 'dir',
  },
  {
    component: 'RadioGroup',
    componentProps: {
      options: [
        { label: 'input', value: 'input' },
        { label: 'upload', value: 'upload' },
      ],
    },
    fieldName: 'inputOrUpload',
    label: '',
    defaultValue: 'input',
    dependencies: {
      if(values: any) {
        return values.type === FILE_TYPE_FILE;
      },
      triggerFields: ['type'],
    },
  },
  {
    component: 'Textarea',
    componentProps: {
      placeholder: 'Please Input',
    },
    fieldName: 'content',
    label: 'Content',
    dependencies: {
      if(values: any) {
        return (
          values.type === FILE_TYPE_FILE && values.inputOrUpload === 'input'
        );
      },
      triggerFields: ['type', 'inputOrUpload'],
    },
  },
  {
    fieldName: 'path',
    label: 'file',
    component: h(LocalUpload),
    dependencies: {
      if(values: any) {
        return (
          values.type === FILE_TYPE_FILE && values.inputOrUpload === 'upload'
        );
      },
      triggerFields: ['type', 'inputOrUpload'],
    },
  },
];

export const UPDATE_FORM_SCHEMA: any[] = [
  {
    component: 'Input',
    controlClass: 'hidden',
    fieldName: 'id',
    label: 'Id',
    hideLabel: true,
  },
  {
    component: 'Input',
    componentProps: {
      placeholder: '请输入',
    },
    fieldName: 'name',
    label: 'Name',
    rules: 'required',
  },
  {
    component: 'TreeSelect',
    componentProps: {
      class: 'w-full',
      allowClear: false,
      placeholder: '请选择',
      showSearch: false,
      treeData: [],
      treeNodeFilterProp: 'label',
    },
    fieldName: 'pid',
    label: 'Parent',
  },
];

export const COPY_FORM_SCHEMA: any[] = [
  {
    component: 'Input',
    controlClass: 'hidden',
    fieldName: 'fromId',
    label: 'From Id',
    hideLabel: true,
  },
  {
    component: 'Input',
    componentProps: {
      placeholder: '请输入',
      disabled: true,
    },
    fieldName: 'fromName',
    label: 'From File',
    rules: 'required',
  },
  {
    component: 'Input',
    componentProps: {
      placeholder: '请输入',
    },
    fieldName: 'name',
    label: 'Name',
    rules: 'required',
  },
  {
    component: 'TreeSelect',
    componentProps: {
      class: 'w-full',
      allowClear: false,
      placeholder: '请选择',
      showSearch: false,
      treeData: [],
      treeNodeFilterProp: 'label',
    },
    fieldName: 'pid',
    label: 'Parent',
  },
];

export const ROOT_MENU = [
  {
    key: 'create',
    icon: () => h(Button, { size: 'small', type: 'primary' }, h(PlusOutlined)),
  },
];

export const FILE_MENU = [
  {
    key: 'copy',
    icon: () => h(Button, { size: 'small', type: 'primary' }, h(CopyOutlined)),
  },
  {
    key: 'edit',
    icon: () => h(Button, { size: 'small', type: 'primary' }, h(EditOutlined)),
  },
  {
    key: 'delete',
    icon: () =>
      h(Button, { size: 'small', type: 'primary' }, h(DeleteOutlined)),
  },
];

export const DIR_MENU = [
  {
    key: 'create',
    icon: () => h(Button, { size: 'small', type: 'primary' }, h(PlusOutlined)),
  },
  {
    key: 'edit',
    icon: () => h(Button, { size: 'small', type: 'primary' }, h(EditOutlined)),
  },
  {
    key: 'delete',
    icon: () =>
      h(Button, { size: 'small', type: 'primary' }, h(DeleteOutlined)),
  },
];

export const TYPE_MD = 'md';
export const TYPE_IMAGE = 'image';

export const TYPE_TXT = 'txt';

export const TYPE_UNKNOWN = 'unknown';

export const getFileExtension = (name: string) => {
  const lastDotIndex = name.lastIndexOf('.');
  let fileExtension = '';
  if (lastDotIndex !== -1) {
    fileExtension = name.slice(lastDotIndex + 1);
  }
  return fileExtension;
};

export const getFileType = (name: string) => {
  const fileExtension = getFileExtension(name);
  let fileType = TYPE_UNKNOWN;
  switch (fileExtension) {
    case 'jpg': {
      fileType = TYPE_IMAGE;
      break;
    }
    case 'json': {
      fileType = TYPE_TXT;
      break;
    }
    case 'md': {
      fileType = TYPE_MD;
      break;
    }
    case 'MD': {
      fileType = TYPE_MD;
      break;
    }
    case 'png': {
      fileType = TYPE_IMAGE;
      break;
    }
    case 'svg': {
      fileType = TYPE_IMAGE;
      break;
    }
    case 'txt': {
      fileType = TYPE_TXT;
      break;
    }
  }
  return fileType;
};

export const decodeUint8ArrayToString = (content: Uint8Array) => {
  return decoder.decode(content);
};

export const encodeStringToUint8Array = (content: string) => {
  return encoder.encode(content);
};

export const uint8ArrayToImageSrc = (name: string, content: Uint8Array) => {
  // 创建一个Blob对象
  const extension = getFileExtension(name);
  const blob = new Blob([content], { type: `image/${extension}` });
  // 创建一个指向Blob内容的URL
  // 返回创建的URL
  return URL.createObjectURL(blob);
};

const getTreeChildren = (key: string, map: Map<string, File[]>) => {
  const children: TreeItem[] = [];
  const value = map.get(key);
  if (value) {
    value.forEach((element: File) => {
      children.push({
        key: element.id,
        value: element.id,
        title: element.name,
        pkey: element.pid,
        children: getTreeChildren(element.id, map),
      });
    });
  }
  return children;
};

export const getFileTree = async () => {
  const workspaceStore = useWorkspaceStore();
  const curWorkspace = workspaceStore.getWorkspace();
  if (!curWorkspace) {
    return;
  }
  const files = await getAllWorkspaceFiles(FILE_TYPE_DIR);
  const map: Map<string, File[]> = new Map();
  const root: TreeItem = {
    key: curWorkspace.id,
    value: curWorkspace.id,
    title: `${curWorkspace.name} (workspace)`,
  };
  files.forEach((file: File) => {
    let values = map.get(file.pid);
    if (!values) {
      values = [];
    }
    values.push(file);
    map.set(file.pid, values);
  });
  root.children = getTreeChildren(root.key, map);
  return root;
};
