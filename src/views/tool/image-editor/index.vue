<template>
  <PageWrapper contentBackground title="Image Editor" content="todo">
    <Row :gutter="16">
      <Col :span="4"> // todo 相册，然后弹出对话框进行编辑 </Col>
      <Col :span="20">
        <div class="py-4">
          <Button type="success">
            <template #icon>
              <ExpandOutlined />
            </template>
            Save
          </Button>
          <Button type="warning" class="ml-2">
            <template #icon>
              <CompressOutlined />
            </template>
            Download
          </Button>
        </div>
        <div class="h-150 w-300">
          <ImageEditor />
        </div>
      </Col>
    </Row>
  </PageWrapper>
</template>
<script lang="ts" setup>
  import { PageWrapper } from '@/components/Page';
  import { ImageEditor } from '@/components/ImageEditor';
  import { Button, Col, Row } from 'ant-design-vue';
  import { CompressOutlined, ExpandOutlined } from '@ant-design/icons-vue';
</script>
