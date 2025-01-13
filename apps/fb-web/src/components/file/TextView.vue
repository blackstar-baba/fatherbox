<script setup lang="ts">
import { Textarea } from 'ant-design-vue';

interface Props {
  content: string;
}

const props = withDefaults(defineProps<Props>(), {});
</script>
<template>
  <Textarea
    :maxlength="20000"
    :rows="17"
    :value="props.content"
    disabled
    placeholder="a simple text editor"
    show-count
  />
</template>
