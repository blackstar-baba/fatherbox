<script lang="ts" setup>
import { ref } from 'vue';

import { Page } from '@vben/common-ui';

import { Card, Col, Row } from 'ant-design-vue';

import DirTree from '#/components/file/DirTree.vue';
import FileTable from '#/components/file/FileTable.vue';

const fileTable = ref<any>();

const updateTable = (pid: string) => {
  fileTable.value.updateTable(pid);
};
</script>
<template>
  <Page auto-content-height description="File Manage" title="File Manage">
    <Row :gutter="16">
      <Col :span="6">
        <!-- todo user cal style better -->
        <Card :body-style="{ height: '500px' }" :bordered="false">
          <DirTree @selected-id="updateTable" />
        </Card>
      </Col>
      <Col :span="18">
        <Card :body-style="{ height: '500px' }" :bordered="false">
          <FileTable ref="fileTable" />
        </Card>
      </Col>
    </Row>
  </Page>
</template>
