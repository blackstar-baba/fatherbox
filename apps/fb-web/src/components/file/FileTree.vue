<script lang="ts" setup>
import { ref, unref, watchEffect } from 'vue';

import { useVbenModal } from '@vben/common-ui';

import {
  DownOutlined,
  ExpandAltOutlined,
  ExportOutlined,
  FileOutlined,
  FolderOpenOutlined,
  FolderOutlined,
  ImportOutlined,
  PlusOutlined,
  RedoOutlined,
  SaveOutlined,
  ShrinkOutlined,
} from '@ant-design/icons-vue';
import { save as tauriSave } from '@tauri-apps/api/dialog';
import { writeTextFile } from '@tauri-apps/api/fs';
import {
  Button,
  Dropdown,
  Flex,
  Menu,
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
  getFileContent,
  updateFileContent,
  updateFileName,
} from '#/api';
import { useWorkspaceStore } from '#/store';
import { downloadByData } from '#/utils/file/downloadUtil';

import {
  CREATE_FORM_SCHEMA,
  DIR_MENU,
  FILE_MENU,
  FILE_TYPE_DIR,
  FILE_TYPE_FILE,
  type FileContent,
  ROOT_MENU,
  UPDATE_FORM_SCHEMA,
} from './file';

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
  activeFileId: string;
  content: string;
  editFiles: FileContent[];
}

const props = withDefaults(defineProps<Props>(), {});

const emits = defineEmits<{
  delete: [id: string];
  importContent: [content: string];
  open: [content: FileContent];
  select: [file: File];
  update: [
    file: {
      content: string;
      id: string;
    },
  ];
}>();

const workspaceStore = useWorkspaceStore();
const filesRef = ref<File[]>([]);
const fileTreeRef = ref<TreeItem[]>([]);
const fileIdRef = ref<String>('');
const selectedKeysRef = ref<any[]>([]);
const expendedKeysRef = ref<any[]>([]);
const deleteFileRef = ref<File>();
const menuItemsRef = ref<any[]>([]);
const fileInput = ref<HTMLInputElement>();

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
  expendAllKeyInner(fileTreeRef.value);
};

const getFileByKey = (key: string) => {
  if (filesRef.value) {
    const result = unref(filesRef).filter((item: any) => item.id === key);
    if (result.length > 0) {
      return result[0];
    }
  }
  return undefined;
};

const onClick = async (_: any, node: any) => {
  const key = node.dataRef.key;
  const type = node.dataRef.type;
  const title = node.dataRef.title;
  if (type === FILE_TYPE_FILE) {
    selectedKeysRef.value = [];
    selectedKeysRef.value.push(key);
    fileIdRef.value = key;
    getFileContent(key).then((content: any) => {
      emits('open', {
        id: key,
        name: title,
        // todo may be need byte[]
        content: content.toString(),
      });
      message.success(`open file ${title} success`);
    });
  } else {
    expendedKeysRef.value = node.expanded
      ? expendedKeysRef.value.filter((k) => k !== key)
      : [...expendedKeysRef.value, key];
  }
  emits('select', node.dataRef);
};

const onExpand = (keys: any) => {
  expendedKeysRef.value = keys;
};

const onRightClick = (obj: any) => {
  const nodeType = obj.node.type;
  switch (nodeType) {
    case '': {
      menuItemsRef.value = ROOT_MENU;
      break;
    }
    case FILE_TYPE_DIR: {
      menuItemsRef.value = DIR_MENU;
      break;
    }
    case FILE_TYPE_FILE: {
      menuItemsRef.value = FILE_MENU;
      break;
    }
  }
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
        type: element.type,
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
  getAllWorkspaceFiles().then((files: File[]) => {
    filesRef.value = files;
    fileTreeRef.value = [];
    const map: Map<String, File[]> = new Map();
    const root: TreeItem = {
      key: curWorkspace.id,
      value: curWorkspace.id,
      title: `${curWorkspace.name} (workspace)`,
      type: '',
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
    fileTreeRef.value.push(root);
    fileIdRef.value = root.key;
    // selectedKeysRef.value.push(root.key);
    // expend all keys
    expendAllKeys();
    // emits('selectedId', fileIdRef.value.toString());
  });
};

function onSubmit(values: Record<string, any>) {
  createFile({
    name: values.name,
    pid: values.pid,
    type: values.type,
    ...values,
  }).then((file: any) => {
    if (file.id) {
      fileIdRef.value = file.id;
      message.success(`create ${values.type} success`);
      updateFileTree();
    }
  });
}

function onUpdateSubmit(values: Record<string, any>) {
  updateFileName({
    id: values.id,
    name: values.name,
  }).then((_: any) => {
    emits('update', {
      id: values.id,
      name: values.name,
    });
    message.success('update success');
    updateFileTree();
  });
}

const [Form, formApi] = useVbenForm({
  handleSubmit: onSubmit,
  schema: CREATE_FORM_SCHEMA,
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

const [EditForm, editFormApi] = useVbenForm({
  handleSubmit: onUpdateSubmit,
  schema: UPDATE_FORM_SCHEMA,
  showDefaultActions: false,
});

const [EditModal, editModalApi] = useVbenModal({
  onCancel() {
    editModalApi.close();
  },
  onConfirm: async () => {
    await editFormApi.validateAndSubmitForm();
    editModalApi.close();
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
        emits('delete', id);
        deleteFileRef.value = undefined;
        updateFileTree();
        message.success('delete dir success');
        deleteModalApi.close();
      });
    }
  },
});

