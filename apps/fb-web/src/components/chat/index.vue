<script setup lang="ts">
import { onMounted, onUnmounted, type Ref, ref } from 'vue';

import { LucideInfo, RiStopCircleFill } from '@vben/icons';

import {
  DownloadOutlined,
  MenuFoldOutlined,
  MenuUnfoldOutlined,
} from '@ant-design/icons-vue';
import { save } from '@tauri-apps/api/dialog';
import { writeBinaryFile } from '@tauri-apps/api/fs';
import {
  Button,
  Layout,
  LayoutContent,
  LayoutFooter,
  LayoutHeader,
  LayoutSider,
  message,
  Select,
  type SelectProps,
} from 'ant-design-vue';
import { toBlob } from 'html-to-image';

import {
  type ChatInfo,
  type ChatMessage,
  createChat,
  editMessage,
  fetchChatAPIProcessNew,
  getChatMessages,
  getChats,
  getModels,
  regenerateMessage,
} from '#/api';
import { $t } from '#/locales';

import { History, Input, Message } from './components';
import { useScroll } from './hooks/useScroll';

const { scrollRef, scrollToBottom, scrollToBottomIfAtBottom } = useScroll();

const modelsRef = ref<SelectProps['options']>([]);
const modelRef = ref('');
const templatesRef = ref<SelectProps['options']>([]);
const templateRef = ref('');
const activeIdRef = ref<null | string>(null);
const chatInfosRef = ref<ChatInfo[]>([]);

// todo we need store to cache message
// todo 什么时候cache，cache什么时候消失
// const db = window.indexedDB.open('chat', 1);
// db.transaction('messages', 'readwrite');
// const store = transaction.objectStore('users');
// store.put({ id: 1, name: 'chat-test', age: 25 });
const chatMessagesRef = ref<ChatMessage[]>([]);

const collapsedRef = ref<boolean>(false);
const loadingRef = ref<boolean>(false);
const inputRef = ref<null | Ref>(null);

async function handleSelect(id: null | string) {
  activeIdRef.value = id;
  if (!id) {
    chatMessagesRef.value = [];
    return;
  }
  chatMessagesRef.value = (await getChatMessages({ id })) as ChatMessage[];
}

async function handleSubmit(value: string) {
  await onConversation(value);
}

async function onConversation(message: string) {
  if (loadingRef.value) return;
  if (!message || message.trim() === '') return;
  loadingRef.value = true;
  let currentUuid = activeIdRef.value;
  // create new chat
  if (currentUuid === null) {
    const chatInfo = await createChat({ name: message.slice(0, 20) });
    chatInfosRef.value.push(chatInfo);
    currentUuid = chatInfo.id;
    activeIdRef.value = currentUuid;
  }

  // add user message & think system message
  chatMessagesRef.value.push(
    {
      role: 'user',
      content: message,
      loading: false,
    },
    {
      role: 'system',
      content: $t('chat.message.thinking'),
      loading: true,
    },
  );
  scrollToBottom();
  // request
  await fetchChatAPIProcessNew({
    id: currentUuid,
    prompt: message,
    model: modelRef.value,
    onDownloadProgress: (message: string) => {
      const chatMessage =
        chatMessagesRef.value[chatMessagesRef.value.length - 1];
      if (chatMessage) {
        chatMessage.content = message;
        chatMessage.loading = false;
      }
      scrollToBottomIfAtBottom();
    },
  });
  loadingRef.value = false;
}

async function onRegenerate(index: number) {
  if (loadingRef.value) {
    return;
  }
  if (!chatMessagesRef.value[index]) {
    message.error($t('chat.message.notFound'));
    return;
  }
  loadingRef.value = true;

  chatMessagesRef.value.splice(index);
  // push system message
  chatMessagesRef.value.push({
    role: 'system',
    content: $t('chat.message.thinking'),
    loading: true,
  });

  if (activeIdRef.value === null) {
    message.error($t('chat.message.notFound'));
    return;
  }

  await regenerateMessage({
    id: activeIdRef.value,
    index,
    model: modelRef.value,
    onDownloadProgress: (message: string) => {
      const chatMessage =
        chatMessagesRef.value[chatMessagesRef.value.length - 1];
      if (chatMessage) {
        chatMessage.content = message;
        chatMessage.loading = false;
      }
      scrollToBottomIfAtBottom();
    },
  });
  loadingRef.value = false;
}

async function onEdit(index: number, newPrompt: string) {
  if (loadingRef.value) {
    return;
  }
  if (!chatMessagesRef.value[index]) {
    message.error($t('chat.message.notFound'));
    return;
  }
  loadingRef.value = true;
  // delete old messages
  chatMessagesRef.value.splice(index);

  if (activeIdRef.value === null) {
    message.error($t('chat.message.notFound'));
    return;
  }
  // add user message & think system message
  chatMessagesRef.value.push(
    {
      role: 'user',
      content: newPrompt,
      loading: false,
    },
    {
      role: 'system',
      content: $t('chat.message.thinking'),
      loading: true,
    },
  );

  scrollToBottom();
  await editMessage({
    id: activeIdRef.value,
    index,
    model: modelRef.value,
    onDownloadProgress: (message: string) => {
      const chatMessage =
        chatMessagesRef.value[chatMessagesRef.value.length - 1];
      if (chatMessage) {
        chatMessage.content = message;
        chatMessage.loading = false;
      }
      scrollToBottomIfAtBottom();
    },
    prompt: newPrompt,
  });
  loadingRef.value = false;
}

