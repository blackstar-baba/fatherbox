<template>
  <PageWrapper dense contentFullHeight fixedHeight contentClass="flex">
    <DirTree class="w-5/16 xl:w-1/4" @select="handleSelect" />
    <BasicTable @register="registerTable" class="w-11/16 xl:w-3/4" :searchInfo="searchInfo">
      <template #toolbar>
        <a-button type="primary" @click="handleCreate">Create</a-button>
        <a-button type="primary" @click="handleExport">Export</a-button>
      </template>
      <template #bodyCell="{ column, record }">
        <template v-if="column.key === 'action'">
          <TableAction
            :actions="[
              {
                icon: 'clarity:note-edit-line',
                tooltip: 'Edit File',
                onClick: handleEdit.bind(null, record),
              },
              {
                icon: 'ant-design:delete-outlined',
                color: 'error',
                tooltip: 'Delete File',
                popConfirm: {
                  title: 'Confirm Delete?',
                  placement: 'left',
                  confirm: handleDelete.bind(null, record),
                },
              },
            ]"
          />
        </template>
      </template>
    </BasicTable>
    <FileModal @register="registerModal" @success="handleSuccess" />
  </PageWrapper>
</template>
<script lang="ts" setup>
  import { reactive } from 'vue';

  import { BasicTable, useTable, TableAction } from '@/components/Table';
  import { PageWrapper } from '@/components/Page';
  import DirTree from './DirTree.vue';
  import { useModal } from '@/components/Modal';
  import FileModal from './FileModal.vue';
  import { columns, deleteFile, getFiles, searchFormSchema } from '@/views/file-manager/file.data';

  defineOptions({ name: 'FileManagement' });

  // const go = useGo();
  const [registerModal, { openModal }] = useModal();
  const searchInfo = reactive<Recordable>({});
  const [registerTable, { reload, getSearchInfo }] = useTable({
    title: 'Files',
    api: getFiles,
    rowKey: 'id',
    columns,
    formConfig: {
      labelWidth: 50,
      schemas: searchFormSchema,
      autoSubmitOnEnter: true,
    },
    useSearchForm: true,
    showTableSetting: true,
    showIndexColumn: false,
    bordered: true,
    handleSearchInfoFn(info) {
      console.log('handleSearchInfoFn', info);
      return info;
    },
    actionColumn: {
      width: 120,
      title: 'Action',
      dataIndex: 'action',
      // slots: { customRender: 'action' },
    },
  });

  function handleCreate() {
    openModal(true, {
      isUpdate: false,
      record: {
        pid: searchInfo.pid,
      },
    });
  }

  function handleEdit(record: Recordable) {
    console.log(record);
    openModal(true, {
      isUpdate: true,
      record: record,
    });
  }

  async function handleDelete(record: Recordable) {
    await deleteFile(record.id);
    await reload();
  }

  function handleExport() {
    console.log(getSearchInfo());
  }

  async function handleSuccess({ _values }) {
    await reload();
  }

  async function handleSelect(pid = '') {
    searchInfo.pid = pid;
    searchInfo.name = '';
    await reload();
  }
</script>
