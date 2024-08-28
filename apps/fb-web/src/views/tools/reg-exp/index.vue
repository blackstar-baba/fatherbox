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

const exp = ref<string>('');
const template = ref<string>('');
const flag = ref<string[]>(['g']);
const text = ref<string>('');
const maxFlagCount = ref(2);

const templates = ref<SelectProps['options']>([
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

const flags = ref<SelectProps['options']>([
  {
    label: 'global',
    value: 'g',
  },
  {
    label: 'ignore case',
    value: 'i',
  },
  {
    label: 'multiline',
    value: 'm',
  },
  {
    label: 'dotAll',
    value: 'd',
  },
  {
    label: 'unicode',
    value: 'u',
  },
  {
    label: 'sticky',
    value: 'y',
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

function validate() {
  result.value = [];
  if (!exp.value) {
    message.error('expression is null');
    return;
  }
  if (!text.value) {
    message.error('text is null');
    return;
  }
  let regExp = new RegExp(exp.value, flag.value.join(''));
  // Make sure the regex has the global flag set to find all matches
  if (!regExp.global) {
    regExp = new RegExp(regExp.source, `${regExp.flags}g`);
  }
  // Use matchAll to iterate through all matches and get their indices
  for (const matchResult of text.value.matchAll(regExp)) {
    result.value.push({
      index: matchResult.index!,
      matched: matchResult[0],
    });
  }
}

const handleTemplateSelect: SelectProps['onChange'] = (value) => {
  exp.value = value ? value.toString() : '';
};

function copyRegExp() {
  copy(exp.value);
}

function copyText() {
  copy(text.value);
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
              placeholder="template"
              @select="handleTemplateSelect"
            />
            <Select
              v-model:value="flag"
              :max-tag-count="maxFlagCount"
              :options="flags"
              class="ml-2 w-80"
              mode="multiple"
              placeholder="flag"
            />
            <Button class="ml-2" @click="copyRegExp()">
              <template #icon>
                <CopyOutlined />
              </template>
              copy
            </Button>
          </div>
          <Textarea
            v-model:value="exp"
            :maxlength="1000"
            :rows="3"
            placeholder="FatherBox"
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
            v-model:value="text"
            :maxlength="1000"
            :rows="3"
            placeholder="I Love FatherBox"
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
