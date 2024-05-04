<template>
  <PageWrapper
    contentBackground
    title="Encryptor"
    content="Support general encrypt & decrypt algorithm. md5/base64"
  >
    <Row :gutter="16">
      <Col :span="24">
        <Card :bordered="false">
          <Textarea v-model:value="text1" :rows="8" show-count :maxlength="8000" />
          <div class="py-4">
            <Select ref="select" v-model:value="algo" class="w-30" :options="algos" />
            <Button type="success" class="ml-2" @click="encrypt()">
              <template #icon>
                <ExpandOutlined />
              </template>
              encrypt
            </Button>
            <Button type="warning" class="ml-2" @click="decrypt()">
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
            class="bg-white"
            v-model:value="text2"
            :rows="8"
            show-count
            :maxlength="8000"
            readonly
          />
        </Card>
      </Col>
    </Row>
  </PageWrapper>
</template>
<script lang="ts" setup>
  import { ref } from 'vue';
  import { Button, Row, Col, Card, Textarea, Select, type SelectProps } from 'ant-design-vue';
  import { ExpandOutlined, CompressOutlined, CopyOutlined } from '@ant-design/icons-vue';
  import { PageWrapper } from '@/components/Page';
  // import { decrypt as aesDecrypt, encrypt as aesEncrypt } from 'crypto-js/aes';
  import MD5 from 'crypto-js/md5';
  import Base64 from 'crypto-js/enc-base64';
  import Utf8 from 'crypto-js/enc-utf8';
  import { useMessage } from '@/hooks/web/useMessage';

  const { createMessage } = useMessage();
  const { error } = createMessage;

  let text1 = ref<string>('');
  let text2 = ref<string>('');
  let algo = ref<string>('md5');

  const algos = ref<SelectProps['options']>([
    {
      value: 'md5',
      label: 'MD5',
    },
    {
      value: 'base64',
      label: 'BASE64',
    },
  ]);

  function encrypt() {
    try {
      switch (algo.value) {
        case 'md5':
          text2.value = MD5(text1.value).toString();
          break;
        case 'base64':
          let wordArray = Utf8.parse(text1.value);
          text2.value = Base64.stringify(wordArray);
          break;
        default:
        // do nothing
      }
    } catch (error2) {
      error(error2.message);
    }
  }

  function decrypt() {
    try {
      switch (algo.value) {
        case 'md5':
          error('Unsupported md5 decrypt');
          break;
        case 'base64':
          let parsedWordArray = Base64.parse(text1.value);
          text2.value = Utf8.stringify(parsedWordArray);
          break;
        default:
        // do nothing
      }
    } catch (error2) {
      error(error2.message);
    }
  }

  function copy() {
    let input = document.createElement('textarea');
    input.style.position = 'fixed';
    input.style.opacity = String(0);
    input.value = text2.value;
    document.body.appendChild(input);
    input.select();
    document.execCommand('Copy');
    document.body.removeChild(input);
  }
</script>
