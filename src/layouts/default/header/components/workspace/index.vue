<template>
  <div :class="prefixCls">
    <Popover title="" trigger="click" :overlayClassName="`${prefixCls}__overlay`">
      <Icon icon="bi:person-workspace" />
      <template #content>
        <Select
          v-model:value="workspace"
          :options="workspaces"
          @change="handleChange"
          style="width: 120px"
        />
      </template>
    </Popover>
  </div>
</template>
<script lang="ts" setup>
  import { ref } from 'vue';
  import { Popover, Select } from 'ant-design-vue';
  import type { SelectProps } from 'ant-design-vue';
  import { SelectValue } from 'ant-design-vue/es/select';
  import { useDesign } from '@/hooks/web/useDesign';
  import Icon from '@/components/Icon/Icon.vue';
  import { useWorkspaceStore } from '@/store/modules/workspace';
  import { invoke } from '@tauri-apps/api/tauri';

  const { prefixCls } = useDesign('header-workspace');
  const workspaceStore = useWorkspaceStore();

  const workspaces = ref<SelectProps['options']>([]);
  const workspace = ref('');
  let data: any = [];

  if (window.__TAURI__) {
    invoke('list_workspaces_cmd', {
      request: {},
    }).then((message: any) => {
      data = [];
      message.result.forEach((element: any) => {
        data.push({
          value: element.id,
          label: element.name,
        });
      });
      workspaces.value = data;
      if (data.length > 0) {
        workspace.value = data[0].value;
        workspaceStore.setWorkspaceInfo({
          id: data[0].value,
          name: data[0].label,
        });
      } else {
        workspaceStore.setWorkspaceInfo({
          id: undefined,
          name: undefined,
        });
      }
    });
  }

  const handleChange = (value: SelectValue) => {
    console.log(`selected workspace id: ${value}`);
    data.forEach((element: any) => {
      let elementValue = value?.toString();
      if (elementValue === element.value) {
        workspaceStore.setWorkspaceInfo({
          id: element.value,
          name: element.label,
        });
      }
    });
  };
</script>
<style lang="less">
  @prefix-cls: ~'@{namespace}-header-workspace';

  .@{prefix-cls} {
    padding-bottom: 1px;

    &__overlay {
      max-width: 360px;
    }

    svg {
      width: 0.9em;
    }
  }
</style>
