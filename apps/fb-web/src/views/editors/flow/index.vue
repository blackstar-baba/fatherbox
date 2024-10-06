<script lang="ts" setup>
import { ref } from 'vue';

import { Page } from '@vben/common-ui';

import { CompressOutlined, ExpandOutlined } from '@ant-design/icons-vue';
import { save } from '@tauri-apps/api/dialog';
import { writeBinaryFile, writeTextFile } from '@tauri-apps/api/fs';
import { Button } from 'ant-design-vue';

import Diagram from '#/components/flow/Diagram.vue';
import { downloadByData } from '#/utils/file/downloadUtil';

const flowRef = ref();
const text = ref('### Hello World');
const fileInput = ref<HTMLInputElement>();
const fileName = ref('demo.flow');

function importFlow() {
  if (fileInput.value) {
    fileInput.value.click();
  }
}
async function exportFlow() {
  if (window.__TAURI__) {
    const filePath = await save({ defaultPath: fileName.value });
    if (filePath && flowRef) {
      await writeTextFile(filePath, flowRef.value.getContent());
    }
  } else {
    downloadByData(flowRef.value.getContent(), fileName.value);
  }
}

// logic flow export safari issue: https://github.com/didi/LogicFlow/issues/1466
async function exportImg() {
  const img = await flowRef.value.getImg();
  const blob = img.data;
  if (window.__TAURI__) {
    const filePath = await save({ defaultPath: 'flow.png' });
    if (filePath && flowRef) {
      // 将 Blob 转换为 ArrayBuffer
      const arrayBuffer = await blob.arrayBuffer();
      // 检查是否转换成功
      await writeBinaryFile(filePath, arrayBuffer);
    }
  } else {
    downloadByData(blob, 'flow.png');
  }
}

function handleFileChange(event: any) {
  const file = event.target.files[0];
  fileName.value = file.name;
  if (file) {
    const fileReader = new FileReader();
    fileReader.addEventListener('load', () => {
      text.value = fileReader.result as string;
      if (flowRef.value) {
        flowRef.value.setContent(text.value);
      }
    });
    // eslint-disable-next-line unicorn/prefer-blob-reading-methods
    fileReader.readAsText(file);
  }
}
</script>

<template>
  <Page description="flow chart" title="Flow Editor">
    <div class="mb-2">
      <Button type="primary" @click="importFlow">
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
      <Button class="ml-2" danger type="primary" @click="exportFlow">
        <template #icon>
          <CompressOutlined />
        </template>
        Export
      </Button>
      <Button class="ml-2" danger type="primary" @click="exportImg">
        <template #icon>
          <CompressOutlined />
        </template>
        Export Image
      </Button>
    </div>
    <div class="flow-container">
      <Diagram ref="flowRef" />
    </div>
  </Page>
</template>
<style scoped>
.flow-container {
  width: 100%;
  background-color: #f8f9fa;
}
</style>
