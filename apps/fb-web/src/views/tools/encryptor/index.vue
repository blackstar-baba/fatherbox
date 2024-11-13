<script lang="ts" setup>
import { ref } from 'vue';

import { Page } from '@vben/common-ui';

import {
  CompressOutlined,
  CopyOutlined,
  ExpandOutlined,
} from '@ant-design/icons-vue';
import {
  Button,
  Card,
  Col,
  message,
  Row,
  Select,
  type SelectProps,
  Textarea,
} from 'ant-design-vue';
import Base64 from 'crypto-js/enc-base64';
import Utf8 from 'crypto-js/enc-utf8';
// import { decrypt as aesDecrypt, encrypt as aesEncrypt } from 'crypto-js/aes';
import MD5 from 'crypto-js/md5';

const text1 = ref<string>('');
const text2 = ref<string>('');
const algo = ref<string>('md5');

const algos = ref<SelectProps['options']>([
  {
    label: 'MD5',
    value: 'md5',
  },
  {
    label: 'BASE64',
    value: 'base64',
  },
]);

function encrypt() {
  switch (algo.value) {
    case 'base64': {
      const wordArray = Utf8.parse(text1.value);
      text2.value = Base64.stringify(wordArray);
      break;
    }
    case 'md5': {
      text2.value = MD5(text1.value).toString();
      break;
    }
    default:
    // do nothing
  }
}

function decrypt() {
  switch (algo.value) {
    case 'base64': {
      const parsedWordArray = Base64.parse(text1.value);
      text2.value = Utf8.stringify(parsedWordArray);
      break;
    }
    case 'md5': {
      message.warning('Unsupported md5 decrypt');
      break;
    }
    default:
    // do nothing
  }
}

function copy() {
  const input = document.createElement('textarea');
  input.style.position = 'fixed';
  input.style.opacity = String(0);
  input.value = text2.value;
  document.body.append(input);
  input.select();
  document.execCommand('Copy');
  input.remove();
  message.success('copy success');
}
</script>
<template>
  <Page
    description="Support general encrypt & decrypt algorithm. md5/base64"
    title="Encryptor"
  >
    <Row :gutter="16">
      <Col :span="24">
        <Card :bordered="false">
          <Textarea
            v-model:value="text1"
            :maxlength="8000"
            :rows="8"
            show-count
          />
          <div class="py-4">
            <Select v-model:value="algo" :options="algos" class="w-36" />
            <Button class="ml-2" type="primary" @click="encrypt()">
              <template #icon>
                <ExpandOutlined />
              </template>
              encrypt
            </Button>
            <Button class="ml-2" danger type="primary" @click="decrypt()">
              <template #icon>
                <CompressOutlined />
              </template>
              decrypt
            </Button>
            <Button class="ml-2" @click="copy()">
              <template #icon>
                <CopyOutlined />
              </template>
              copy
            </Button>
          </div>
          <Textarea
            v-model:value="text2"
            :maxlength="8000"
            :rows="8"
            class="bg-white"
            readonly
            show-count
          />
        </Card>
      </Col>
    </Row>
  </Page>
</template>
