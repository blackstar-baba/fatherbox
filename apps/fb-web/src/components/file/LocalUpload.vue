<script lang="ts" setup>
import { type PropType, ref, watch } from 'vue';

import { FolderOpenOutlined } from '@ant-design/icons-vue';
import { open } from '@tauri-apps/api/dialog';
import { Button, List, ListItem, message } from 'ant-design-vue';

defineOptions({ name: 'LocalUpload' });

const props = defineProps({
  value: {
    type: String as PropType<string>,
    default: () => '',
  },
});

const emit = defineEmits(['change', 'delete']);

const files = ref<string[]>([]);

watch(
  () => props.value,
  (value) => {
    if (value) {
      files.value = [];
      files.value.push(value);
    }
  },
  { immediate: true },
);

async function handleChooseFile() {
  const selected = await open({
    directory: false,
    multiple: false,
  });
  message.info(selected);
  if (Array.isArray(selected)) {
    message.warn('can choose one file only');
    // user selected multiple files
  } else if (selected === null) {
    // user cancelled the selection
  } else {
    if (files.value.length === 1) {
      message.warn('exist file');
      return;
    }
    files.value.push(selected);
    // emit('change', files.value[0]);
  }
}

function deleteItem(item: string) {
  files.value = [];
  emit('delete', item);
}
</script>
<template>
  <div>
    <Button class="mb-2" size="small" type="primary" @click="handleChooseFile">
      <template #icon>
        <FolderOpenOutlined />
      </template>
      Choose File
    </Button>
    <List
      :data-source="files"
      bordered
      class="w-3/4"
      item-layout="horizontal"
      size="small"
    >
      <template #renderItem="{ item }">
        <ListItem>
          <div class="w-1/2">{{ item }}</div>
          <template #actions>
            <a @click="deleteItem(item)">delete</a>
          </template>
        </ListItem>
      </template>
    </List>
  </div>
</template>
