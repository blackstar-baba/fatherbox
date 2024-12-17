<script lang="ts" setup>
import { ref, unref, watchEffect } from 'vue';

import { useVbenModal } from '@vben/common-ui';

import {
  DeleteOutlined,
  EditOutlined,
  ExpandAltOutlined,
  PlusOutlined,
  RedoOutlined,
  ShrinkOutlined,
} from '@ant-design/icons-vue';
import {
  Button,
  Dropdown,
  Flex,
  Menu,
  MenuItem,
  message,
  Tag,
  Tree,
} from 'ant-design-vue';

import { useVbenForm } from '#/adapter';
import {
  createFile,
  deleteFile,
  type File,
  getAllWorkspaceFiles,
  updateFileName,
} from '#/api';
import { useWorkspaceStore } from '#/store';

interface TreeItem {
  key: string;
  value: string;
  title: string;
  pkey?: string;
  children?: TreeItem[];
  disabled?: boolean;
}

defineOptions({ name: 'Menu' });

const emits = defineEmits<{
  selectedId: [fileId: string];
}>();

const workspaceStore = useWorkspaceStore();
const filesRef = ref<File[]>([]);
const fileTree = ref<TreeItem[]>([]);
const fileIdRef = ref<String>('');
const selectedKeysRef = ref<any[]>([]);
const expendedKeysRef = ref<any[]>([]);
const deleteFileRef = ref<File>();

const expendAllKeyInner = (items: TreeItem[]) => {
  items.forEach((item: TreeItem) => {
    expendedKeysRef.value.push(item.key);
    if (item.children) {
      expendAllKeyInner(item.children);
    }
  });
};

const expendAllKeys = () => {
  expendedKeysRef.value = [];
  expendAllKeyInner(fileTree.value);
};

const selectTreeItem = async (_: any, node: any) => {
  fileIdRef.value = node.dataRef.key;
  selectedKeysRef.value = [];
  selectedKeysRef.value.push(node.dataRef.key);
  emits('selectedId', fileIdRef.value.toString());
};

const onExpand = (keys: any) => {
  expendedKeysRef.value = keys;
};

const shrinkAllKeys = () => {
  expendedKeysRef.value = [];
};

const getTreeChildren = (key: String, map: Map<String, File[]>) => {
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

const updateFileTree = () => {
  const curWorkspace = workspaceStore.getWorkspace();
  if (!curWorkspace) {
    return;
  }
  getAllWorkspaceFiles('dir').then((files: File[]) => {
    filesRef.value = files;
    fileTree.value = [];
    const map: Map<String, File[]> = new Map();
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
    fileTree.value.push(root);
    fileIdRef.value = root.key;
    selectedKeysRef.value.push(root.key);
    // expend all keys
    expendAllKeys();
    emits('selectedId', fileIdRef.value.toString());
  });
};

function onSubmit(values: Record<string, any>) {
  if (values.id) {
    updateFileName({
      id: values.id,
      name: values.name,
    }).then((_: any) => {
      message.success('update dir success');
      updateFileTree();
    });
  } else {
    createFile({
      name: values.name,
      pid: values.pid,
      type: 'dir',
    }).then((file: any) => {
      if (file.id) {
        fileIdRef.value = file.id;
        message.success('create dir success');
        updateFileTree();
      }
    });
  }
}

const [Form, formApi] = useVbenForm({
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
  ],
  showDefaultActions: false,
});

const [Modal, modalApi] = useVbenModal({
  onCancel() {
    modalApi.close();
  },
  onConfirm: async () => {
    await formApi.validateAndSubmitForm();
    modalApi.close();
  },
});

const [DeleteModal, deleteModalApi] = useVbenModal({
  onCancel() {
    deleteModalApi.close();
  },
  onConfirm: async () => {
    const id = deleteFileRef.value?.id;
    if (id) {
      deleteFile(id).then(() => {
        deleteFileRef.value = undefined;
        updateFileTree();
        message.success('delete dir success');
        deleteModalApi.close();
      });
    }
  },
});

const getFileByKey = (key: string) => {
  if (filesRef.value) {
    const result = unref(filesRef).filter((item: any) => item.id === key);
    if (result.length > 0) {
      return result[0];
    }
  }
  return undefined;
};

const onContextMenuClick = (key: string, menuKey: number | string) => {
  message.success(
    JSON.stringify({
      key,
      menuKey,
    }),
  );
  const menu = menuKey.toString();
  switch (menu) {
    case 'create': {
      modalApi.setState({ title: 'Create Dir' });
      formApi.updateSchema([
        {
          fieldName: 'pid',
          componentProps: {
            treeData: fileTree.value,
            disabled: false,
          },
        },
      ]);
      formApi.setValues({
        pid: key,
      });
      modalApi.open();

      break;
    }
    case 'delete': {
      const file = getFileByKey(key);
      if (file) {
        deleteFileRef.value = file;
        deleteModalApi.open();
      }
      break;
    }
    case 'edit': {
      modalApi.setState({ title: 'Edit Dir' });
      const file = getFileByKey(key);
      if (file) {
        formApi.setValues({
          id: file.id,
          pid: file.pid,
          name: file.name,
        });
      }
      formApi.updateSchema([
        {
          fieldName: 'pid',
          componentProps: {
            treeData: fileTree.value,
            disabled: true,
          },
        },
      ]);
      modalApi.open();
      break;
    }
  }
};

watchEffect(() => {
  updateFileTree();
});
</script>
<template>
  <Flex vertical>
    <div class="mb-2">
      <Button size="small" type="primary" @click="updateFileTree">
        <template #icon>
          <RedoOutlined />
        </template>
      </Button>
      <Button class="ml-2" size="small" type="primary" @click="expendAllKeys">
        <template #icon>
          <ExpandAltOutlined />
        </template>
      </Button>
      <Button class="ml-2" size="small" type="primary" @click="shrinkAllKeys">
        <template #icon>
          <ShrinkOutlined />
        </template>
      </Button>
    </div>
    <Tree
      :expanded-keys="expendedKeysRef"
      :selected-keys="selectedKeysRef"
      :tree-data="fileTree"
      @click="selectTreeItem"
      @expand="onExpand"
    >
      <template #title="{ title, key }">
        <Dropdown :trigger="['contextmenu']">
          <span>{{ title }}</span>
          <template #overlay>
            <Menu
              @click="({ key: menuKey }) => onContextMenuClick(key, menuKey)"
            >
              <MenuItem key="create">
                <Button size="small" type="primary">
                  <template #icon>
                    <PlusOutlined />
                  </template>
                </Button>
              </MenuItem>
              <MenuItem v-if="key !== workspaceStore.getId()" key="edit">
                <Button size="small" type="primary">
                  <template #icon>
                    <EditOutlined />
                  </template>
                </Button>
              </MenuItem>
              <MenuItem v-if="key !== workspaceStore.getId()" key="delete">
                <Button size="small" type="primary">
                  <template #icon>
                    <DeleteOutlined />
                  </template>
                </Button>
              </MenuItem>
            </Menu>
          </template>
        </Dropdown>
      </template>
    </Tree>
  </Flex>
  <Modal>
    <Form />
  </Modal>
  <DeleteModal title="Remove File">
    <div class="m-2">
      Are you sure to delete dir:
      <Tag class="mb-2 ml-2" color="#87d068">
        {{ deleteFileRef?.name }}
      </Tag>
    </div>
  </DeleteModal>
</template>
