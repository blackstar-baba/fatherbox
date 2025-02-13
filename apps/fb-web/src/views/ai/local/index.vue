<script lang="ts" setup>
import { onMounted, ref } from 'vue';

import { Page } from '@vben/common-ui';

import { useResizeObserver } from '@vueuse/core';
import { Card } from 'ant-design-vue';

import Chat from '#/components/chat/index.vue';

const pageElementRef = ref<any>(null);

const chatStyle = ref({
  'border-radius': '8px',
  height: `500px`,
  width: `1024px`,
});

function updateChatStyle() {
  if (pageElementRef.value) {
    const rect = pageElementRef.value.$el.getBoundingClientRect();
    chatStyle.value.height = `${rect.height - 220}px`;
    chatStyle.value.width = `${rect.width - 60}px`;
  }
}

onMounted(() => {
  useResizeObserver(pageElementRef, (_) => {
    updateChatStyle();
  });
});
</script>
<template>
  <Page
    ref="pageElementRef"
    auto-content-height
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
      <div class="my-2" style="width: 100%; height: 450px">
        <Chat />
      </div>
    </Card>
  </Page>
</template>
