<script lang="ts" setup>
import { ref } from 'vue';

import { Page } from '@vben/common-ui';

import { Card, Col, Row } from 'ant-design-vue';

import FileTree from '#/components/file/FileTree.vue';
import Cherry from '#/components/markdown/cherry.vue';

const cherryRef = ref();
const text = ref('### Hello FatherBox');

function setContent(content: string) {
  text.value = content;
  // update manual, prevent circulation
  cherryRef.value?.setContent(text.value);
}

function updateText(content: string) {
  text.value = content;
}

// how to get locale & dark mode
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
          <FileTree :content="text" @send-content="setContent" />
        </Card>
      </Col>
      <Col :span="18">
        <Card :body-style="{ height: '500px' }" :bordered="false">
          <Cherry
            ref="cherryRef"
            :model-value="text"
            @send-content="updateText"
          />
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
</style>
