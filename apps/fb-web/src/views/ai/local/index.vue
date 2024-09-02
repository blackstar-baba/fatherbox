<script lang="ts" setup>
import { ref } from 'vue';

import { Page } from '@vben/common-ui';

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
</script>
<template>
  <Page description="use local models running on ollama" title="Local ai">
    <deep-chat
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
