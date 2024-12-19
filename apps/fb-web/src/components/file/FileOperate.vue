<script setup lang="ts">
import { ref, watch } from 'vue';

import { useVbenModal } from '@vben/common-ui';

import {
  ExportOutlined,
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

interface TreeItem {
  key: string;
  value: string;
  title: string;
  pkey?: string;
  children?: TreeItem[];
  disabled?: boolean;
}

interface Props {
  content: string;
}

defineOptions({ name: 'Menu' });

const props = withDefaults(defineProps<Props>(), {});

const emits = defineEmits<{
  getBody: [fileBody: string];
  sendContent: [content: string];
}>();
const workspaceStore = useWorkspaceStore();
const fileTree = ref<TreeItem[]>([]);
const fileIdRef = ref<String>('');
const fileNameRef = ref<String>('');
const fileInput = ref<HTMLInputElement>();

const getChildren = (key: String, map: Map<String, File[]>) => {
  const children: TreeItem[] = [];
  const value = map.get(key);
  if (value) {
    value.forEach((element: any) => {
      children.push({
        key: element.id,
        value: element.id,
        title: element.name,
        pkey: element.pid,
        children: getChildren(element.id, map),
      });
    });
  }
  return children;
};

const updateFileTree = () => {
  getAllWorkspaceFiles().then((files: File[]) => {
    fileTree.value = [];
    const curWorkspace = workspaceStore.getWorkspace();
    if (!curWorkspace) {
      return;
    }
    const map: Map<String, File[]> = new Map();
    const root: TreeItem = {
      key: curWorkspace.id,
      title: curWorkspace.name,
      value: curWorkspace.id,
    };
    files.forEach((file: File) => {
      let values = map.get(file.pid);
      if (!values) {
        values = [];
      }
      values.push(file);
      map.set(file.pid, values);
    });
    root.children = getChildren(root.key, map);
    fileTree.value.push(root);
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
    type: 'file',
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
  if (node.children.length > 0) {
    message.warn('dir can not open');
    return;
  }
  fileIdRef.value = node.dataRef.key;
  fileNameRef.value = node.dataRef.title;
  const content = (await getFileContent(node.dataRef.key)) as string;
  emits('sendContent', content);
  modalApi.close();
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
  () => fileTree.value,
  (treeData) => {
    createFormApi.updateSchema([
      {
        componentProps: {
          treeData,
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
</script>
<template>
  <Card :bordered="false" class="mb-2 w-20">
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
