<script setup lang="ts">
import { ref, watch } from 'vue';

import { useVbenModal } from '@vben/common-ui';

// todo add new method
import {
  CopyOutlined,
  ExportOutlined,
  FileAddOutlined,
  FolderOpenOutlined,
  ImportOutlined,
  SaveOutlined,
} from '@ant-design/icons-vue';
import { save } from '@tauri-apps/api/dialog';
import { writeTextFile } from '@tauri-apps/api/fs';
import {
  Badge,
  Button,
  Card,
  Flex,
  message,
  Row,
  Tag,
  Tooltip,
  Tree,
} from 'ant-design-vue';

import { useVbenForm } from '#/adapter/form';
import {
  createFile,
  type File,
  getAllWorkspaceFiles,
  getFileContent,
  updateFileContent,
} from '#/api/core/file';
import { useWorkspaceStore } from '#/store';
import { downloadByData } from '#/utils/file/downloadUtil';

import { FILE_TYPE_DIR, FILE_TYPE_FILE } from './file';

interface TreeItem {
  key: string;
  value: string;
  title: string;
  pkey?: string;
  children?: TreeItem[];
  disabled?: boolean;
  type: string;
}

interface Props {
  content: string;
}

const props = withDefaults(defineProps<Props>(), {});

const emits = defineEmits<{
  getBody: [fileBody: string];
  sendContent: [content: string];
}>();
const workspaceStore = useWorkspaceStore();
const filesRef = ref<File[]>([]);
const fileTree = ref<TreeItem[]>([]);
const fileIdRef = ref<String>('');
const fileNameRef = ref<String>('');
const fileInput = ref<HTMLInputElement>();

const getFileChildren = (key: String, map: Map<String, File[]>) => {
  const children: TreeItem[] = [];
  const value = map.get(key);
  if (value) {
    value.forEach((element: any) => {
      if (element.type === FILE_TYPE_DIR) {
        const items = map.get(element.id);
        if (items && items.length > 0) {
          children.push({
            key: element.id,
            value: element.id,
            title: element.name,
            pkey: element.pid,
            children: getFileChildren(element.id, map),
            disabled: true,
            type: element.type,
          });
        }
      } else if (element.type === FILE_TYPE_FILE) {
        children.push({
          key: element.id,
          value: element.id,
          title: element.name,
          pkey: element.pid,
          type: element.type,
        });
      }
    });
  }
  return children;
};

const getDirChildren = (key: String, map: Map<String, File[]>) => {
  const children: TreeItem[] = [];
  const value = map.get(key);
  if (value) {
    value.forEach((element: any) => {
      if (element.type === FILE_TYPE_DIR) {
        children.push({
          key: element.id,
          value: element.id,
          title: element.name,
          pkey: element.pid,
          children: getDirChildren(element.id, map),
          type: element.type,
        });
      }
    });
  }
  return children;
};

const updateFileTree = () => {
  getAllWorkspaceFiles().then((files: File[]) => {
    filesRef.value = files;
  });
};

const [Modal, modalApi] = useVbenModal({
  onCancel() {
    modalApi.close();
  },
  onConfirm() {
    modalApi.close();
  },
});

function onSubmit(values: Record<string, any>) {
  createFile({
    name: values.name,
    pid: values.pid,
    content: props.content,
    type: FILE_TYPE_FILE,
  }).then((file: any) => {
    if (file.id) {
      fileIdRef.value = file.id;
      message.success('create file success');
    }
  });
}

