<script lang="ts" setup>
import { ref, watch } from 'vue';

import { Page } from '@vben/common-ui';

import {
  CompressOutlined,
  CopyOutlined,
  DoubleRightOutlined,
  ExpandOutlined,
} from '@ant-design/icons-vue';
import { isObject } from '@vueuse/core';
import {
  Button,
  Card,
  Col,
  InputSearch,
  message,
  Row,
  Textarea,
  Tree,
} from 'ant-design-vue';

import FileOperate from '#/components/file/file-operate.vue';

interface TreeObject {
  title: string;
  key: string;
  isIndex?: boolean;
  children?: TreeObject[]; // 可选属性
}

const defaultJsonObject = {
  sites: [
    {
      name: 'blackstar',
      url: 'www.blackstar.com',
    },
    {
      name: 'google',
      url: 'www.google.com',
    },
    {
      name: 'weibo',
      url: 'www.weibo.com',
    },
  ],
};
const jsonText = ref(JSON.stringify(defaultJsonObject, null, '  '));
const expandedKeys = ref<(number | string)[]>([]);
const searchValue = ref<string>('');
const autoExpandParent = ref<boolean>(true);
// let jsonTree = ref(jsonTextToTree(jsonText.value));
const jsonTree = ref(jsonTextToTree('', defaultJsonObject));

const onExpand = (keys: any) => {
  expandedKeys.value = keys;
  autoExpandParent.value = false;
};

watch(searchValue, (value) => {
  const pKeys: string[] = [];
  collectParentKey(pKeys, '', value, jsonTree.value);
  expandedKeys.value = pKeys;
  searchValue.value = value;
  autoExpandParent.value = true;
});

function collectParentKey(
  pKeys: String[],
  pKey: string,
  value: string,
  children: any,
) {
  for (const item of children) {
    if (item.title.includes(value) && !item.isIndex) {
      pKeys.push(pKey);
    }
    if (item.children !== undefined) {
      collectParentKey(pKeys, item.key, value, item.children);
    }
  }
}

function jsonTextToTree(pkey: string, jsonObject: any): TreeObject[] {
  const arr: TreeObject[] = [];
  for (const key in jsonObject) {
    const curKey = pkey + key;
    const treeObject: TreeObject = {
      key: curKey,
      title: key,
    };
    const value = jsonObject[key];
    if (isObject(value)) {
      treeObject.children = jsonTextToTree(`${curKey} `, value);
    } else if (Array.isArray(value)) {
      treeObject.children = [];
      for (const index in value) {
        const newTreeObject: TreeObject = {
          isIndex: true,
          key: `${curKey} ${index}`,
          title: index,
          children: jsonTextToTree(`${curKey} ${index} `, value[index]),
        };
        treeObject.children.push(newTreeObject);
      }
    } else if (value !== undefined && value !== null) {
      treeObject.children = [
        {
          key: `${curKey} ${value}`,
          title: value.toString(),
        },
      ];
    }
    arr.push(treeObject);
  }
  return arr;
}

function format() {
  try {
    const jsonObject = JSON.parse(jsonText.value);
    jsonText.value = JSON.stringify(jsonObject, null, '  ');
  } catch (error_) {
    const error = error_ as Error;
    message.error(error.message);
  }
}

function compress() {
  try {
    const jsonObject = JSON.parse(jsonText.value);
    jsonText.value = JSON.stringify(jsonObject, null, '');
  } catch (error_) {
    const error = error_ as Error;
    message.error(error.message);
  }
}

function copy() {
  const input = document.createElement('textarea');
  input.style.position = 'fixed';
  input.style.opacity = String(0);
  input.value = jsonText.value;
  document.body.append(input);
  input.select();
  document.execCommand('Copy');
  input.remove();
  message.success('copy success');
}

function textToTree() {
  try {
    const jsonObject = JSON.parse(jsonText.value);
    jsonTree.value = jsonTextToTree('', jsonObject);
  } catch (error_) {
    const error = error_ as Error;
    message.error(error.message);
  }
}

function setContent(content: string) {
  jsonText.value = content;
}
</script>
<template>
  <Page
    description="Support JSON formatting and compression, and simultaneously provide a tree-like way to display the structure."
    title="Json Formatter"
  >
    <Row :gutter="16">
      <Col :span="10">
        <FileOperate :content="jsonText" @send-content="setContent" />
        <Card :bordered="false">
          <div class="py-4">
            <Button type="primary" @click="format">
              <template #icon>
                <ExpandOutlined />
              </template>
              Format
            </Button>
            <Button class="ml-2" danger type="primary" @click="compress">
              <template #icon>
                <CompressOutlined />
              </template>
              Compress
            </Button>
            <Button class="ml-2" @click="copy">
              <template #icon>
                <CopyOutlined />
              </template>
              Copy
            </Button>
          </div>
          <Textarea
            v-model:value="jsonText"
            :maxlength="8000"
            :rows="17"
            show-count
          />
        </Card>
      </Col>
      <Col :span="1">
        <div class="pt-60">
          <Button @click="textToTree">
            <template #icon>
              <DoubleRightOutlined />
            </template>
          </Button>
        </div>
      </Col>
      <Col :span="13">
        <Card :bordered="false">
          <div class="w-80 py-4">
            <InputSearch
              v-model:value="searchValue"
              class="mb-8"
              placeholder="Search"
            />
          </div>
          <div class="h-96 overflow-y-auto pt-6">
            <Tree
              :auto-expand-parent="autoExpandParent"
              :expanded-keys="expandedKeys"
              :tree-data="jsonTree"
              show-line
              @expand="onExpand"
            >
              <template #title="{ title }">
                <span v-if="title.indexOf(searchValue) > -1">
                  {{ title.substr(0, title.indexOf(searchValue)) }}
                  <span style="color: #f50">{{ searchValue }}</span>
                  {{
                    title.substr(
                      title.indexOf(searchValue) + searchValue.length,
                    )
                  }}
                </span>
                <span v-else>{{ title }}</span>
              </template>
            </Tree>
          </div>
        </Card>
      </Col>
    </Row>
  </Page>
</template>
