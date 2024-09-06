<script lang="ts" setup>
import { onMounted, ref } from 'vue';

import { Page } from '@vben/common-ui';
import { LucideInfo } from '@vben/icons';

import { Card, Select, type SelectProps } from 'ant-design-vue';
import { v4 as uuidv4 } from 'uuid';

import {
  chatRequest,
  getChatHistoryMessages,
  getChats,
  getModels,
} from '#/api/core/ai';

import 'deep-chat';

const models = ref<SelectProps['options']>([]);
const model = ref('');
const templates = ref<SelectProps['options']>([]);
const template = ref('');
const chats = ref<SelectProps['options']>([]);
const chatId = ref('');
const chatElementRef = ref<any>(null);

const history = ref([]);
// { role: 'user', text: 'Hey, how are you today?' },
// { role: 'ai', text: 'I am doing very well!' },

const avatars = ref({
  ai: { src: '/logo.png', styles: { avatar: { marginLeft: '-3px' } } },
  default: {
    src: '/avatar.svg',
    styles: {
      avatar: { height: '30px', width: '30px' },
      container: { marginTop: '8px' },
    },
  },
});

const style = ref({
  'border-radius': '8px',
  width: '1024px',
});

function getChatHistory(id: string) {
  getChatHistoryMessages(id).then((data: any) => {
    history.value = data;
  });
}

function updateChats(id: string) {
  getChats().then((data: any) => {
    chats.value = [];
    data.forEach((charInfo: any) => {
      chats.value?.push({
        label: charInfo.name,
        value: charInfo.id,
      });
      chatId.value = id;
    });
  });
}

onMounted(() => {
  // get chats
  getChats().then((data: any) => {
    chats.value = [];
    chatId.value = '';
    data.forEach((charInfo: any) => {
      chats.value?.push({
        label: charInfo.name,
        value: charInfo.id,
      });
    });
    if (chats.value.length > 0) {
      chatId.value = chats.value[0]?.value as string;
      getChatHistory(chatId.value);
    }
  });
  // get models
  getModels().then((data: any) => {
    models.value = [];
    model.value = '';
    data.models.forEach((model: any) => {
      models.value?.push({
        label: model.name,
        value: model.name,
      });
    });
    if (models.value.length > 0) {
      model.value = models.value[0]?.value as string;
    }
  });

  if (chatElementRef.value) {
    chatElementRef.value.connect = {
      handler: (body: any, signals: any) => {
        try {
          // todo div auto scale
          // default model if exist
          const existChat: boolean = chatId.value !== '';
          body.model = model.value;
          body.stream = false;
          body.id = existChat ? chatId.value : uuidv4();
          chatRequest(body)
            .then((data: any) => {
              if (!existChat) {
                updateChats(chatId.value);
              }
              signals.onResponse({
                text: data.text.toString() || 'Handler response',
              });
            })
            .catch((error) => {
              signals.onResponse({ error: `Fetch error: ${error.message}` });
            });
        } catch (error: any) {
          signals.onResponse({ error: `Handler error: ${error.message}` });
        }
      },
    };
  }
});
</script>
<template>
  <Page
    description="use local models running on ollama, need install ollama and download llms first."
    title="Local AI"
  >
    <template #extra>
      <div>
        <span class="text-base font-normal underline">
          <a href="https://ollama.com/download" target="_blank">
            Download Ollama
          </a>
        </span>
      </div>
    </template>
    <Card :bordered="false">
      <div class="my-2">
        <span>Models</span>
        <LucideInfo class="ml-1 inline size-4" />
        <Select
          v-model:value="model"
          :options="models"
          class="ml-2 w-36"
          placeholder="model"
        />
        <span class="ml-2">Prompt Templates</span>
        <Select
          v-model:value="template"
          :options="templates"
          class="ml-2 w-36"
          placeholder="template"
        />
      </div>
      <deep-chat
        ref="chatElementRef"
        :avatars="avatars"
        :demo="true"
        :history="history"
        :style="style"
        :text-input="{ placeholder: { text: 'Welcome to the demo!' } }"
        images="true"
        mixed-files="true"
      />
    </Card>
  </Page>
</template>
