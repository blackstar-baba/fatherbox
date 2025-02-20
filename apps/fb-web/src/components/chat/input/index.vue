<script lang="ts" setup>
import { computed, ref } from 'vue';

import { RiSendPlaneFill } from '@vben/icons';
import { $t } from '@vben/locales';

import { AutoComplete, Button, Space, Textarea } from 'ant-design-vue';

interface Props {
  loading: boolean;
}

interface Emit {
  (e: 'submit', text: string): void;
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
});

const emit = defineEmits<Emit>();

const promptTemplate = ref<
  {
    key: string;
    value: string;
  }[]
>([]);

const textRef = ref<string>('');

// 可优化部分
// 搜索选项计算，这里使用value作为索引项，所以当出现重复value时渲染异常(多项同时出现选中效果)
// 理想状态下其实应该是key作为索引项,但官方的renderOption会出现问题，所以就需要value反renderLabel实现
const searchOptions = computed(() => {
  return textRef.value.startsWith('/')
    ? promptTemplate.value
        .filter((item: { key: string }) =>
          item.key.toLowerCase().includes(textRef.value.slice(1).toLowerCase()),
        )
        .map((obj: { value: any }) => {
          return obj.value;
        })
    : [];
});

const onSelect = (value: any) => {
  for (const i of promptTemplate.value) {
    if (i.key === value) {
      textRef.value = i.value;
      break;
    }
  }
  // message.success(value);
};

const buttonDisabled = computed(() => {
  return props.loading || !textRef.value || textRef.value === '';
});

function handleEnter(event: KeyboardEvent) {
  if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault();
    handleSubmit();
  }
}

function handleSubmit() {
  emit('submit', textRef.value);
  textRef.value = '';
}
</script>

<template>
  <span>
    <Space :size="10">
      <AutoComplete
        v-model:value="textRef"
        :options="searchOptions"
        @select="onSelect"
      >
        <Textarea
          :auto-size="{ minRows: 1, maxRows: 8 }"
          :placeholder="$t('chat.message.placeholder')"
          class="min-w-[450px]"
          @press-enter="handleEnter"
        />
      </AutoComplete>
      <Button :disabled="buttonDisabled" type="primary" @click="handleSubmit">
        <RiSendPlaneFill />
      </Button>
    </Space>
  </span>
</template>
