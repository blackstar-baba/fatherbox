<template>
  <PageWrapper
    contentBackground
    title="Uuid Generator"
    content="Support for RFC4122 version 1, and 4 UUIDs"
  >
    <Row :gutter="16">
      <Col :span="12">
        <Card :bordered="false" class="h-120">
          <div class="py-4">
            <Select ref="select" v-model:value="version" class="w-50" :options="versions" />
            <Select ref="select" v-model:value="number" class="w-20 ml-2" :options="numbers" />
            <Button type="success" class="ml-2" @click="generate">
              <template #icon>
                <EnterOutlined />
              </template>
              Generate
            </Button>
            <Button class="ml-2" @click="download">
              <template #icon>
                <DownloadOutlined />
              </template>
              Download
            </Button>
          </div>
          <div class="h-100 overflow-y-auto">
            <List item-layout="horizontal" :data-source="uuids">
              <template #renderItem="{ item }">
                <ListItem>{{ item }}</ListItem>
              </template>
            </List>
          </div>
        </Card>
      </Col>
      <Col :span="12">
        <Card :bordered="false" class="h-120">
          <Divider orientation="left">Null UUID</Divider>
          <div class="py-4">
            {{ nullUuid }}
            <Button type="success" class="ml-2" @click="copyNullUuid">
              <template #icon>
                <CopyOutlined />
              </template>
              Copy
            </Button>
          </div>
          <Divider orientation="left">UUID Validate</Divider>
          <div class="pt-4 h-20">
            <Input
              v-model:value="needValidateUuid"
              placeholder="6ec0bd7f-11c0-43da-975e-2a8ad9ebae0b"
              class="w-100"
            />
            <Button type="success" class="ml-2" @click="validate">
              <template #icon>
                <CheckOutlined />
              </template>
              Validate
            </Button>
            <p class="my-4">{{ validateResult }}</p>
          </div>
        </Card>
      </Col>
    </Row>
  </PageWrapper>
</template>
<script lang="ts" setup>
  import { ref } from 'vue';
  import {
    NIL as NIL_UUID,
    v1 as uuidv1,
    v4 as uuidv4,
    validate as uuidValidate,
    version as uuidVersion,
  } from 'uuid';
  import { Button, Row, Col, Card, Select, List, ListItem, Input, Divider } from 'ant-design-vue';
  import {
    EnterOutlined,
    DownloadOutlined,
    CopyOutlined,
    CheckOutlined,
  } from '@ant-design/icons-vue';
  import { downloadByData } from '@/utils/file/download';
  import type { SelectProps } from 'ant-design-vue';
  import { PageWrapper } from '@/components/Page';

  const versions = ref<SelectProps['options']>([
    {
      value: '1',
      label: 'Version 1(Time Based)',
    },
    {
      value: '4',
      label: 'Version 4(Random)',
    },
  ]);
  const numbers = ref<SelectProps['options']>([
    {
      value: '1',
      label: '1',
    },
    {
      value: '2',
      label: '2',
    },
    {
      value: '5',
      label: '5',
    },
    {
      value: '10',
      label: '10',
    },
    {
      value: '20',
      label: '20',
    },
    {
      value: '50',
      label: '50',
    },
    {
      value: '100',
      label: '100',
    },
  ]);

  let version = ref('4');
  let number = ref('5');
  let uuids = ref<String[]>([]);
  const nullUuid = ref(NIL_UUID);
  let needValidateUuid = ref(NIL_UUID);
  let validateResult = ref('');

  function generate() {
    let num = parseInt(number.value);
    uuids.value = [];
    for (let i = 0; i < num; i++) {
      switch (version.value) {
        case '1':
          uuids.value.push(uuidv1());
          break;
        case '4':
          uuids.value.push(uuidv4());
          break;
      }
    }
  }

  generate();

  function download() {
    let data = '';
    for (let i = 0; i < uuids.value.length; i++) {
      data += uuids.value[i] + '\n';
    }
    downloadByData(data, 'uuids.txt');
  }

  function validate() {
    if (needValidateUuid.value === nullUuid.value) {
      validateResult.value = needValidateUuid.value + ' is null uuid';
      return;
    }
    let result = uuidValidate(needValidateUuid.value);
    if (result) {
      let version = uuidVersion(needValidateUuid.value);
      validateResult.value = needValidateUuid.value + ' is uuid, version is ' + version;
    } else {
      validateResult.value = needValidateUuid.value + ' is not uuid';
    }
  }

  function copyNullUuid() {
    let input = document.createElement('textarea');
    input.style.position = 'fixed';
    input.style.opacity = String(0);
    input.value = nullUuid.value;
    document.body.appendChild(input);
    input.select();
    document.execCommand('Copy');
    document.body.removeChild(input);
  }
</script>
