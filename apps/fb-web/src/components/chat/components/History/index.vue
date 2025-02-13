<script setup lang="ts">
import { createVNode, onMounted } from 'vue';

import { RiMessage3Line } from '@vben/icons';
import { $t } from '@vben/locales';

import {
  CheckOutlined,
  CloseOutlined,
  DeleteOutlined,
  EditOutlined,
  ExclamationCircleOutlined,
  PlusCircleOutlined,
} from '@ant-design/icons-vue';
import { Button, Input, Modal } from 'ant-design-vue';

import { type ChatInfo, deleteChat, updateChatName } from '#/api';

interface Props {
  loading: boolean;
  active: null | string;
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  active: null,
});

const emit = defineEmits<{ select: [null | string] }>();

const chatInfosModel = defineModel<ChatInfo[]>('chatInfos', {
  default: [],
});

function handleAdd() {
  setActive(null);
}

async function handleSelect({ id }: ChatInfo, event?: MouseEvent) {
  event?.stopPropagation();
  setActive(id);
}

function handleEdit(chatInfo: ChatInfo, event?: MouseEvent) {
  event?.stopPropagation();
  chatInfo.oldName = chatInfo.name;
  chatInfo.isEdit = true;
}

async function handleSave(chatInfo: ChatInfo, event?: MouseEvent) {
  event?.stopPropagation();
  // todo can get response
  await updateChatName(chatInfo);
  for (const item of chatInfosModel.value) {
    if (item.id === chatInfo.id) {
      item.name = chatInfo.name;
    }
  }
  chatInfo.oldName = undefined;
  chatInfo.isEdit = false;
}

function handleCancel(chatInfo: ChatInfo, event?: MouseEvent) {
  event?.stopPropagation();
  chatInfo.name = chatInfo.oldName ?? '';
  chatInfo.oldName = undefined;
  chatInfo.isEdit = false;
}

function handleDelete(chatInfo: ChatInfo, event?: MouseEvent | TouchEvent) {
  event?.stopPropagation();
  Modal.confirm({
    title: $t('chat.chat.clear'),
    icon: createVNode(ExclamationCircleOutlined),
    content: `${$t('chat.chat.clearHistoryConfirm')}[${chatInfo.name}]?`,
    okText: $t('common.confirm'),
    okType: 'danger',
    cancelText: $t('common.cancel'),
    async onOk() {
      const id = chatInfo.id;
      await deleteChat(chatInfo);
      const index = chatInfosModel.value.findIndex((item) => item.id === id);
      chatInfosModel.value.splice(index, 1);
      if (isActive(id)) {
        setActive(null);
      }
    },
    onCancel() {},
    class: 'test',
  });
}

async function handleEnter(chatInfo: ChatInfo, event: KeyboardEvent) {
  event?.stopPropagation();
  if (event.key === 'Enter') {
    chatInfo.oldName = undefined;
    await updateChatName(chatInfo);
    chatInfo.isEdit = false;
  }
}

function isActive(id: string) {
  return props.active === id;
}

function setActive(id: null | string) {
  emit('select', id);
}

onMounted(async () => {});
</script>

<template>
  <div class="p-4">
    <Button @click="handleAdd">
      <template #icon>
        <PlusCircleOutlined />
      </template>
      {{ $t('chat.chat.new') }}
    </Button>
  </div>
  <div class="p-4">
    <div class="gap-2 text-sm">
      <template v-if="chatInfosModel.length === 0">
        <div
          class="mt-4 flex flex-col items-center text-center text-neutral-300"
        >
          <span>{{ $t('chat.chat.noData') }}</span>
        </div>
      </template>
      <template v-else>
        <div v-for="(item, index) of chatInfosModel" :key="index" class="mt-2">
          <a
            :class="
              isActive(item.id) && [
                'border-[#4b9e5f]',
                'bg-neutral-100',
                'text-[#4b9e5f]',
                'dark:bg-[#24272e]',
                'dark:border-[#4b9e5f]',
                'pr-14',
              ]
            "
            class="group relative flex cursor-pointer items-center gap-3 break-all rounded-md px-3 py-1 hover:bg-neutral-100 dark:border-neutral-800 dark:hover:bg-[#24272e]"
            @click="handleSelect(item, $event)"
          >
            <span>
              <RiMessage3Line />
            </span>
            <div
              class="relative flex-1 overflow-hidden text-ellipsis whitespace-nowrap break-all"
            >
              <Input
                v-if="item.isEdit"
                v-model:value="item.name"
                class="w-8/12"
                size="small"
                @keydown="handleEnter(item, $event)"
              />
              <span v-else>{{ item.name }}</span>
            </div>
            <div
              v-if="isActive(item.id)"
              class="visible absolute right-1 z-10 flex"
            >
              <template v-if="item.isEdit">
                <Button
                  size="small"
                  type="text"
                  @click="handleSave(item, $event)"
                >
                  <CheckOutlined />
                </Button>
                <Button
                  size="small"
                  type="text"
                  @click="handleCancel(item, $event)"
                >
                  <CloseOutlined />
                </Button>
              </template>
              <template v-else>
                <Button
                  size="small"
                  type="text"
                  @click="handleEdit(item, $event)"
                >
                  <EditOutlined />
                </Button>
                <Button
                  size="small"
                  type="text"
                  @click="handleDelete(item, $event)"
                >
                  <DeleteOutlined />
                </Button>
              </template>
            </div>
          </a>
        </div>
      </template>
    </div>
  </div>
</template>
