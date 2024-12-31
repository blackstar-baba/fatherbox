import { h } from 'vue';

import {
  DeleteOutlined,
  EditOutlined,
  PlusOutlined,
} from '@ant-design/icons-vue';
import { Button } from 'ant-design-vue';

import LocalUpload from '#/components/file/LocalUpload.vue';

export const FILE_TYPE_DIR = 'dir';
export const FILE_TYPE_FILE = 'file';

export const CREATE_FORM_SCHEMA: any[] = [
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
    fieldName: 'file',
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

export const ROOT_MENU = [
  {
    key: 'create',
    icon: () => h(Button, { size: 'small', type: 'primary' }, h(PlusOutlined)),
  },
];

export const FILE_MENU = [
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
