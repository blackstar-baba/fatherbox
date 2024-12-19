<script lang="ts" setup>
import { ref, watch } from 'vue';

import { FolderOpenOutlined } from '@ant-design/icons-vue';
import { open } from '@tauri-apps/api/dialog';
import { Button, List, ListItem, message } from 'ant-design-vue';

defineOptions({ name: 'LocalUpload' });

const emit = defineEmits(['change', 'delete']);

const modelValue = defineModel<string>();

const files = ref<string[]>([]);

watch(
  () => modelValue.value,
  (value) => {
    files.value = [];
    if (value) {
      files.value.push(value);
    }
  },
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
    modelValue.value = files.value[0];
    emit('change', files.value[0]);
  }
}

function deleteItem(item: string) {
  files.value = [];
  modelValue.value = '';
  emit('delete', item);
}
</script>
<template>
  <div class="w-full">
    <Button class="mb-2" size="small" type="primary" @click="handleChooseFile">
      <template #icon>
        <FolderOpenOutlined />
      </template>
      Choose File
    </Button>
    <List
      v-if="files.length > 0"
      :data-source="files"
      bordered
      class="w-full"
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
