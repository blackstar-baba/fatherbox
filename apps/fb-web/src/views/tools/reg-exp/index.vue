<script lang="ts" setup>
import { ref } from 'vue';

import { Page } from '@vben/common-ui';
import { LucideInfo } from '@vben/icons';

import { CopyOutlined, ExpandOutlined } from '@ant-design/icons-vue';
import {
  Button,
  Card,
  Col,
  message,
  Row,
  Select,
  type SelectProps,
  Table,
  Textarea,
} from 'ant-design-vue';

const text1 = ref<string>('');
const text2 = ref<string>('');
const template = ref<string>('');

const templates = ref<SelectProps['options']>([
  {
    label: 'template',
    value: '',
  },
  {
    label: 'number',
    value: '[0-9]+',
  },
  {
    label: 'letter',
    value: '[a-z]+',
  },
  {
    label: 'chinese',
    value: String.raw`[\u4e00-\u9fa5]`,
  },
]);

const result = ref<any[]>();

const resultColumns = [
  {
    dataIndex: 'matched',
    key: 'matched',
    title: 'matched',
  },
  {
    dataIndex: 'index',
    key: 'index',
    title: 'index',
  },
];

const handleTemplateChange: SelectProps['onChange'] = (value) => {
  text1.value = value ? value.toString() : '';
};

function validate() {
  result.value = [];
  let exp = new RegExp(text1.value);
  // Make sure the regex has the global flag set to find all matches
  if (!exp.global) {
    exp = new RegExp(exp.source, `${exp.flags}g`);
  }
  // Use matchAll to iterate through all matches and get their indices
  for (const matchResult of text2.value.matchAll(exp)) {
    result.value.push({
      index: matchResult.index!,
      matched: matchResult[0],
    });
  }
}

function copyRegExp() {
  copy(text1.value);
}

function copyText() {
  copy(text2.value);
}

function copy(value: any) {
  const input = document.createElement('textarea');
  input.style.position = 'fixed';
  input.style.opacity = '0';
  input.value = value;
  document.body.append(input);
  input.select();
  document.execCommand('Copy');
  input.remove();
  message.success('copy success');
}
</script>
<template>
  <Page
    description="Help for Testing & Creating Regular Expression"
    title="RegExp"
  >
    <Row :gutter="16">
      <Col :span="24">
        <Card :bordered="false">
          <div class="py-2">
            <span>Regular Expression</span>
            <LucideInfo class="ml-1 inline size-4" />
            <Select
              v-model:value="template"
              :options="templates"
              class="ml-2 w-36"
              @change="handleTemplateChange"
            />
            <Button class="ml-2" @click="copyRegExp()">
              <template #icon>
                <CopyOutlined />
              </template>
              copy
            </Button>
          </div>
          <Textarea
            v-model:value="text1"
            :maxlength="1000"
            :rows="3"
            placeholder="fatherbox"
            show-count
          />
          <div class="py-2">
            <span>Text</span>
            <LucideInfo class="ml-1 inline size-4" />
            <Button class="ml-2" @click="copyText()">
              <template #icon>
                <CopyOutlined />
              </template>
              copy
            </Button>
          </div>
          <Textarea
            v-model:value="text2"
            :maxlength="1000"
            :rows="3"
            placeholder="fatherbox coming"
            show-count
          />
          <div class="py-4">
            <Button type="primary" @click="validate()">
              <template #icon>
                <ExpandOutlined />
              </template>
              validate
            </Button>
          </div>
          <Table :columns="resultColumns" :data-source="result" />
        </Card>
      </Col>
    </Row>
  </Page>
</template>
