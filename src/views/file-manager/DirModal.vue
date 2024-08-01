<template>
  <BasicModal v-bind="$attrs" @register="registerModal" :title="getTitle" @ok="handleSubmit">
    <BasicForm @register="registerForm" />
  </BasicModal>
</template>
<script lang="ts" setup>
  import { ref, computed, unref } from 'vue';
  import { BasicModal, useModalInner } from '@/components/Modal';
  import { BasicForm, useForm } from '@/components/Form';
  import {
    getDirs,
    createDir,
    updateFile,
    dirFormSchema,
    DIR_TYPE,
  } from '@/views/file-manager/file.data';

  defineOptions({ name: 'DirModal' });

  const emit = defineEmits(['success', 'register']);

  const isUpdate = ref(true);
  const rowId = ref('');

  const [registerForm, { setFieldsValue, updateSchema, resetFields, validate }] = useForm({
    labelWidth: 100,
    baseColProps: { span: 24 },
    schemas: dirFormSchema,
    showActionButtonGroup: false,
    actionColOptions: {
      span: 23,
    },
  });

  const [registerModal, { setModalProps, closeModal }] = useModalInner(async (data) => {
    await resetFields();
    setModalProps({ confirmLoading: false });
    isUpdate.value = !!data?.isUpdate;

    if (unref(isUpdate)) {
      rowId.value = data.record.id;
    }

    await setFieldsValue({
      ...data.record,
    });

    const treeData = await getDirs();
    await updateSchema([
      {
        field: 'pid',
        componentProps: { treeData },
        // dynamicDisabled: () => {
        //   return !isUpdate.value;
        // },
      },
    ]);
  });

  const getTitle = computed(() => (!unref(isUpdate) ? 'Add Directory' : 'Edit Directory'));

  async function handleSubmit() {
    try {
      const values = await validate();
      setModalProps({ confirmLoading: true });
      console.info(values);
      if (unref(isUpdate)) {
        await updateFile(rowId.value, values.pid, DIR_TYPE, values.name);
      } else {
        await createDir(values.pid, DIR_TYPE, values.name);
      }
      closeModal();
      emit('success', { isUpdate: unref(isUpdate), values: { ...values } });
    } finally {
      setModalProps({ confirmLoading: false });
    }
  }
</script>
