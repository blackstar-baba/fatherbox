<script lang="ts" setup>
import { ref } from 'vue';

import { Page } from '@vben/common-ui';

import { CompressOutlined, ExpandOutlined } from '@ant-design/icons-vue';
import { Button } from 'ant-design-vue';

import MilkEditor from './milk-editor.vue';

const text = ref('### hello world');

function open() {}
function exportDoc() {}
</script>

<template>
  <Page
    description="use local models running on ollama, need install ollama and download llms first."
    title="Markdown Editor"
  >
    <div class="mb-2">
      <ButtonCounter />
      <Button type="primary" @click="open">
        <template #icon>
          <ExpandOutlined />
        </template>
        Open
      </Button>
      <Button class="ml-2" danger type="primary" @click="exportDoc">
        <template #icon>
          <CompressOutlined />
        </template>
        Export
      </Button>
    </div>
    <MilkEditor :markdown="text" />
  </Page>
</template>
