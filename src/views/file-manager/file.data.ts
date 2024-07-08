import { BasicColumn, FormSchema } from '@/components/Table';
import { FileParams, SimpleFileEntry } from '@/api/sys/model/fileModel';
import { invoke } from '@tauri-apps/api/tauri';

export const workspace = 'default';

export const DIR_TYPE = 'dir';
export const FILE_TYPE = 'file';
export const columns: BasicColumn[] = [
  {
    title: 'name',
    dataIndex: 'name',
    width: 200,
    align: 'left',
  },
  // {
  //   title: '状态',
  //   dataIndex: 'status',
  //   width: 80,
  //   customRender: ({ record }) => {
  //     const status = record.status;
  //     const enable = ~~status === 0;
  //     const color = enable ? 'green' : 'red';
  //     const text = enable ? '启用' : '停用';
  //     return h(Tag, { color: color }, () => text);
  //   },
  // },
  {
    title: 'type',
    dataIndex: 'type',
    width: 180,
  },
  {
    title: 'size',
    dataIndex: 'size',
    width: 180,
  },
  {
    title: 'createTime',
    dataIndex: 'createTime',
    width: 180,
  },
  {
    title: 'modifyTime',
    dataIndex: 'modifyTime',
    width: 180,
  },
];

// const isDir = (type: string) => type === 'dir';
// const isFile = (type: string) => type === 'txt';

// const isMenu = (type: string) => type === '1';
// const isButton = (type: string) => type === '2';

export const searchFormSchema: FormSchema[] = [
  {
    field: 'name',
    label: 'name',
    component: 'Input',
    colProps: { span: 8 },
  },
];

export const dirFormSchema: FormSchema[] = [
  {
    field: 'name',
    label: 'name',
    component: 'Input',
    required: true,
  },
  {
    field: 'parentPath',
    label: 'parentPath',
    component: 'TreeSelect',
    defaultValue: '',
    componentProps: {
      fieldNames: {
        label: 'name',
        value: 'path',
      },
      getPopupContainer: () => document.body,
    },
  },
];

export const formSchema: FormSchema[] = [
  {
    field: 'type',
    label: 'type',
    component: 'RadioButtonGroup',
    defaultValue: 'dir',
    componentProps: {
      options: [
        { label: 'dir', value: 'dir' },
        { label: 'file', value: 'file' },
      ],
      onChange: (e: any) => {
        console.log(e);
      },
    },
    colProps: { lg: 24, md: 24 },
  },
  {
    field: 'name',
    label: 'name',
    component: 'Input',
    required: true,
  },

  {
    field: 'parentPath',
    label: 'parentPath',
    component: 'TreeSelect',
    defaultValue: '',
    componentProps: {
      fieldNames: {
        label: 'name',
        value: 'path',
      },
      getPopupContainer: () => document.body,
    },
  },
  {
    field: 'inputOrUpload',
    label: 'inputOrUpload',
    component: 'RadioButtonGroup',
    defaultValue: 'input',
    componentProps: {
      options: [
        { label: 'input', value: 'input' },
        { label: 'upload', value: 'upload' },
      ],
      onChange: (e: any) => {
        console.log(e);
      },
    },
    // dynamicDisabled: ({ values }) => {
    //   return !!values.uploadOrEdit;
    // },
    ifShow: ({ values }) => values.type == 'file' && !values.path,
  },
  {
    field: 'content',
    label: 'content',
    component: 'InputTextArea',
    ifShow: ({ values }) =>
      values.type == 'file' && !values.path && values.inputOrUpload === 'input',
  },
  {
    field: 'file',
    label: 'file',
    component: 'LocalUpload',
    componentProps: {},
    colProps: { lg: 24, md: 24 },
    ifShow: ({ values }) =>
      values.type == 'file' && !values.path && values.inputOrUpload === 'upload',
  },
  {
    field: 'path',
    label: 'path',
    component: 'Input',
    dynamicDisabled: ({ values }) => {
      return values.path;
    },
    ifShow: ({ values }) => values.path,
  },

  // {
  //   field: 'show',
  //   label: '是否显示',
  //   component: 'RadioButtonGroup',
  //   defaultValue: '0',
  //   componentProps: {
  //     options: [
  //       { label: '是', value: '0' },
  //       { label: '否', value: '1' },
  //     ],
  //   },
  //   ifShow: ({ values }) => !isButton(values.type),
  // },
];

export const getFiles = (param: FileParams) => {
  if (window.__TAURI__) {
    return invoke('list_workspace_files', {
      request: {
        workspace: workspace,
        recursive: false,
        name: param.name,
        path: param.path ? param.path : '',
        type: 'file',
      },
    }).then((message: any) => {
      console.log(message);
      const fileEntrys: SimpleFileEntry[] = [];
      message.result.forEach((element: any) => {
        fileEntrys.push({
          name: element.name,
          size: element.size,
          type: element.type,
          createTime: element.createTime,
          modifyTime: element.modifyTime,
          path: element.path,
          parentPath: element.parentPath,
        });
      });
      // processFileEntrys(message.result, '');
      return fileEntrys;
    });
  }
  return Promise.resolve([]);
};

export const getDirs = () => {
  if (window.__TAURI__) {
    // how to return promise
    return invoke('list_workspace_dirs', { workspace: workspace }).then((message: any) => {
      console.log(message);
      // processFileEntrys(message.result, '');
      message.result.forEach((element: any) => {
        element.bkName = element.name;
      });
      return message.result;
    });
  }
  return Promise.resolve([]);
};

export const createFile = (
  workspace: string,
  fileName: string,
  fileType: string,
  parentPath: string,
) => {
  if (window.__TAURI__) {
    // how to return promise
    return invoke('create_workspace_file', {
      workspace: workspace,
      fileName: fileName,
      fileType: fileType,
      parentPath: parentPath,
    })
      .then((message: any) => {
        console.log(message);
      })
      .catch((error) => {
        console.error(error);
      });
  }
  return Promise.resolve([]);
};

export const deleteFile = (workspace: string, fileName: string, parentPath: string) => {
  if (window.__TAURI__) {
    // how to return promise
    return invoke('delete_workspace_file', {
      workspace: workspace,
      fileName: fileName,
      parentPath: parentPath,
    })
      .then((message: any) => {
        console.log(message);
      })
      .catch((error) => {
        console.error(error);
      });
  }
  return Promise.resolve([]);
};
