<script setup lang="ts">
import { onMounted, onUnmounted, type Ref, ref, watchEffect } from 'vue';

import { LucideInfo } from '@vben/icons';

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
  editChatMessageWithStream,
  generateChatMessageWithStream,
  getChatMessages,
  getChats,
  listEnableAiSource,
  listEnableAiSourceModels,
  regenerateChatMessageWithStream,
  type Source,
} from '#/api';
import { $t } from '#/locales';
import { useWorkspaceStore } from '#/store';

import History from './history/index.vue';
import Input from './input/index.vue';
import Message from './message/index.vue';
import { useScroll } from './useScroll';

const { scrollRef, scrollToBottom, scrollToBottomIfAtBottom } = useScroll();

const sourcesRef = ref<SelectProps['options']>([]);
const sourceRef = ref('');
const modelsRef = ref<SelectProps['options']>([]);
const modelRef = ref('');
const templatesRef = ref<SelectProps['options']>([]);
const templateRef = ref('');
const activeIdRef = ref<null | string>(null);
const chatInfosRef = ref<ChatInfo[]>([]);

const workspaceStore = useWorkspaceStore();

// todo we need store to cache message
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
  chatMessagesRef.value = await getChatMessages({ id });
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
    const currWorkspace = workspaceStore.getWorkspace();
    if (currWorkspace) {
      const chatInfo = await createChat({
        name: message.slice(0, 20),
        wid: currWorkspace.id,
      });
      chatInfosRef.value.push(chatInfo);
      currentUuid = chatInfo.id;
      activeIdRef.value = currentUuid;
    }
  }
  if (currentUuid === null) {
    return;
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
      content: '',
      loading: true,
    },
  );
  scrollToBottom();
  const lastChatMessage =
    chatMessagesRef.value[chatMessagesRef.value.length - 1];
  // request
  await generateChatMessageWithStream({
    id: currentUuid,
    prompt: message,
    modelId: modelRef.value,
    sourceId: sourceRef.value,
    onProgress: (message: null | string, _: number) => {
      if (lastChatMessage) {
        if (message) {
          lastChatMessage.content += message;
        } else {
          lastChatMessage.loading = false;
          loadingRef.value = false;
        }
      }
      scrollToBottomIfAtBottom();
    },
  });
}

async function onRegenerate(index: number) {
  if (loadingRef.value) {
    return;
  }
  if (!chatMessagesRef.value[index]) {
    message.error($t('chat.message.notFound'));
    return;
  }
  if (activeIdRef.value === null) {
    message.error($t('chat.message.notFound'));
    return;
  }

  loadingRef.value = true;
  chatMessagesRef.value.splice(index);
  // push system message
  chatMessagesRef.value.push({
    role: 'system',
    content: '',
    loading: true,
  });

  const lastChatMessage =
    chatMessagesRef.value[chatMessagesRef.value.length - 1];

  await regenerateChatMessageWithStream({
    id: activeIdRef.value,
    index,
    modelId: modelRef.value,
    sourceId: sourceRef.value,
    onProgress: (message: null | string, _: number) => {
      if (lastChatMessage) {
        if (message) {
          lastChatMessage.content += message;
        } else {
          lastChatMessage.loading = false;
          loadingRef.value = false;
        }
      }
      scrollToBottomIfAtBottom();
    },
  });
}

async function onEdit(index: number, newPrompt: string) {
  if (loadingRef.value) {
    return;
  }
  if (!chatMessagesRef.value[index]) {
    message.error($t('chat.message.notFound'));
    return;
  }
  if (activeIdRef.value === null) {
    message.error($t('chat.message.notFound'));
    return;
  }

  loadingRef.value = true;
  // delete old messages
  chatMessagesRef.value.splice(index);
  // add user message & think system message
  chatMessagesRef.value.push(
    {
      role: 'user',
      content: newPrompt,
      loading: false,
    },
    {
      role: 'system',
      content: '',
      loading: true,
    },
  );

  const lastChatMessage =
    chatMessagesRef.value[chatMessagesRef.value.length - 1];

  await editChatMessageWithStream({
    id: activeIdRef.value,
    index,
    modelId: modelRef.value,
    sourceId: sourceRef.value,
    onProgress: (message: null | string, _: number) => {
      if (lastChatMessage) {
        if (message) {
          lastChatMessage.content += message;
        } else {
          lastChatMessage.loading = false;
          loadingRef.value = false;
        }
      }
      scrollToBottomIfAtBottom();
    },
    prompt: newPrompt,
  });
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

// function handleStop() {
//   if (loadingRef.value) {
//     loadingRef.value = false;
//   }
// }

onMounted(async () => {
  scrollToBottom();
  if (inputRef.value) inputRef.value?.focus();

  // get source
  const sources = await listEnableAiSource();
  sources.forEach((source: Source) => {
    sourcesRef.value?.push({
      label: source.name,
      value: source.id,
    });
  });
  if (sourcesRef.value && sourcesRef.value.length > 0) {
    sourceRef.value = sourcesRef.value[0]?.value as string;
  }

  if (sourceRef.value) {
    const models = await listEnableAiSourceModels({
      sourceId: sourceRef.value,
    });
    models.forEach((model: any) => {
      modelsRef.value?.push({
        label: model.name,
        value: model.id,
      });
    });
    if (modelsRef.value && modelsRef.value.length > 0) {
      modelRef.value = modelsRef.value[0]?.value as string;
    }
  }
});

onUnmounted(() => {});

watchEffect(async () => {
  const wid = workspaceStore.getId();
  if (wid) {
    chatInfosRef.value = await getChats({
      wid,
    });
  }
});
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
          <span class="ml-5">Sources</span>
          <LucideInfo class="ml-1 inline size-4" />
          <Select
            v-model:value="sourceRef"
            :options="sourcesRef"
            class="ml-2 w-36"
            placeholder="model"
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
                      :error="item.error"
                      :inversion="item.role === 'user'"
                      :loading="item.loading"
                      :text="
                        item.content
                          ? item.content
                          : $t('chat.message.thinking')
                      "
                      @edit="(returnValue) => onEdit(index, returnValue)"
                      @regenerate="onRegenerate(index)"
                    />
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
