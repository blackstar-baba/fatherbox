<template>
  <div>
    <Button class="mb-2" type="primary" @click="handleChooseFile">选择文件</Button>
    <List class="w-3/4" size="small" bordered item-layout="horizontal" :data-source="files">
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

<script lang="ts" setup>
  import { type PropType, ref, watch } from 'vue';
  import { open } from '@tauri-apps/api/dialog';
  import { Button, List, ListItem } from 'ant-design-vue';

  import { useMessage } from '@/hooks/web/useMessage';

  const { createMessage } = useMessage();
  const { warn } = createMessage;

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
    console.info(selected);
    if (Array.isArray(selected)) {
      warn('can choose one file only');
      // user selected multiple files
    } else if (selected === null) {
      // user cancelled the selection
    } else {
      if (files.value.length == 1) {
        warn('exist file');
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
