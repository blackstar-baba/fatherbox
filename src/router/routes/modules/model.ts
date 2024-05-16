import type { AppRouteModule } from '@/router/types';

import { LAYOUT } from '@/router/constant';
import { t } from '@/hooks/web/useI18n';

const tool: AppRouteModule = {
  path: '/model',
  name: 'Model',
  component: LAYOUT,
  redirect: '/model/remote',
  meta: {
    orderNo: 10,
    icon: 'eos-icons:ai',
    title: t('routes.model.model'),
  },
  children: [
    {
      path: 'remote',
      name: 'Remote',
      component: () => import('@/views/model/remote/index.vue'),
      meta: {
        // affix: true,
        title: t('routes.model.remote'),
      },
    },
    {
      path: 'ollama',
      name: 'Ollama',
      component: () => import('@/views/model/ollama/index.vue'),
      meta: {
        // affix: true,
        title: t('routes.model.ollama'),
      },
    },
  ],
};

export default tool;
