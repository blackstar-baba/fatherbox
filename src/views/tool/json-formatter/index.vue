<template>
  <PageWrapper
    contentBackground
    title="Json Formatter"
    content="Support JSON formatting and compression, and simultaneously provide a tree-like way to display the structure."
  >
    <Row :gutter="16">
      <Col :span="10">
        <Card :bordered="false">
          <div class="py-4">
            <Button type="success" @click="format">
              <template #icon>
                <ExpandOutlined />
              </template>
              Format
            </Button>
            <Button type="warning" class="ml-2" @click="compress">
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
          <Textarea v-model:value="jsonText" :rows="17" show-count :maxlength="8000" />
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
          <div class="py-4 w-80">
            <InputSearch v-model:value="searchValue" class="mb-8" placeholder="Search" />
          </div>
          <div class="pt-6 h-88 overflow-y-auto">
            <Tree
              :expanded-keys="expandedKeys"
              :auto-expand-parent="autoExpandParent"
              show-line
              :tree-data="jsonTree"
              @expand="onExpand"
            >
              <template #title="{ title }">
                <span v-if="title.indexOf(searchValue) > -1">
                  {{ title.substr(0, title.indexOf(searchValue)) }}
                  <span style="color: #f50">{{ searchValue }}</span>
                  {{ title.substr(title.indexOf(searchValue) + searchValue.length) }}
                </span>
                <span v-else>{{ title }}</span>
              </template>
            </Tree>
          </div>
        </Card>
      </Col>
    </Row>
  </PageWrapper>
</template>
<script lang="ts" setup>
  import { ref, watch } from 'vue';
  import { Button, Row, Col, Card, Textarea, InputSearch, Tree } from 'ant-design-vue';
  import {
    ExpandOutlined,
    CompressOutlined,
    CopyOutlined,
    DoubleRightOutlined,
  } from '@ant-design/icons-vue';
  import { PageWrapper } from '@/components/Page';
  import { isArray, isObject } from '@/utils/is';
  import { useMessage } from '@/hooks/web/useMessage';

  interface TreeObject {
    title: string;
    key: string;
    isIndex?: boolean;
    children?: TreeObject[]; // 可选属性
  }

  const { createMessage } = useMessage();

  const { error } = createMessage;

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
  let jsonText = ref(JSON.stringify(defaultJsonObject, null, '  '));
  const expandedKeys = ref<(string | number)[]>([]);
  const searchValue = ref<string>('');
  const autoExpandParent = ref<boolean>(true);
  // let jsonTree = ref(jsonTextToTree(jsonText.value));
  let jsonTree = ref(jsonTextToTree('', defaultJsonObject));

  const onExpand = (keys: string[]) => {
    expandedKeys.value = keys;
    autoExpandParent.value = false;
  };

  watch(searchValue, (value) => {
    let pKeys = [];
    collectParentKey(pKeys, '', value, jsonTree.value);
    expandedKeys.value = pKeys;
    searchValue.value = value;
    autoExpandParent.value = true;
  });

  function collectParentKey(pKeys: String[], pKey: string, value: string, children: any) {
    for (let item of children) {
      if (item.title.indexOf(value) > -1) {
        if (!item.isIndex) {
          pKeys.push(pKey);
        }
      }
      if (item.children !== undefined) {
        collectParentKey(pKeys, item.key, value, item.children);
      }
    }
  }

  function jsonTextToTree(pkey, jsonObject): TreeObject[] {
    let arr: TreeObject[] = [];
    for (let key in jsonObject) {
      let curKey = pkey + key;
      let treeObject: TreeObject = {
        title: key,
        key: curKey,
      };
      let value = jsonObject[key];
      if (isObject(value)) {
        treeObject.children = jsonTextToTree(curKey + ' ', value);
      } else if (isArray(value)) {
        treeObject.children = [];
        for (let index in value) {
          let newTreeObject: TreeObject = {
            title: index,
            key: curKey + ' ' + index,
            isIndex: true,
            children: jsonTextToTree(curKey + ' ' + index + ' ', value[index]),
          };
          treeObject.children.push(newTreeObject);
        }
      } else if (value !== undefined && value !== null) {
        treeObject.children = [
          {
            title: value.toString(),
            key: curKey + ' ' + value,
          },
        ];
      }
      arr.push(treeObject);
    }
    return arr;
  }

  function format() {
    try {
      let jsonObject = JSON.parse(jsonText.value);
      jsonText.value = JSON.stringify(jsonObject, null, '  ');
    } catch (error2) {
      error(error2.message);
    }
  }

  function compress() {
    try {
      let jsonObject = JSON.parse(jsonText.value);
      jsonText.value = JSON.stringify(jsonObject, null, '');
    } catch (error2) {
      error(error2.message);
    }
  }

  function copy() {
    let input = document.createElement('textarea');
    input.style.position = 'fixed';
    input.style.opacity = String(0);
    input.value = jsonText.value;
    document.body.appendChild(input);
    input.select();
    document.execCommand('Copy');
    document.body.removeChild(input);
  }

  function textToTree() {
    try {
      let jsonObject = JSON.parse(jsonText.value);
      jsonTree.value = jsonTextToTree('', jsonObject);
    } catch (error2) {
      error(error2.message);
    }
  }
</script>
