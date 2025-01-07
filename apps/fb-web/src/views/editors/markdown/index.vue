<script lang="ts" setup>
import type { Key } from 'ant-design-vue/es/_util/type';

import { ref } from 'vue';

import { Page } from '@vben/common-ui';

import { Card, Col, Row, TabPane, Tabs } from 'ant-design-vue';

import { type FileContent } from '#/components/file/file';
import FileTree from '#/components/file/FileTree.vue';
import Cherry from '#/components/markdown/cherry.vue';

const textRef = ref('Hello FatherBox Markdown Editor');
const filesRef = ref<FileContent[]>([]);
const activeKeyRef = ref('');

function setFile(fileContent: FileContent) {
  if (!filesRef.value.some((k) => k.id === fileContent.id)) {
    filesRef.value.push(fileContent);
  }
  activeKeyRef.value = fileContent.id;
}

function updateFile(file: { id: string; name: string }) {
  for (let i = 0; i < filesRef.value.length; i++) {
    const existFile = filesRef.value[i];
    if (existFile && existFile.id === file.id) {
      existFile.name = file.name;
      break;
    }
  }
}

function deleteFile(id: string) {
  const key = id;
  let lastIndex = 0;
  for (let i = 0; i < filesRef.value.length; i++) {
    const file = filesRef.value[i];
    if (file && file.id === key) {
      lastIndex = i - 1;
      if (lastIndex < 0) {
        activeKeyRef.value = '';
      } else {
        const lastFile = filesRef.value[lastIndex];
        if (lastFile) {
          activeKeyRef.value = lastFile.id;
        }
      }
    }
  }
  filesRef.value = filesRef.value.filter((file) => file.id !== key);
}

function importContent(content: string) {
  const existedFileContent = filesRef.value.filter(
    (k) => k.id === activeKeyRef.value,
  );
  if (existedFileContent.length > 0 && existedFileContent[0]) {
    existedFileContent[0].content = content;
  }
}

const onTabEdit = (
  targetKey: Key | KeyboardEvent | MouseEvent,
  action: 'add' | 'remove',
) => {
  const key = targetKey as string;
  if (action === 'remove') {
    let lastIndex = 0;
    for (let i = 0; i < filesRef.value.length; i++) {
      const file = filesRef.value[i];
      if (file && file.id === key) {
        lastIndex = i - 1;
        if (lastIndex < 0) {
          activeKeyRef.value = '';
        } else {
          const lastFile = filesRef.value[lastIndex];
          if (lastFile) {
            activeKeyRef.value = lastFile.id;
          }
        }
      }
    }
    filesRef.value = filesRef.value.filter((file) => file.id !== key);
  }
};
</script>

<template>
  <Page
    auto-content-height
    description="Cherry Markdown Editor is a Javascript Markdown editor. It has the advantages such as out-of-the-box, lightweight and easy to extend."
    title="Markdown Editor"
  >
    <Row :gutter="16">
      <Col :span="6">
        <Card :body-style="{ height: '500px' }" :bordered="false">
          <FileTree
            :active-file-id="activeKeyRef"
            :content="textRef"
            :edit-files="filesRef"
            @delete="deleteFile"
            @import-content="importContent"
            @open="setFile"
            @update="updateFile"
          />
        </Card>
      </Col>
      <Col :span="18">
        <Card :body-style="{ height: '500px' }" :bordered="false">
          <Tabs
            v-model:active-key="activeKeyRef"
            hide-add
            type="editable-card"
            @edit="onTabEdit"
          >
            <TabPane key="" :closable="false" tab="Introduction">
              {{ textRef }}
            </TabPane>
            <TabPane
              v-for="(file, index) in filesRef"
              :key="file.id"
              :tab="file.name"
            >
              <Cherry v-model:file="filesRef[index]" :md-id="file.id" />
            </TabPane>
          </Tabs>
        </Card>
      </Col>
    </Row>
  </Page>
</template>

<style>
.cherry-markdown ol {
  list-style: decimal;
}

.cherry-markdown ul {
  list-style: disc;
}

.ant-tabs-card .ant-tabs-content {
  height: 400px;
  margin-top: -8px;
}
</style>

<!-- todo auto height -->
