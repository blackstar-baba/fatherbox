<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from 'vue';

import { useI18n } from '@vben/locales';
import { usePreferences } from '@vben-core/preferences';

import Cherry from 'cherry-markdown';

import 'cherry-markdown/dist/cherry-markdown.min.css';

const props = defineProps({
  height: {
    default: 500,
    type: Number,
  },
  mdId: {
    default: 'markdown-container',
    type: String,
  },
  modelValue: {
    default: '',
    type: String,
  },
});

const cherryRef = ref<Cherry>();

const setContent = (val: string) => {
  cherryRef.value?.setMarkdown(val);
};

const getContent = () => {
  return cherryRef.value?.getMarkdown();
};

const getHtml = () => {
  return cherryRef.value?.getHtml();
};

// 图片加载回调
// const beforeImageMounted = (e, src) => {
//   return { [e]: src };
// };

defineExpose({
  getContent,
  getHtml,
  setContent,
});

const { locale } = useI18n();
watch([() => locale.value], ([locale]) => {
  if (locale === 'en-US') {
    cherryRef.value?.setLocale('en_US');
  } else {
    cherryRef.value?.setLocale('zh_CN');
  }
});

const { theme } = usePreferences();
watch([() => theme.value], ([theme]) => {
  cherryRef.value?.setTheme(theme);
});

// const fileUpload = (file, callback) => {
//   if (file.size / 1024 / 1024 > 200) {
//     return proxy.$modal.msgError('请上传200M以内的图片！');
//   }
//   if (!file.type.includes('image')) {
//     return proxy.$modal.msgError('仅支持上传图片！');
//   }
//   const formData = new FormData();
//   formData.append('file', file);
//   console.log(file, 'file');
//   loading.value = true;
//   uploadImg(props.knwlgId, formData)
//     .then((res) => {
//       loading.value = false;
//       callback(
//         `${import.meta.env.VITE_APP_BASE_API}/ekms/images/v1/preview/${
//           res.data.imgId
//         }`,
//       );
//     })
//     .catch(() => {
//       loading.value = false;
//     });
// };

const initMd = () => {
  cherryRef.value = new Cherry({
    callback: {
      // afterChange,
      // beforeImageMounted,
    },
    editor: {
      defaultModel: 'editOnly',
    },
    // fileUpload,
    engine: {
      syntax: {
        header: {
          anchorStyle: 'none',
        },
        toc: {
          showAutoNumber: true,
        },
      },
    },
    id: props.mdId,
    toolbars: {
      bubble: [
        'bold',
        'italic',
        'underline',
        'strikethrough',
        'sub',
        'sup',
        '|',
        'size',
        'color',
      ],
      customMenu: [],
      float: [
        'h1',
        'h2',
        'h3',
        '|',
        'checklist',
        'quote',
        'quickTable',
        'code',
      ],
      theme: 'light',
      toolbar: [
        'bold',
        'italic',
        'underline',
        'strikethrough',
        '|',
        'color',
        'header',
        '|',
        'list',
        'image',
        {
          insert: [
            'audio',
            'video',
            'link',
            'hr',
            'br',
            'code',
            'formula',
            'toc',
            'table',
            'line-table',
            'bar-table',
            'pdf',
            'word',
          ],
        },
        'graph',
        // 'settings',
        // "switchModel",
        'togglePreview',
      ],
    },
    value: props.modelValue,
  });
};

const destroyMd = () => {
  cherryRef.value?.destroy();
};

onMounted(() => {
  initMd();
});

onBeforeUnmount(() => {
  destroyMd();
});
</script>
<template>
  <div :id="mdId" :style="{ height: `${height}px` }"></div>
</template>
