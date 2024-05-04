import type { AppRouteModule } from '@/router/types';

import { LAYOUT } from '@/router/constant';
import { t } from '@/hooks/web/useI18n';

const tool: AppRouteModule = {
  path: '/tool',
  name: 'Tool',
  component: LAYOUT,
  redirect: '/tool/time-parser',
  meta: {
    orderNo: 10,
    icon: 'ion:hammer-outline',
    title: t('routes.tool.tool'),
  },
  children: [
    {
      path: 'time-parser',
      name: 'TimeParser',
      component: () => import('@/views/tool/time-parser/index.vue'),
      meta: {
        // affix: true,
        title: t('routes.tool.time-parser'),
      },
    },
    {
      path: 'json-formatter',
      name: 'JsonFormatter',
      component: () => import('@/views/tool/json-formatter/index.vue'),
      meta: {
        // affix: true,
        title: t('routes.tool.json-formatter'),
      },
    },
    {
      path: 'uuid-generator',
      name: 'UuidGenerator',
      component: () => import('@/views/tool/uuid-generator/index.vue'),
      meta: {
        // affix: true,
        title: t('routes.tool.uuid-generator'),
      },
    },
    {
      path: 'encryptor',
      name: 'Encryptor',
      component: () => import('@/views/tool/encryptor/index.vue'),
      meta: {
        // affix: true,
        title: t('routes.tool.encryptor'),
      },
    },
  ],
};

export default tool;
