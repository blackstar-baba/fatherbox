<script lang="ts" setup>
import type { VxeGridProps } from '#/adapter/vxe-table';

import { h, ref } from 'vue';

import { useVbenModal } from '@vben/common-ui';

import {
  DeleteOutlined,
  EditOutlined,
  PlusOutlined,
} from '@ant-design/icons-vue';
import { Button, message, Tag } from 'ant-design-vue';

import { useVbenForm, type VbenFormProps } from '#/adapter/form';
import { useVbenVxeGrid } from '#/adapter/vxe-table';
import {
  createFile as createFileApi,
  deleteFile as deleteFileById,
  type File,
  getWorkspaceFilesByPage,
  updateFileName,
} from '#/api';
import LocalUpload from '#/components/file/LocalUpload.vue';

import { encodeStringToUint8Array, FILE_TYPE_FILE } from './file';

interface RowType {
  category: string;
  color: string;
  id: string;
  price: string;
  productName: string;
  releaseDate: string;
}

const pidRef = ref<String>('');
const deleteFileRef = ref<File>();

const formOptions: VbenFormProps = {
  // 默认展开
  collapsed: false,
  schema: [
    {
      component: 'Input',
      componentProps: {
        placeholder: 'Please enter name',
      },
      defaultValue: '',
      fieldName: 'name',
      label: 'Name',
    },
  ],
  // 控制表单是否显示折叠按钮
  showCollapseButton: false,
  // 按下回车时是否提交表单
  submitOnEnter: false,
};

const gridOptions: VxeGridProps<RowType> = {
  checkboxConfig: {
    highlight: true,
    labelField: 'name',
  },
  columns: [
    { title: 'Index', type: 'seq', width: 80 },
    { field: 'name', title: 'Name' },
    { field: 'size', title: 'Size' },
    {
      field: 'createTime',
      formatter: 'formatUnixTimeToDateTime',
      title: 'Create Time',
    },
    {
      field: 'updateTime',
      formatter: 'formatUnixTimeToDateTime',
      title: 'Update Time',
    },
    {
      field: 'action',
      slots: { default: 'action' },
      title: 'Action',
      width: 130,
    },
  ],
  keepSource: true,
  pagerConfig: {},
  // toolbarConfig: {
  //   order: 'left',
  // },
  proxyConfig: {
    autoLoad: false,
    ajax: {
      query: async ({ page }, formValues) => {
        const fileId = pidRef.value;
        if (fileId) {
          return await getWorkspaceFilesByPage({
            pageSize: page.pageSize,
            pageNum: page.currentPage - 1,
            pid: fileId,
            type: FILE_TYPE_FILE,
            ...formValues,
          });
        }
        return new Promise((resolve) => {
          resolve({
            total: 0,
            item: [],
          });
        });
      },
    },
  },
};

const [Grid, gridApi] = useVbenVxeGrid({ formOptions, gridOptions });

const updateTable = (pid: string) => {
  pidRef.value = pid;
  gridApi.query({ pid });
};

function onSubmit(values: Record<string, any>) {
  if (values.id) {
    updateFileName({
      id: values.id,
      name: values.name,
    }).then((_: any) => {
      message.success('update file success');
      updateTable(pidRef.value.toString());
    });
  } else {
    createFileApi({
      name: values.name,
      pid: pidRef.value.toString(),
      content: values.content
        ? encodeStringToUint8Array(values.content)
        : undefined,
      path: values.file ?? undefined,
      type: FILE_TYPE_FILE,
    }).then((file: any) => {
      if (file.id) {
        message.success('create file success');
        updateTable(pidRef.value.toString());
      }
    });
  }
}

const [createForm, createFormApi] = useVbenForm({
  handleSubmit: onSubmit,
  schema: [
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
    },
    {
      component: 'Textarea',
      componentProps: {
        placeholder: '请输入内容',
      },
      fieldName: 'content',
      label: 'Content',
      dependencies: {
        if(values) {
          return values.inputOrUpload === 'input';
        },
        triggerFields: ['inputOrUpload'],
      },
    },
    {
      fieldName: 'file',
      label: 'file',
      component: h(LocalUpload),
      dependencies: {
        if(values) {
          return values.inputOrUpload === 'upload';
        },
        triggerFields: ['inputOrUpload'],
      },
    },
  ],
  showDefaultActions: false,
});

const [updateForm, updateFormApi] = useVbenForm({
  handleSubmit: onSubmit,
  schema: [
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
  ],
  showDefaultActions: false,
});

const [CreateModal, createModalApi] = useVbenModal({
  onCancel() {
    createModalApi.close();
  },
  onConfirm: async () => {
    await createFormApi.validateAndSubmitForm();
    createModalApi.close();
  },
});

const [UpdateModal, updateModalApi] = useVbenModal({
  onCancel() {
    updateModalApi.close();
  },
  onConfirm: async () => {
    await updateFormApi.validateAndSubmitForm();
    updateModalApi.close();
  },
});

const [DeleteModal, deleteModalApi] = useVbenModal({
  onCancel() {
    deleteModalApi.close();
  },
  onConfirm: async () => {
    const id = deleteFileRef.value?.id;
    if (id) {
      deleteFileById(id).then(() => {
        deleteFileRef.value = undefined;
        updateTable(pidRef.value.toString());
        message.success('delete file success');
        deleteModalApi.close();
      });
    }
  },
});

const createFile = (_: any) => {
  createModalApi.open();
};

const editFile = (row: any) => {
  updateFormApi.setValues({
    id: row.id,
    name: row.name,
  });
  updateModalApi.open();
};

const deleteFile = (row: any) => {
  deleteFileRef.value = row as File;
  deleteModalApi.open();
};

defineExpose({ updateTable });
</script>
<template>
  <Grid>
    <template #toolbar-tools>
      <Button size="small" type="primary" @click="createFile">
        <template #icon>
          <PlusOutlined />
        </template>
      </Button>
    </template>
    <template #action="{ row }">
      <!-- modal -->
      <Button size="small" type="primary" @click="editFile(row)">
        <template #icon>
          <EditOutlined />
        </template>
      </Button>
      <Button class="ml-2" size="small" type="primary" @click="deleteFile(row)">
        <template #icon>
          <DeleteOutlined />
        </template>
      </Button>
    </template>
  </Grid>
  <CreateModal>
    <createForm />
  </CreateModal>
  <UpdateModal>
    <updateForm />
  </UpdateModal>
  <DeleteModal>
    <div class="m-2">
      Are you sure to delete file:
      <Tag class="mb-2 ml-2" color="#87d068">
        {{ deleteFileRef?.name }}
      </Tag>
    </div>
  </DeleteModal>
</template>
