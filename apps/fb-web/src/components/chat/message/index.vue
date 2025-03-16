<script setup lang="ts">
import { ref } from 'vue';

import { useVbenModal } from '@vben/common-ui';
import { $t } from '@vben/locales';

import {
  CheckOutlined,
  CloseOutlined,
  CopyOutlined,
  EditOutlined,
  RedoOutlined,
  SaveOutlined,
} from '@ant-design/icons-vue';
import { Button, message, Textarea } from 'ant-design-vue';

import { useVbenForm } from '#/adapter';
import { createFile as createFileApi } from '#/api';
import {
  encodeStringToUint8Array,
  FILE_TYPE_FILE,
  getFileTree,
} from '#/components/file/file';
import { copy } from '#/utils';

import MessageAvatar from './message-avatar.vue';
import MessageText from './message-text.vue';

interface Props {
  text: string;
  inversion?: boolean;
  error?: boolean;
  loading?: boolean;
}

interface Emit {
  (e: 'regenerate'): void;
  (e: 'edit', prompt: string): void;
}

const props = defineProps<Props>();

const emit = defineEmits<Emit>();

const asRawText = ref(props.inversion);

const messageRef = ref<HTMLElement>();

const isEditRef = ref(false);

const editTextRef = ref('');

function handleRegenerate() {
  messageRef.value?.scrollIntoView();
  emit('regenerate');
}

function handleEditSubmit() {
  isEditRef.value = false;
  messageRef.value?.scrollIntoView();
  emit('edit', editTextRef.value);
}

function handleEditCancel() {
  isEditRef.value = false;
  editTextRef.value = '';
}

function handleEdit() {
  isEditRef.value = true;
  editTextRef.value = props.text;
}

function handleCopy() {
  try {
    copy(props.text);
  } catch {
    message.error($t('chat.message.copyFailed'));
  }
}

function onSubmit(values: Record<string, any>) {
  createFileApi({
    name: values.name,
    pid: values.pid,
    content: values.content
      ? encodeStringToUint8Array(values.content)
      : undefined,
    path: values.file ?? undefined,
    type: FILE_TYPE_FILE,
  }).then((file: any) => {
    if (file.id) {
      message.success('save file success');
    }
  });
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
    {
      component: 'Textarea',
      componentProps: {
        placeholder: '请输入内容',
      },
      fieldName: 'content',
      label: 'Content',
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

async function handleSave() {
  const fileTree = await getFileTree();
  // todo can throw error
  if (fileTree) {
    createFormApi.updateSchema([
      {
        fieldName: 'pid',
        componentProps: {
          treeData: [fileTree],
          disabled: false,
        },
      },
    ]);
    createFormApi.setValues({
      content: props.text,
      pid: fileTree?.value,
    });
    createModalApi.open();
  }
}
</script>

<template>
  <div
    ref="messageRef"
    :class="[{ 'flex-row-reverse': inversion }]"
    class="mb-6 flex w-full overflow-hidden"
  >
    <div
      :class="[inversion ? 'ml-2' : 'mr-2']"
      class="flex h-8 flex-shrink-0 basis-8 items-center justify-center overflow-hidden rounded-full"
    >
      <MessageAvatar :image="inversion" />
    </div>
    <div
      :class="[inversion ? 'items-end' : 'items-start']"
      class="overflow-hidden"
    >
      <div
        :class="[inversion ? 'flex-row-reverse' : 'flex-row']"
        class="flex items-end pb-3"
      >
        <MessageText
          v-if="!isEditRef"
          :as-raw-text="asRawText"
          :error="error"
          :inversion="inversion"
          :loading="loading"
          :text="text"
        />
        <Textarea
          v-if="isEditRef"
          v-model:value="editTextRef"
          :auto-size="{ minRows: 1, maxRows: 8 }"
          :placeholder="$t('chat.message.placeholder')"
          class="min-w-[250px]"
        />
      </div>
      <p :class="[inversion ? 'text-right' : 'text-left']">
        <Button v-if="!isEditRef" size="small" type="text" @click="handleCopy">
          <CopyOutlined />
        </Button>
        <Button
          v-if="!inversion && !loading"
          size="small"
          type="text"
          @click="handleSave"
        >
          <SaveOutlined />
        </Button>
        <Button
          v-if="!inversion && !loading"
          size="small"
          type="text"
          @click="handleRegenerate"
        >
          <RedoOutlined />
        </Button>
        <Button
          v-if="inversion && !isEditRef"
          size="small"
          type="text"
          @click="handleEdit"
        >
          <EditOutlined />
        </Button>
        <Button
          v-if="inversion && isEditRef"
          size="small"
          type="text"
          @click="handleEditSubmit"
        >
          <CheckOutlined />
        </Button>
        <Button
          v-if="inversion && isEditRef"
          size="small"
          type="text"
          @click="handleEditCancel"
        >
          <CloseOutlined />
        </Button>
      </p>
    </div>
  </div>
  <CreateModal>
    <createForm />
  </CreateModal>
</template>
