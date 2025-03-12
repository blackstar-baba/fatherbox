<script lang="ts" setup>
import { computed, onMounted, type StyleValue } from 'vue';

import { Page } from '@vben/common-ui';
import { CSS_VARIABLE_LAYOUT_CONTENT_HEIGHT } from '@vben-core/shared/constants';

import { Card } from 'ant-design-vue';

import Chat from '#/components/chat/index.vue';

const contentStyle = computed<StyleValue>(() => {
  return {
    width: '100%',
    height: `calc(var(${CSS_VARIABLE_LAYOUT_CONTENT_HEIGHT}) - 180px)`,
    overflowY: 'auto',
  };
});

onMounted(() => {});
</script>
<template>
  <Page
    auto-content-height
    description="use ollama local models or remote modes."
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
      <div :style="contentStyle" class="my-2">
        <Chat />
      </div>
    </Card>
  </Page>
</template>
