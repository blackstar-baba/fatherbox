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

  const { prefixCls } = useDesign('header-workspace');

  const handleChange = (value: SelectValue) => {
    console.log(`selected ${value}`);
  };

  const workspaces = ref<SelectProps['options']>([
    {
      value: 'default',
      label: 'Default',
    },
    {
      value: 'disabled',
      label: 'Disabled',
      disabled: true,
    },
  ]);

  const workspace = ref('default');
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
