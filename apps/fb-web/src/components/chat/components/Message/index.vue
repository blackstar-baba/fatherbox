<script setup lang="ts">
import { ref } from 'vue';

import { $t } from '@vben/locales';

import {
  CheckOutlined,
  CloseOutlined,
  CopyOutlined,
  EditOutlined,
  RedoOutlined,
} from '@ant-design/icons-vue';
import { Button, message, Textarea } from 'ant-design-vue';

import { copy } from '#/utils';

import AvatarComponent from './Avatar.vue';
import TextComponent from './Text.vue';

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
      <AvatarComponent :image="inversion" />
    </div>
    <div
      :class="[inversion ? 'items-end' : 'items-start']"
      class="overflow-hidden"
    >
      <div
        :class="[inversion ? 'flex-row-reverse' : 'flex-row']"
        class="flex items-end pb-3"
      >
        <TextComponent
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
</template>
