<template>
  <PageWrapper title="Markdown editor">
    <div>
      <a-button @click="open" class="mb-2 ml-2" type="primary"> Open </a-button>
      <input type="file" style="display: none" ref="fileInput" @change="handleFileChange" />
      <a-button @click="download" class="mb-2 ml-2" type="primary"> Download </a-button>
      <a-button @click="clearValue" class="mb-2 ml-2" type="default"> Clear </a-button>
      <MarkDown v-model:value="valueRef" @change="handleChange" ref="markDownRef" placeholder="" />
    </div>
    <div class="mt-2">
      <Card title="Markdown Viewer 组件演示">
        <MarkdownViewer :value="valueRef" />
      </Card>
    </div>
  </PageWrapper>
</template>
<script lang="ts" setup>
  import { ref } from 'vue';
  import { MarkDown, MarkDownActionType, MarkdownViewer } from '@/components/Markdown';
  import { PageWrapper } from '@/components/Page';
  import { Card } from 'ant-design-vue';
  import { type Nullable } from '@vben/types';
  import { save } from '@tauri-apps/api/dialog';
  import { writeTextFile } from '@tauri-apps/api/fs';
  import { downloadByData } from '@/utils/file/download';

  const markDownRef = ref<Nullable<MarkDownActionType>>(null);
  const valueRef = ref(`
# 标题h1

##### 标题h5

**加粗**
*斜体*
~~删除线~~
[链接](https://github.com/vbenjs/vue-vben-admin)
↓分割线↓

---


* 无序列表1
  * 无序列表1.1

1. 有序列表1
2. 有序列表2

* [ ] 任务列表1
* [x] 任务列表2

> 引用示例

\`\`\`js
// 代码块:
(() => {
  var htmlRoot = document.getElementById('htmlRoot');
  var theme = window.localStorage.getItem('__APP__DARK__MODE__');
  if (htmlRoot && theme) {
    htmlRoot.setAttribute('data-theme', theme);
    theme = htmlRoot = null;
  }
})();
\`\`\`

| 表格 | 示例 | 🎉️ |
| --- | --- | --- |
| 1 | 2 | 3 |
| 4 | 5 | 6 |
`);
  const fileInput = ref(null);
  const fileName = ref('demo.md');

  function open() {
    fileInput.value.click();
  }

  async function download() {
    if (window.__TAURI__) {
      const filePath = await save({ defaultPath: fileName.value });
      await writeTextFile(filePath, valueRef.value);
    } else {
      downloadByData(valueRef.value, fileName.value);
    }
  }

  function handleChange(v: string) {
    valueRef.value = v;
  }

  function clearValue() {
    valueRef.value = '';
  }

  function handleFileChange(event) {
    let file = event.target.files[0];
    fileName.value = file.name;
    if (file) {
      const fileReader = new FileReader();
      fileReader.onload = () => {
        const fileContent = fileReader.result as string;
        console.log(fileContent);
        valueRef.value = fileContent;
      };
      fileReader.readAsText(file);
    }
  }
</script>
