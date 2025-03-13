<script setup lang="ts">
import { onMounted, ref } from 'vue';

import { useVbenModal } from '@vben/common-ui';

import {
  EditOutlined,
  EyeInvisibleOutlined,
  EyeOutlined,
  MinusCircleOutlined,
  PlusOutlined,
} from '@ant-design/icons-vue';
import {
  Button,
  Divider,
  InputPassword,
  Menu,
  MenuItem,
  Space,
  Switch,
  Tag,
} from 'ant-design-vue';

import { useVbenForm } from '#/adapter';
import {
  createAiSource,
  deleteAiSource,
  enableAiSource,
  listAiSource,
  type Source,
  updateAiSource,
} from '#/api';

import SettingModel from './setting-model.vue';

const chatSourcesRef = ref<Source[]>([]);

const selectSourceKeysRef = ref<string[]>([]);

const sourceRef = ref<Source>();

const keyVisibleRef = ref<boolean>(false);

function onSubmit(values: Record<string, any>) {
  if (values.id) {
    updateAiSource({
      id: values.id,
      name: values.name,
      url: values.url,
      key: values.key,
    }).then(async () => {
      chatSourcesRef.value = await listAiSource();
      if (sourceRef.value) {
        sourceRef.value.name = values.name;
        sourceRef.value.url = values.url;
        sourceRef.value.key = values.key;
      }
    });
  } else {
    createAiSource({
      name: values.name,
      url: values.url,
      key: values.key,
    }).then(async () => {
      chatSourcesRef.value = await listAiSource();
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
  {
    component: 'Input',
    componentProps: {
      placeholder: 'please input url',
    },
    fieldName: 'url',
    label: 'Url',
    rules: 'required',
  },
  {
    component: 'Input',
    componentProps: {
      placeholder: 'please input key',
    },
    fieldName: 'key',
    label: 'Key',
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
    const id = sourceRef.value?.id;
    if (id) {
      deleteAiSource({ id }).then(async () => {
        chatSourcesRef.value = await listAiSource();
        if (chatSourcesRef.value.length > 0 && chatSourcesRef.value[0]) {
          selectSourceKeysRef.value = [chatSourcesRef.value[0].id];
          sourceRef.value = chatSourcesRef.value[0];
        } else {
          sourceRef.value = undefined;
        }
      });
    }
    deleteModalApi.close();
  },
});

function handleAdd() {
  modalApi.open();
}

function handleSelect(item: Source) {
  sourceRef.value = item;
}

function handleEdit() {
  formApi.setValues({
    ...sourceRef.value,
  });
  modalApi.open();
}

async function handleEnableChange(item: Source) {
  await enableAiSource({
    id: item.id,
    enable: item.enable,
  });
}

function handleDelete() {
  deleteModalApi.open();
}

onMounted(async () => {
  chatSourcesRef.value = await listAiSource();
  if (chatSourcesRef.value && chatSourcesRef.value[0]) {
    selectSourceKeysRef.value = [chatSourcesRef.value[0].id];
    sourceRef.value = chatSourcesRef.value[0];
  }
});
</script>

<template>
  <div class="flex h-full flex-row">
    <div class="h-full w-[256px] border-r pr-4">
      <div class="py-4">
        <Button class="w-full" type="primary" @click="handleAdd">
          <template #icon>
            <PlusOutlined />
          </template>
          Add Source
        </Button>
      </div>
      <div class="h-5/6">
        <Menu
          v-model:selected-keys="selectSourceKeysRef"
          class="h-5/6"
          mode="inline"
          style="width: 100%; border-right-width: 0"
        >
          <MenuItem
            v-for="source in chatSourcesRef"
            :key="source.id"
            :title="source.name"
            @click="handleSelect(source)"
          >
            <div class="flex justify-between">
              <span>{{ source.name }}</span>
              <span>
                <Tag v-if="source.enable" color="green">On</Tag>
              </span>
            </div>
          </MenuItem>
        </Menu>
      </div>
    </div>
    <div v-if="sourceRef" class="ml-6 flex w-full flex-col">
      <div class="flex justify-between">
        <div>
          <span class="text-lg leading-[2em]">{{ sourceRef.name }}</span>
          <span v-if="sourceRef.buildin" class="ml-2">
            <Tag color="green">BuildIn</Tag>
          </span>
        </div>
        <div>
          <Space>
            <Switch
              v-model:checked="sourceRef.enable"
              @change="handleEnableChange(sourceRef)"
            />
            <Button class="ml-3" type="text" @click="handleEdit()">
              <template #icon>
                <EditOutlined />
              </template>
            </Button>
            <Button class="ml-3" type="text" @click="handleDelete()">
              <template #icon>
                <MinusCircleOutlined />
              </template>
            </Button>
          </Space>
        </div>
      </div>
      <Divider />
      <div class="flex justify-stretch">
        <Space>
          <div>
            <span>Url</span>
            <span class="ml-8">
              <a :href="sourceRef.url" class="vben-link" target="_blank">
                {{ sourceRef.url }}
              </a>
            </span>
          </div>
          <div>
            <span class="ml-8">Key</span>
            <span class="ml-8">
              <Space>
                <InputPassword
                  v-model:value="sourceRef.key"
                  v-model:visible="keyVisibleRef"
                  :visibility-toggle="false"
                  class="w-[250px]"
                  disabled
                />
                <Button @click="keyVisibleRef = !keyVisibleRef">
                  <template #icon>
                    <EyeInvisibleOutlined v-if="keyVisibleRef" />
                    <EyeOutlined v-if="!keyVisibleRef" />
                  </template>
                </Button>
              </Space>
            </span>
          </div>
        </Space>
      </div>
      <Divider />
      <div class="mt-3">
        <SettingModel :source="sourceRef" />
      </div>
    </div>
    <Modal>
      <Form />
    </Modal>
    <DeleteModal>
      <div class="m-2">
        Are you sure delete source:
        <Tag class="mb-2 ml-2" color="#87d068">
          {{ sourceRef?.name }}
        </Tag>
      </div>
    </DeleteModal>
  </div>
</template>
