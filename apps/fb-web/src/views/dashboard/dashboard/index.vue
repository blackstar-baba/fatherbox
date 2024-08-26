<script lang="ts" setup>
import type { WorkbenchQuickNavItem, WorkbenchTodoItem } from '@vben/common-ui';

import { ref } from 'vue';

import {
  WorkbenchHeader,
  WorkbenchQuickNav,
  WorkbenchTodo,
} from '@vben/common-ui';
import { $t } from '@vben/locales';
import { preferences } from '@vben/preferences';
import { useUserStore } from '@vben/stores';

const userStore = useUserStore();

const quickNavItems: WorkbenchQuickNavItem[] = [
  {
    color: '#1fdaca',
    icon: 'lucide:timer',
    title: 'TimeParser',
  },
  {
    color: '#bf0c2c',
    icon: 'lucide:file-json',
    title: 'JsonFormatter',
  },
  {
    color: '#e18525',
    icon: 'lucide:key-round',
    title: 'UuidGenerator',
  },
  {
    color: '#3fb27f',
    icon: 'mdi:encryption-outline',
    title: 'Encryptor',
  },
];

const todoItems = ref<WorkbenchTodoItem[]>([
  {
    completed: false,
    content: `Love Family.`,
    date: '2024-07-30 11:00:00',
    title: 'todo1',
  },
  {
    completed: true,
    content: `Love Myself.`,
    date: '2024-07-30 11:00:00',
    title: 'todo2',
  },
  {
    completed: false,
    content: `Love FatherBox.`,
    date: '2024-07-30 11:00:00',
    title: 'todo3',
  },
]);
</script>

<template>
  <div class="p-5">
    <WorkbenchHeader
      :avatar="userStore.userInfo?.avatar || preferences.app.defaultAvatar"
    >
      <template #title>
        {{ $t('page.dashboard.welcome') }}, {{ userStore.userInfo?.realName }},
        {{ $t('page.dashboard.day') }}
      </template>
      <template #description></template>
    </WorkbenchHeader>

    <div class="mt-5 flex flex-col lg:flex-row">
      <div class="mr-4 w-full lg:w-3/5">
        <WorkbenchQuickNav
          :items="quickNavItems"
          :title="$t('page.dashboard.quicknav')"
        />
      </div>
      <div class="w-full lg:w-2/5">
        <WorkbenchTodo :items="todoItems" :title="$t('page.dashboard.todo')" />
      </div>
    </div>
  </div>
</template>