const [CreateForm, createFormApi] = useVbenForm({
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
      component: 'TreeSelect',
      componentProps: {
        allowClear: false,
        class: 'w-full',
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

const [CreateModal, createModalApi] = useVbenModal({
  fullscreenButton: false,
  onOpened() {
    updateFileTree();
  },
  onCancel() {
    createModalApi.close();
  },
  onConfirm: async () => {
    await createFormApi.validateAndSubmitForm();
    createModalApi.close();
  },
  onOpenChange(isOpen: boolean) {
    if (isOpen) {
      const { values } = createModalApi.getData<Record<string, any>>();
      if (values) {
        createFormApi.setValues(values);
      }
    }
  },
});

const openFile = (fileId: string) => {
  fileIdRef.value = fileId;
};

const selectTreeItem = async (_: any, node: any) => {
  if (!node.children) {
    fileIdRef.value = node.dataRef.key;
    fileNameRef.value = node.dataRef.title;
    const content = (await getFileContent(node.dataRef.key)) as string;
    emits('sendContent', content);
    modalApi.close();
  }
};

const addFile = () => {
  fileIdRef.value = '';
  fileNameRef.value = '';
  emits('sendContent', '');
};

const openFileModal = () => {
  modalApi.open();
  updateFileTree();
};

const saveFile = async (_: any) => {
  if (fileIdRef.value) {
    updateFileContent({
      id: fileIdRef.value.toString(),
      content: props.content,
    }).then(() => {
      message.success('save file success');
    });
  } else {
    createModalApi.open();
  }
};

watch(
  () => filesRef.value,
  (files) => {
    const map: Map<String, File[]> = new Map();
    files.forEach((file: File) => {
      let values = map.get(file.pid);
      if (!values) {
        values = [];
      }
      values.push(file);
      map.set(file.pid, values);
    });
    const curWorkspace = workspaceStore.getWorkspace();
    if (!curWorkspace) {
      message.error('update file tree failed, current workspace not found');
      return;
    }
    // update file tree
    fileTree.value = [];
    const root: TreeItem = {
      key: curWorkspace.id,
      title: curWorkspace.name,
      value: curWorkspace.id,
      type: FILE_TYPE_DIR,
      disabled: true,
    };
    root.children = getFileChildren(root.key, map);
    fileTree.value.push(root);
    // update dir tree
    const dirTreeData = [];
    const dirRoot: TreeItem = {
      key: curWorkspace.id,
      title: curWorkspace.name,
      value: curWorkspace.id,
      type: FILE_TYPE_DIR,
    };
    dirRoot.children = getDirChildren(dirRoot.key, map);
    dirTreeData.push(dirRoot);
    createFormApi.updateSchema([
      {
        componentProps: {
          treeData: dirTreeData,
        },
        fieldName: 'pid',
      },
    ]);
  },
);

defineExpose({
  openFile,
});

const importFile = async () => {
  if (fileInput.value) {
    fileInput.value.click();
  }
};

const exportFile = async () => {
  // 需要传递一个文件名
  let fileName = 'unknown';
  if (fileNameRef.value) {
    fileName = fileNameRef.value as string;
  }
  if (window.__TAURI__) {
    const filePath = await save({ defaultPath: fileName });
    if (filePath) {
      await writeTextFile(filePath, props.content);
    }
  } else {
    downloadByData(props.content, fileName);
  }
};

function handleFileChange(event: any) {
  const file = event.target.files[0];
  if (file) {
    const fileReader = new FileReader();
    fileReader.addEventListener('load', () => {
      emits('sendContent', fileReader.result as string);
    });
    // eslint-disable-next-line unicorn/prefer-blob-reading-methods
    fileReader.readAsText(file);
    fileNameRef.value = file.name;
    fileIdRef.value = '';
  }
}

function copy() {
  const input = document.createElement('textarea');
  input.style.position = 'fixed';
  input.style.opacity = String(0);
  input.value = props.content;
  document.body.append(input);
  input.select();
  document.execCommand('Copy');
  input.remove();
  message.success('copy success');
}
</script>
<template>
  <Card :bordered="false" class="mb-2 w-20">
    <Row>
      <Tooltip placement="top">
        <template #title> Copy content</template>
        <Button class="mb-2" type="primary" @click="copy">
          <template #icon>
            <CopyOutlined />
          </template>
        </Button>
      </Tooltip>
    </Row>
    <Row>
      <Button class="mb-2" type="primary" @click="addFile">
        <template #icon>
          <FileAddOutlined />
        </template>
      </Button>
    </Row>
    <Row>
      <Tooltip :overlay-inner-style="{ width: '370px' }" placement="right">
        <template v-if="fileIdRef !== ''" #title>
          <Flex vertical>
            <Tag class="mb-2 ml-2" color="#87d068">
              File Id:{{ fileIdRef }}
            </Tag>
            <Tag v-if="fileNameRef !== ''" class="ml-2" color="#87d068">
              File Name:{{ fileNameRef }}
            </Tag>
          </Flex>
        </template>
        <Badge :dot="fileIdRef !== ''" color="#87d068">
          <Button class="mb-2" type="primary" @click="openFileModal">
            <template #icon>
              <FolderOpenOutlined />
            </template>
          </Button>
        </Badge>
      </Tooltip>
    </Row>
    <Row>
      <Button class="mb-2" type="primary" @click="saveFile">
        <template #icon>
          <SaveOutlined />
        </template>
      </Button>
    </Row>
    <Row>
      <Button class="mb-2" type="primary" @click="importFile">
        <template #icon>
          <ImportOutlined />
        </template>
      </Button>
      <input
        ref="fileInput"
        style="display: none"
        type="file"
        @change="handleFileChange"
      />
    </Row>
    <Row>
      <Button class="mb-2" type="primary" @click="exportFile">
        <template #icon>
          <ExportOutlined />
        </template>
      </Button>
    </Row>
  </Card>
  <Modal title="Choose File">
    <Tree :on-click="selectTreeItem" :tree-data="fileTree">
      <template #title="{ title, key }">
        <span v-if="key === ''" style="color: #1890ff">{{ title }}</span>
        <template v-else>{{ title }}</template>
      </template>
    </Tree>
  </Modal>
  <CreateModal title="Create File">
    <CreateForm />
  </CreateModal>
</template>
