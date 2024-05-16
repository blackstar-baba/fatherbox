<template>
  <PageWrapper
    contentBackground
    title="Ollama"
    content="Get up and running with large language models."
  >
    <div class="p-4">
      choose model: <Select ref="select" v-model:value="model" class="w-50" :options="models" />
    </div>
  </PageWrapper>
</template>
<script lang="ts" setup>
  import { PageWrapper } from '@/components/Page';
  import { invoke } from '@tauri-apps/api/tauri';
  import { Response, ModelItems } from '@/api/ollama/model/model';
  import { ref } from 'vue';
  import { Select, type SelectProps } from 'ant-design-vue';

  let model = ref('');
  const models = ref<SelectProps['options']>([]);

  if (window.__TAURI__) {
    invoke('ollama_get_models').then((message) => {
      console.info(message);
      let response: Response<ModelItems> = message;
      let options: SelectProps['options'] = [];
      for (let modelItem of response.result.models) {
        options.push({
          value: modelItem.model,
          label: modelItem.name,
        });
      }
      models.value = options;
    });
  } else {
    // todo
  }
</script>
