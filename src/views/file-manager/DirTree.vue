<template>
  <div class="m-4 mr-0 overflow-hidden bg-white">
    <BasicTree
      title="Directory"
      toolbar
      search
      treeWrapperClassName="h-[calc(100%-35px)] overflow-auto"
      :clickRowToExpand="false"
      :treeData="treeData"
      :rightMenuList="menuList"
      :beforeRightClick="rightClick"
      @select="handleSelect"
    />
    <DirModal @register="registerModal" @success="handleSuccess" />
  </div>
</template>
<script lang="ts" setup>
  import { onMounted, ref } from 'vue';
  import { BasicTree, TreeItem } from '@/components/Tree';
  import { deleteFile, getDirs } from '@/views/file-manager/file.data';
  import { useModal } from '@/components/Modal';
  import DirModal from '@/views/file-manager/DirModal.vue';

  defineOptions({ name: 'DirTree' });

  const [registerModal, { openModal }] = useModal();

  const emit = defineEmits(['select']);

  const treeData = ref<TreeItem[]>([]);

  let rightClickItem: any = undefined;

  const menuList = [
    {
      label: 'create',
      divider: true,
      handler: handleCreate,
    },
    {
      label: 'edit',
      handler: handleUpdate,
    },
    {
      label: 'delete',
      divider: true,
      handler: handleDelete,
    },
  ];

  async function fetch() {
    treeData.value = await getDirs();
  }

  async function rightClick(node: any, event: any) {
    console.info(event);
    rightClickItem = node.dataRef;
    // if (data.path === '') {
    //   return [menuList[0]];
    // }
    return menuList;
  }

  function handleSelect(keys) {
    emit('select', keys[0]);
  }

  function handleCreate() {
    // if (rightClickItem) {
    //   console.info(rightClickItem);
    // }
    // rightClickItem = undefined;
    openModal(true, {
      isUpdate: false,
      record: {
        pid: rightClickItem.key,
      },
    });
  }

  function handleUpdate() {
    openModal(true, {
      // todo how to get pid
      isUpdate: true,
      record: {
        id: rightClickItem.key,
        name: getTitle(treeData.value, rightClickItem.key),
        pid: rightClickItem.pkey,
      },
    });
  }

  async function handleDelete() {
    if (rightClickItem) {
      let id = rightClickItem.key;
      if (id !== '') {
        await deleteFile(id);
        await fetch();
      }
    }
    rightClickItem = undefined;
  }

  function getTitle(treeItems: TreeItem[], key: string) {
    for (let i = 0; i < treeItems.length; i++) {
      let treeItem = treeItems[i];
      if (treeItem.key === key) {
        return treeItem.title;
      } else if (treeItem.children && treeItem.children.length > 0) {
        let title = getTitle(treeItem.children, key);
        if (title !== '') {
          return title;
        }
      }
    }
    return '';
  }

  function handleSuccess() {
    fetch();
  }

  onMounted(() => {
    fetch();
  });
</script>
