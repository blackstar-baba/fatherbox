<script setup lang="ts">
import { onMounted, ref } from 'vue';

import { useVbenModal } from '@vben/common-ui';

import {
  MinusCircleOutlined,
  PlusOutlined,
  RedoOutlined,
} from '@ant-design/icons-vue';
import { Button, Menu, MenuItem, Switch, Tag } from 'ant-design-vue';

import { useVbenForm } from '#/adapter';
import {
  createAiSourceModel,
  deleteAiSourceModel,
  enableAiSourceModel,
  listAiSourceModels,
  type Model,
  type Source,
  updateAiSourceModel,
} from '#/api';

interface Props {
  source: Source;
}

const props = withDefaults(defineProps<Props>(), {});

const modelsRef = ref<Model[]>([]);

const modelRef = ref<Model>();

function onSubmit(values: Record<string, any>) {
  if (values.id) {
    updateAiSourceModel({
      id: values.id,
      name: values.name,
    }).then(async () => {
      modelsRef.value = await listAiSourceModels({
        sourceId: props.source.id,
      });
    });
  } else {
    createAiSourceModel({
      name: values.name,
      sourceId: props.source.id,
    }).then(async () => {
      modelsRef.value = await listAiSourceModels({
        sourceId: props.source.id,
      });
    });
  }
}

const schema = [
  {
    component: 'Input',
    controlClass: 'hidden',
    fieldName: 'id',
    label: 'Id',
    hideLabel: true,
  },
  {
    component: 'Input',
    componentProps: {
      placeholder: 'please input name',
    },
    fieldName: 'name',
    label: 'Name',
    rules: 'required',
  },
];

const [Form, formApi] = useVbenForm({
  handleSubmit: onSubmit,
  schema,
  showDefaultActions: false,
});

const [Modal, modalApi] = useVbenModal({
  onCancel() {
    modalApi.close();
  },
  onConfirm: async () => {
    await formApi.validateAndSubmitForm();
    modalApi.close();
  },
});

const [DeleteModal, deleteModalApi] = useVbenModal({
  onCancel() {
    deleteModalApi.close();
  },
  onConfirm: async () => {
    const id = modelRef.value?.id;
    if (id) {
      // todo 后台未删掉
      deleteAiSourceModel({ id }).then(async () => {
        modelsRef.value = await listAiSourceModels({
          sourceId: props.source.id,
        });
        modelRef.value = null;
      });
    }
    deleteModalApi.close();
  },
});

function handleAdd() {
  modalApi.open();
}

function handleReload() {
  modalApi.open();
}

function handleDelete(item: Model) {
  modelRef.value = item;
  deleteModalApi.open();
}

async function handleEnableChange(item: Model) {
  await enableAiSourceModel({
    id: item.id,
    enable: item.enable,
  });
}

onMounted(async () => {
  await listAiSourceModels({
    sourceId: props.source.id,
  }).then((sources) => {
    modelsRef.value = sources;
  });
});
</script>
<template>
  <div>
    <span>Models</span>
    <Button class="ml-3" type="primary" @click="handleAdd">
      <template #icon>
        <PlusOutlined />
      </template>
      Add Model
    </Button>
    <Button class="ml-3" type="primary" @click="handleReload">
      <template #icon>
        <RedoOutlined />
      </template>
      Sync Model
    </Button>
  </div>
  <div class="mt-4">
    <Menu
      class="h-5/6"
      mode="inline"
      style="width: 100%; border-right-width: 0"
    >
      <MenuItem v-for="model in modelsRef" :key="model.id">
        <div class="flex justify-between">
          <span class="mr-4">{{ model.name }}</span>
          <div>
            <Switch
              v-model:checked="model.enable"
              @change="handleEnableChange(model)"
            />
            <Button class="ml-3" type="text" @click="handleDelete(model)">
              <template #icon>
                <MinusCircleOutlined />
              </template>
            </Button>
          </div>
        </div>
      </MenuItem>
    </Menu>
  </div>
  <Modal>
    <Form />
  </Modal>
  <DeleteModal>
    <div class="m-2">
      Are you sure delete model:
      <Tag class="mb-2 ml-2" color="#87d068">
        {{ modelRef?.name }}
      </Tag>
    </div>
  </DeleteModal>
</template>
