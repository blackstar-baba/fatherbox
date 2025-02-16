<script lang="ts" setup>
import { onMounted } from 'vue';

import { Page } from '@vben/common-ui';

import { Card } from 'ant-design-vue';

import Chat from '#/components/chat/index.vue';

onMounted(() => {});
</script>
<template>
  <Page
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
