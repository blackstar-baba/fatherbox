<script lang="ts" setup>
import { ref } from 'vue';

import { Page } from '@vben/common-ui';

import { CopyOutlined } from '@ant-design/icons-vue';
import {
  Button,
  Card,
  Col,
  message,
  Row,
  Textarea,
  Tooltip,
} from 'ant-design-vue';

import FileOperate from '#/components/file/FileOperate.vue';

const text = ref('');
function copy() {
  const input = document.createElement('textarea');
  input.style.position = 'fixed';
  input.style.opacity = String(0);
  input.value = text.value;
  document.body.append(input);
  input.select();
  document.execCommand('Copy');
  input.remove();
  message.success('copy success');
}

function setContent(content: string) {
  text.value = content;
}
</script>
<template>
  <Page
    description="A general-purpose editor that can edit all text except for specific zones."
    title="Text Editor"
  >
    <Row :gutter="16">
      <Col :span="2">
        <FileOperate :content="text" @send-content="setContent" />
      </Col>
      <Col :span="22">
        <div class="py-4">
          <Tooltip placement="top">
            <template #title> Copy </template>
            <Button @click="copy">
              <template #icon>
                <CopyOutlined />
              </template>
            </Button>
          </Tooltip>
        </div>
        <Card :body-style="{ height: '500px' }" :bordered="false">
          <Textarea
            v-model:value="text"
            :maxlength="20000"
            :rows="17"
            placeholder="a simple text editor"
            show-count
          />
        </Card>
      </Col>
    </Row>
  </Page>
</template>