const create = () => {
  formApi.updateSchema([
    {
      fieldName: 'pid',
      componentProps: {
        treeData: fileTreeRef.value,
        disabled: false,
      },
    },
  ]);
  if (fileTreeRef.value && fileTreeRef.value.length > 0) {
    formApi.setValues({
      pid: fileTreeRef.value[0]?.key,
    });
  }
  modalApi.open();
};

const save = () => {
  props.editFiles.forEach((fileContent) => {
    updateFileContent({
      id: fileContent.id,
      content: fileContent.content,
    }).then(() => {
      message.success('save file success');
    });
  });
};

function handleFileChange(event: any) {
  const file = event.target.files[0];
  if (file) {
    const fileReader = new FileReader();
    fileReader.addEventListener('load', () => {
      emits('importContent', fileReader.result as string);
    });
    // eslint-disable-next-line unicorn/prefer-blob-reading-methods
    fileReader.readAsText(file);
    if (fileInput.value) {
      fileInput.value.value = '';
    }
  }
}

const importContent = async () => {
  if (!props.activeFileId) {
    message.warn('can not find can import content file');
    return;
  }
  if (fileInput.value) {
    fileInput.value.click();
  }
};

const exportContent = async () => {
  if (!props.activeFileId) {
    message.warn('can not find can import content file');
    return;
  }
  const existedFileContent = props.editFiles.filter(
    (k) => k.id === props.activeFileId,
  );
  if (!existedFileContent || existedFileContent.length === 0) {
    message.warn('can not find can import content file');
    return;
  }
  const fileContent = existedFileContent[0];
  if (fileContent) {
    if (window.__TAURI__) {
      const filePath = await tauriSave({ defaultPath: fileContent.name });
      if (filePath) {
        await writeTextFile(filePath, fileContent.content);
      }
    } else {
      downloadByData(props.content, fileContent.name);
    }
  }
};

const onContextMenuClick = (key: string, menuKey: number | string) => {
  const menu = menuKey.toString();
  switch (menu) {
    case 'create': {
      formApi.updateSchema([
        {
          fieldName: 'pid',
          componentProps: {
            treeData: fileTreeRef.value,
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
      const file = getFileByKey(key);
      if (file) {
        editFormApi.setValues({
          id: file?.id,
          pid: file?.pid,
          name: file?.name,
        });
        editFormApi.updateSchema([
          {
            fieldName: 'pid',
            componentProps: {
              treeData: fileTreeRef.value,
              disabled: true,
            },
          },
        ]);
        editModalApi.open();
      }
      break;
    }
    case 'open': {
      const file = getFileByKey(key);
      if (file) {
        getFileContent(key).then((content: any) => {
          emits('open', {
            id: file.id,
            name: file.name,
            content: content.toString(),
          });
          message.success(`open file ${file.name} success`);
        });
      } else {
        message.error('can not find file');
      }
      break;
    }
    case 'save': {
      const existedFileContent = props.editFiles.filter((k) => k.id === key);
      if (existedFileContent.length > 0 && existedFileContent[0]) {
        updateFileContent({
          id: existedFileContent[0].id,
          content: existedFileContent[0].content,
        }).then(() => {
          message.success('save file success');
        });
      } else {
        message.error('can not find file');
      }
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
      <Button class="ml-2" size="small" type="primary" @click="create">
        <template #icon>
          <PlusOutlined />
        </template>
      </Button>
      <Button class="ml-2" size="small" type="primary" @click="save">
        <template #icon>
          <SaveOutlined />
        </template>
      </Button>
      <Button class="ml-2" size="small" type="primary" @click="importContent">
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
      <Button class="ml-2" size="small" type="primary" @click="exportContent">
        <template #icon>
          <ExportOutlined />
        </template>
      </Button>
    </div>
    <Tree
      :expanded-keys="expendedKeysRef"
      :selected-keys="selectedKeysRef"
      :tree-data="fileTreeRef"
      show-icon
      @click="onClick"
      @expand="onExpand"
      @right-click="onRightClick"
    >
      <template #title="{ title, key }">
        <Dropdown :trigger="['contextmenu']">
          <span>{{ title }}</span>
          <template #overlay>
            <Menu
              :items="menuItemsRef"
              @click="({ key: menuKey }) => onContextMenuClick(key, menuKey)"
            />
          </template>
        </Dropdown>
      </template>
      <template #switcherIcon="{ switcherCls }">
        <DownOutlined :class="switcherCls" />
      </template>
      <template #icon="{ dataRef, expanded }">
        <template v-if="dataRef.type === FILE_TYPE_DIR || dataRef.type === ''">
          <FolderOpenOutlined v-if="expanded" />
          <FolderOutlined v-else />
        </template>
        <template v-else>
          <FileOutlined />
        </template>
      </template>
    </Tree>
  </Flex>
  <Modal title="Create">
    <Form />
  </Modal>
  <EditModal title="Edit">
    <EditForm />
  </EditModal>
  <DeleteModal title="Remove File">
    <div class="m-2">
      Are you sure to delete dir:
      <Tag class="mb-2 ml-2" color="#87d068">
        {{ deleteFileRef?.name }}
      </Tag>
    </div>
  </DeleteModal>
</template>

<!--// todo file import-->
<!--// todo file export-->
<!--// todo file copy-->