async function handleExport() {
  const fileName = 'chat-shot.png';
  if (window.__TAURI__) {
    const filePath = await save({ defaultPath: fileName });
    if (filePath) {
      const ele = document.querySelector('#image-wrapper');
      const blob = await toBlob(ele as HTMLDivElement);
      if (blob) {
        const arrayBuffer = await blob.arrayBuffer();
        await writeBinaryFile(filePath, arrayBuffer);
      }
    }
  }
}

function handleStop() {
  if (loadingRef.value) {
    loadingRef.value = false;
  }
}

onMounted(async () => {
  chatInfosRef.value = await getChats();
  scrollToBottom();
  if (inputRef.value) inputRef.value?.focus();
  // get models
  getModels().then((data: any) => {
    modelsRef.value = [];
    modelRef.value = '';
    data.models.forEach((model: any) => {
      modelsRef.value?.push({
        label: model.name,
        value: model.name,
      });
    });
    if (modelsRef.value.length > 0) {
      modelRef.value = modelsRef.value[0]?.value as string;
    }
  });
});

onUnmounted(() => {});

// watch(
//   () => chatStore.getActive(),
//   (value) => {
//     if (value) {
//       uuidRef.value = value;
//       chatMessagesRef.value = chatStore.getChatMessages(+uuidRef.value);
//       conversationsRef.value = chatMessagesRef.value.filter(
//         (item) => !item.inversion && !!item.conversationOptions,
//       );
//     } else {
//       uuidRef.value = -1;
//       chatMessagesRef.value = [];
//       conversationsRef.value = [];
//     }
//   },
// );
</script>

<template>
  <div class="h-full">
    <Layout style="height: 100%">
      <LayoutSider
        v-model:collapsed="collapsedRef"
        :trigger="null"
        collapsed-width="0"
        collapsible
        width="250"
      >
        <History
          v-model:chat-infos="chatInfosRef"
          :active="activeIdRef"
          :loading="loadingRef"
          @select="handleSelect"
        />
      </LayoutSider>
      <Layout>
        <LayoutHeader style="padding: 4px">
          <MenuUnfoldOutlined
            v-if="collapsedRef"
            class="trigger"
            @click="() => (collapsedRef = !collapsedRef)"
          />
          <MenuFoldOutlined
            v-else
            class="trigger"
            @click="() => (collapsedRef = !collapsedRef)"
          />
          <span class="ml-5">Models</span>
          <LucideInfo class="ml-1 inline size-4" />
          <Select
            v-model:value="modelRef"
            :options="modelsRef"
            class="ml-2 w-36"
            placeholder="model"
          />
          <span class="ml-5">Prompt Templates</span>
          <Select
            v-model:value="templateRef"
            :options="templatesRef"
            class="ml-2 w-36"
            placeholder="template"
          />
        </LayoutHeader>
        <LayoutContent style="height: 100%">
          <div
            id="scrollRef"
            ref="scrollRef"
            class="h-full overflow-hidden overflow-y-auto"
          >
            <div class="m-auto h-full w-full max-w-screen-xl p-4">
              <div id="image-wrapper" class="relative">
                <template v-if="!activeIdRef">
                  <div
                    class="mt-32 flex items-center justify-center text-center text-neutral-300"
                  >
                    <Input :loading="loadingRef" @submit="handleSubmit" />
                  </div>
                </template>
                <template v-else>
                  <div>
                    <Message
                      v-for="(item, index) of chatMessagesRef"
                      :key="index"
                      :date-time="item.time"
                      :error="item.error"
                      :inversion="item.role === 'user'"
                      :loading="item.loading"
                      :text="item.content"
                      @edit="(returnValue) => onEdit(index, returnValue)"
                      @regenerate="onRegenerate(index)"
                    />
                    <div class="sticky bottom-0 left-0 flex justify-center">
                      <Button
                        v-if="loadingRef"
                        type="primary"
                        @click="handleStop"
                      >
                        <template #icon>
                          <RiStopCircleFill />
                        </template>
                        {{ $t('chat.message.stopResponding') }}
                      </Button>
                    </div>
                  </div>
                </template>
              </div>
            </div>
          </div>
        </LayoutContent>
        <LayoutFooter v-if="activeIdRef">
          <div
            class="mt-5 flex items-center justify-center text-center text-neutral-300"
          >
            <Button
              v-if="!loadingRef"
              class="mr-2"
              type="primary"
              @click="handleExport"
            >
              <DownloadOutlined />
            </Button>
            <Input :loading="loadingRef" @submit="handleSubmit" />
          </div>
        </LayoutFooter>
      </Layout>
    </Layout>
  </div>
</template>
