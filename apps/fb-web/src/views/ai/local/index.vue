<script lang="ts" setup>
import { ref } from 'vue';

import { Page } from '@vben/common-ui';

import 'deep-chat';

const history = ref([
  { role: 'user', text: 'Hey, how are you today?' },
  { role: 'ai', text: 'I am doing very well!' },
]);
</script>
<template>
  <Page description="local models running on ollama" title="Local ai">
    <deep-chat
      :demo="true"
      :history="history"
      :text-input="{ placeholder: { text: 'Welcome to the demo!' } }"
      style="border-radius: 8px"
    />
  </Page>
</template>
