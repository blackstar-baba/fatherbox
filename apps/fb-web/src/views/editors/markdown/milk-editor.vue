<script lang="ts" setup>
import { Crepe } from '@milkdown/crepe';

import '@milkdown/crepe/theme/common/style.css';
import '@milkdown/crepe/theme/frame.css'; // 使用 frame 主题，你可以替换为其他主题

import { onBeforeUnmount, onMounted, ref } from 'vue';

const crepeRef = ref<Crepe>(); // 保存 Crepe 实例

onMounted(() => {
  // 创建 Crepe 实例
  const crepe = new Crepe({
    defaultValue: 'Hello, Milkdown!',
    root: document.querySelector('#editor'),
  });

  crepe.create().then(() => {
    // console.log('Editor created');
  });
  crepeRef.value = crepe;
});

onBeforeUnmount(() => {
  // 销毁编辑器
  if (crepeRef.value) {
    crepeRef.value.destroy();
  }
});
</script>

<template>
  <div id="editor"></div>
</template>
