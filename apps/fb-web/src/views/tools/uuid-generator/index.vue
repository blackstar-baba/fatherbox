<script lang="ts" setup>
import type { SelectProps } from 'ant-design-vue';

import { ref } from 'vue';

import { Page } from '@vben/common-ui';

import {
  CheckOutlined,
  CopyOutlined,
  DownloadOutlined,
  EnterOutlined,
} from '@ant-design/icons-vue';
import {
  Button,
  Card,
  Col,
  Divider,
  Input,
  List,
  ListItem,
  Row,
  Select,
} from 'ant-design-vue';
import {
  NIL as NIL_UUID,
  v1 as uuidv1,
  v4 as uuidv4,
  validate as uuidValidate,
  version as uuidVersion,
} from 'uuid';

import { downloadByData } from '#/utils/file/downloadUtil';

const versions = ref<SelectProps['options']>([
  {
    label: 'Version 1(Time Based)',
    value: '1',
  },
  {
    label: 'Version 4(Random)',
    value: '4',
  },
]);
const numbers = ref<SelectProps['options']>([
  {
    label: '1',
    value: '1',
  },
  {
    label: '2',
    value: '2',
  },
  {
    label: '5',
    value: '5',
  },
  {
    label: '10',
    value: '10',
  },
  {
    label: '20',
    value: '20',
  },
  {
    label: '50',
    value: '50',
  },
  {
    label: '100',
    value: '100',
  },
]);

const version = ref('4');
const number = ref('5');
const uuids = ref<String[]>([]);
const nullUuid = ref(NIL_UUID);
const needValidateUuid = ref(NIL_UUID);
const validateResult = ref('');

function generate() {
  const num = Number.parseInt(number.value);
  uuids.value = [];
  for (let i = 0; i < num; i++) {
    switch (version.value) {
      case '1': {
        uuids.value.push(uuidv1());
        break;
      }
      case '4': {
        uuids.value.push(uuidv4());
        break;
      }
    }
  }
}

generate();

function download() {
  let data = '';
  for (let i = 0; i < uuids.value.length; i++) {
    data += `${uuids.value[i]}\n`;
  }
  downloadByData(data, 'uuids.txt');
}

function validate() {
  if (needValidateUuid.value === nullUuid.value) {
    validateResult.value = `${needValidateUuid.value} is null uuid`;
    return;
  }
  const result = uuidValidate(needValidateUuid.value);
  if (result) {
    const version = uuidVersion(needValidateUuid.value);
    validateResult.value = `${needValidateUuid.value} is uuid, version is ${version}`;
  } else {
    validateResult.value = `${needValidateUuid.value} is not uuid`;
  }
}

function copyNullUuid() {
  const input = document.createElement('textarea');
  input.style.position = 'fixed';
  input.style.opacity = String(0);
  input.value = nullUuid.value;
  document.body.append(input);
  input.select();
  document.execCommand('Copy');
  input.remove();
}
</script>
<template>
  <Page
    description="Support for RFC4122 version 1, and 4 UUIDs"
    title="Uuid Generator"
  >
    <Row :gutter="16">
      <Col :span="13">
        <Card :bordered="false" class="h-120">
          <div class="py-4">
            <Select v-model:value="version" :options="versions" class="w-50" />
            <Select
              v-model:value="number"
              :options="numbers"
              class="w-18 ml-2"
            />
            <Button class="ml-2" type="primary" @click="generate">
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
            <List :data-source="uuids" item-layout="horizontal">
              <template #renderItem="{ item }">
                <ListItem>{{ item }}</ListItem>
              </template>
            </List>
          </div>
        </Card>
      </Col>
      <Col :span="11">
        <Card :bordered="false" class="h-120">
          <Divider orientation="left">Null</Divider>
          <div class="py-4">
            {{ nullUuid }}
            <Button class="ml-2" type="primary" @click="copyNullUuid">
              <template #icon>
                <CopyOutlined />
              </template>
              Copy
            </Button>
          </div>
          <Divider orientation="left">Validate</Divider>
          <div class="h-20 pt-4">
            <Input
              v-model:value="needValidateUuid"
              class="w-85"
              placeholder="6ec0bd7f-11c0-43da-975e-2a8ad9ebae0b"
            />
            <Button class="ml-2" type="primary" @click="validate">
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
  </Page>
</template>
