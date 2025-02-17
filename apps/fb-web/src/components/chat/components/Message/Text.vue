<script lang="ts" setup>
import { computed, onMounted, onUnmounted, onUpdated, ref } from 'vue';

import { useVbenModal } from '@vben/common-ui';

import MdKatex from '@vscode/markdown-it-katex';
import { message } from 'ant-design-vue';
import hljs from 'highlight.js';
import MarkdownIt from 'markdown-it';
import MdLinkAttributes from 'markdown-it-link-attributes';
import MdMermaid from 'mermaid-it-markdown';

import { useVbenForm } from '#/adapter';
import { createFile as createFileApi } from '#/api';
import {
  encodeStringToUint8Array,
  FILE_TYPE_FILE,
  getFileTree,
} from '#/components/file/file';
import { $t } from '#/locales';
import { copy } from '#/utils';

interface Props {
  inversion?: boolean;
  error?: boolean;
  text?: string;
  loading?: boolean;
  asRawText?: boolean;
}

const props = defineProps<Props>();

const textRef = ref<HTMLElement>();

const mdi = new MarkdownIt({
  html: false,
  linkify: true,
  highlight(code: any, language: any) {
    const validLang = !!(language && hljs.getLanguage(language));
    if (validLang) {
      const lang = language ?? '';
      return highlightBlock(
        hljs.highlight(code, { language: lang }).value,
        lang,
      );
    }
    return highlightBlock(hljs.highlightAuto(code).value, '');
  },
});

mdi
  .use(MdLinkAttributes, { attrs: { target: '_blank', rel: 'noopener' } })
  .use(MdKatex)
  .use(MdMermaid);

const wrapClass = computed(() => {
  return [
    'p-2',
    'text-wrap',
    'min-w-[20px]',
    'max-w-[1000px]',
    'rounded-md',
    props.inversion ? 'bg-[#f4f6f8]' : '',
    'dark:bg-[#1e1e1e]',
    props.inversion ? 'message-request' : 'message-reply',
    { 'text-red-500': props.error },
  ];
});

const text = computed(() => {
  const value = props.text ?? '';
  if (!props.asRawText) {
    // 对数学公式进行处理，自动添加 $$ 符号
    const escapedText = escapeBrackets(escapeDollarNumber(value));
    return mdi.render(escapedText);
  }
  return value;
});

function highlightBlock(str: string, lang?: string) {
  return `<pre class="code-block-wrapper"><div class="code-block-header"><span class="code-block-header__lang">${lang}</span><span><span class="code-block-header__copy">${$t('chat.message.copy')}</span><span class="code-block-header__save ml-2">${$t('chat.message.save')}</span></span></div><code class="max-w-[1000px] hljs code-block-body ${lang}">${str}</code></pre>`;
}

function onSubmit(values: Record<string, any>) {
  createFileApi({
    name: values.name,
    pid: values.pid,
    content: values.content
      ? encodeStringToUint8Array(values.content)
      : undefined,
    path: values.file ?? undefined,
    type: FILE_TYPE_FILE,
  }).then((file: any) => {
    if (file.id) {
      message.success('save file success');
    }
  });
}

const [createForm, createFormApi] = useVbenForm({
  handleSubmit: onSubmit,
  schema: [
    {
      component: 'Input',
      componentProps: {
        placeholder: '请输入',
      },
      fieldName: 'name',
      label: 'Name',
      rules: 'required',
    },
    {
      component: 'TreeSelect',
      componentProps: {
        class: 'w-full',
        allowClear: false,
        placeholder: '请选择',
        showSearch: false,
        treeData: [],
        treeNodeFilterProp: 'label',
      },
      fieldName: 'pid',
      label: 'Parent',
    },
    {
      component: 'Textarea',
      componentProps: {
        placeholder: '请输入内容',
      },
      fieldName: 'content',
      label: 'Content',
    },
  ],
  showDefaultActions: false,
});

const [CreateModal, createModalApi] = useVbenModal({
  onCancel() {
    createModalApi.close();
  },
  onConfirm: async () => {
    await createFormApi.validateAndSubmitForm();
    createModalApi.close();
  },
});

function addCopyEvents() {
  if (textRef.value) {
    const copyBtn = textRef.value.querySelectorAll('.code-block-header__copy');
    copyBtn.forEach((btn) => {
      btn.addEventListener('click', () => {
        const code =
          btn.parentElement?.parentElement?.nextElementSibling?.textContent;
        if (code) {
          copy(code);
        }
      });
    });
  }
}

function addSaveEvents() {
  if (textRef.value) {
    const saveButton = textRef.value.querySelectorAll(
      '.code-block-header__save',
    );
    saveButton.forEach((btn) => {
      btn.addEventListener('click', async () => {
        const content =
          btn.parentElement?.parentElement?.nextElementSibling?.textContent;
        if (content) {
          const fileTree = await getFileTree();
          // todo can throw error
          if (fileTree) {
            createFormApi.updateSchema([
              {
                fieldName: 'pid',
                componentProps: {
                  treeData: [fileTree],
                  disabled: false,
                },
              },
            ]);
            createFormApi.setValues({
              content,
              pid: fileTree?.value,
            });
            createModalApi.open();
          }
        }
      });
    });
  }
}

function removeCopyEvents() {
  if (textRef.value) {
    const copyBtn = textRef.value.querySelectorAll('.code-block-header__copy');
    copyBtn.forEach((btn) => {
      // eslint-disable-next-line unicorn/no-invalid-remove-event-listener
      btn.removeEventListener('click', () => {});
    });
  }
}

function removeSaveEvents() {
  if (textRef.value) {
    const saveBtn = textRef.value.querySelectorAll('.code-block-header__save');
    saveBtn.forEach((btn) => {
      // eslint-disable-next-line unicorn/no-invalid-remove-event-listener
      btn.removeEventListener('click', () => {});
    });
  }
}

function escapeDollarNumber(text: string) {
  let escapedText = '';

  for (let i = 0; i < text.length; i += 1) {
    let char = text[i];
    const nextChar = text[i + 1] || ' ';

    if (char === '$' && nextChar >= '0' && nextChar <= '9')
      char = String.raw`\$`;

    escapedText += char;
  }

  return escapedText;
}

function escapeBrackets(text: string) {
  const pattern =
    /(```[\s\S]*?```|`.*?`)|\\\[([\s\S]*?[^\\])\\\]|\\\((.*?)\\\)/g;
  return text.replaceAll(
    pattern,
    (match, codeBlock, squareBracket, roundBracket) => {
      if (codeBlock) return codeBlock;
      else if (squareBracket) return `$$${squareBracket}$$`;
      else if (roundBracket) return `$${roundBracket}$`;
      return match;
    },
  );
}

onMounted(() => {
  addCopyEvents();
  addSaveEvents();
});

onUpdated(() => {});

onUnmounted(() => {
  removeCopyEvents();
  removeSaveEvents();
});
</script>

<template>
  <div :class="wrapClass">
    <div ref="textRef" class="break-words leading-relaxed">
      <div v-if="!inversion">
        <div
          v-if="!asRawText"
          :class="{ 'markdown-body-generate': loading }"
          class="markdown-body"
          v-html="text"
        ></div>
        <div v-else class="whitespace-pre-wrap" v-text="text"></div>
      </div>
      <div v-else class="whitespace-pre-wrap" v-text="text"></div>
    </div>
  </div>
  <CreateModal>
    <createForm />
  </CreateModal>
</template>

<style lang="less">
@import url(./github-markdown.less);
@import url(./highlight.less);
@import url(./style.less);
</style>
