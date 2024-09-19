<script lang="ts" setup>
import { ref } from 'vue';

import { Page } from '@vben/common-ui';

import { CompressOutlined, ExpandOutlined } from '@ant-design/icons-vue';
import { save } from '@tauri-apps/api/dialog';
import { writeTextFile } from '@tauri-apps/api/fs';
import { Button } from 'ant-design-vue';

import { downloadByData } from '#/utils/file/downloadUtil';

import Cherry from './cherry.vue';

const text = ref('### Hello World');

const fileInput = ref<HTMLInputElement>();
const fileName = ref('demo.md');

function importDoc() {
  if (fileInput.value) {
    fileInput.value.click();
  }
}
async function exportDoc() {
  if (window.__TAURI__) {
    const filePath = await save({ defaultPath: fileName.value });
    if (filePath) {
      await writeTextFile(filePath, text.value);
    }
  } else {
    downloadByData(text.value, fileName.value);
  }
}

// todo how to get changed in cherry-editor

function handleFileChange(event: any) {
  const file = event.target.files[0];
  fileName.value = file.name;
  if (file) {
    const fileReader = new FileReader();
    fileReader.addEventListener('load', () => {
      text.value = fileReader.result as string;
    });
    // eslint-disable-next-line unicorn/prefer-blob-reading-methods
    fileReader.readAsText(file);
  }
}

// how to get locale & dark mode
</script>

<template>
  <Page
    description="use local models running on ollama, need install ollama and download llms first."
    title="Markdown Editor"
  >
    <div class="mb-2">
      <Button type="primary" @click="importDoc">
        <template #icon>
          <ExpandOutlined />
        </template>
        Import
      </Button>
      <input
        ref="fileInput"
        style="display: none"
        type="file"
        @change="handleFileChange"
      />
      <Button class="ml-2" danger type="primary" @click="exportDoc">
        <template #icon>
          <CompressOutlined />
        </template>
        Export
      </Button>
    </div>
    <!--    // todo 汉化的处理，皮肤的处理，自适应大小-->
    <Cherry />
  </Page>
</template>

<style>
.cherry-markdown ol {
  list-style: decimal;
}

.cherry-markdown ul {
  list-style: disc;
}
</style>
