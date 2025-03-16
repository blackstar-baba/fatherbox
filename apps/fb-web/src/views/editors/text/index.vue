<script lang="ts" setup>
import { ref } from 'vue';

import { Page } from '@vben/common-ui';

import { Card, Col, Row, Textarea } from 'ant-design-vue';

import { decodeUint8ArrayToString } from '#/components/file/file';
import FileOperate from '#/components/file/FileOperate.vue';

const text = ref('');

function setContent(content: string) {
  text.value = decodeUint8ArrayToString(content);
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
