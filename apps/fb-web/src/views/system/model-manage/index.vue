<script lang="ts" setup>
import { computed, type StyleValue } from 'vue';

import { Page } from '@vben/common-ui';
import { CSS_VARIABLE_LAYOUT_CONTENT_HEIGHT } from '@vben-core/shared/constants';

import { Card } from 'ant-design-vue';

import ModelSetting from '#/components/ai/setting/index.vue';

const contentStyle = computed<StyleValue>(() => {
  return {
    width: '100%',
    height: `calc(var(${CSS_VARIABLE_LAYOUT_CONTENT_HEIGHT}) - 190px)`,
    overflowY: 'auto',
  };
});
</script>
<template>
  <Page auto-content-height description="Manage Models" title="Model Manage">
    <Card :bordered="false">
      <div :style="contentStyle" class="my-2">
        <ModelSetting />
      </div>
    </Card>
  </Page>
</template>
