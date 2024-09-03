<script lang="ts" setup>
import { onMounted, ref } from 'vue';

import { Page } from '@vben/common-ui';

import { chatRequest } from '#/api/core/ai';

import 'deep-chat';

const history = ref([
  { role: 'user', text: 'Hey, how are you today?' },
  { role: 'ai', text: 'I am doing very well!' },
]);

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

const chatElementRef = ref(null);

onMounted(() => {
  // Step 2: Access the chat component and set up the connect handler
  if (chatElementRef.value) {
    chatElementRef.value.connect = {
      handler: (body: any, signals: any) => {
        try {
          // console.info(body);
          // Example: Using fetch to get data from a server
          // todo use stream to do this
          chatRequest(body)
            .then((data) => {
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
  <Page description="use local models running on ollama" title="Local ai">
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
  </Page>
</template>
