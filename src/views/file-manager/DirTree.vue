<template>
  <div class="m-4 mr-0 overflow-hidden bg-white">
    <BasicTree
      title="Directory"
      toolbar
      search
      treeWrapperClassName="h-[calc(100%-35px)] overflow-auto"
      :clickRowToExpand="false"
      :treeData="treeData"
      :fieldNames="{ key: 'path', title: 'name' }"
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
  import { deleteFile, getDirs, workspace } from '@/views/file-manager/file.data';
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
      handler: function () {
        if (rightClickItem) {
          console.info(rightClickItem);
        }
        rightClickItem = undefined;
      },
    },
    {
      label: 'delete',
      divider: true,
      handler: handleDelete,
    },
  ];

  async function fetch() {
    treeData.value = (await getDirs()) as unknown as TreeItem[];
  }

  async function rightClick(node: any, event: any) {
    console.info(event);
    const data = node.dataRef;
    rightClickItem = data;
    if (data.path === '') {
      return [menuList[0]];
    }
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
      parentPath: rightClickItem.path,
    });
  }
  async function handleDelete() {
    if (rightClickItem) {
      let fileName = getTitle(treeData.value, rightClickItem.path);
      if (fileName !== '') {
        await deleteFile(workspace, fileName, rightClickItem.parentPath);
        await fetch();
      }
    }
    rightClickItem = undefined;
  }

  function getTitle(treeItems: TreeItem[], path: string) {
    for (let i = 0; i < treeItems.length; i++) {
      let treeItem = treeItems[i];
      if (treeItem.path == path) {
        return treeItem.name;
      } else if (treeItem.children && treeItem.children.length > 0) {
        let title = getTitle(treeItem.children, path);
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
