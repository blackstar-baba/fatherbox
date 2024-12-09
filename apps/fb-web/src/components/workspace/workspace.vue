<script setup lang="ts">
import type { WorkspaceInfo } from '#/types';

import { ref } from 'vue';

import { BiPersonWorkspace } from '@vben/icons';
import {
  type VbenDropdownMenuItem,
  VbenDropdownRadioMenu,
  VbenIconButton,
} from '@vben-core/shadcn-ui';

import { message } from 'ant-design-vue';

import { getAllWorkspaceApi } from '#/api';
import { useWorkspaceStore } from '#/store';

defineOptions({
  name: 'Workspace',
});

const workspaceStore = useWorkspaceStore();

const menusRef = ref<VbenDropdownMenuItem[]>([]);
const id = ref<string>();

getAllWorkspaceApi().then((workspaces) => {
  if (workspaces) {
    workspaceStore.setWorkspaces(workspaces);
    const id = workspaces[0]?.id;
    if (id) {
      workspaceStore.setId(id);
    }
    workspaces.forEach((workspace: WorkspaceInfo) => {
      menusRef.value.push({
        label: workspace.name,
        value: workspace.id,
      });
    });
  }
});

async function handleUpdate(id: string) {
  if (id !== workspaceStore.getId()) {
    workspaceStore.setId(id);
    message.error('change workspace success');
  }
}
</script>

<template>
  <div>
    <VbenDropdownRadioMenu
      :menus="menusRef"
      :model-value="id"
      @update:model-value="handleUpdate"
    >
      <VbenIconButton>
        <BiPersonWorkspace class="text-foreground size-4" />
      </VbenIconButton>
    </VbenDropdownRadioMenu>
  </div>
</template>
