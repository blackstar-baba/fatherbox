import type { AppRouteModule } from '@/router/types';

import { LAYOUT } from '@/router/constant';
import { t } from '@/hooks/web/useI18n';

const tool: AppRouteModule = {
  path: '/model',
  name: 'Model',
  component: LAYOUT,
  redirect: '/model/doubao',
  meta: {
    orderNo: 10,
    icon: 'ion:hammer-outline',
    title: t('routes.model.model'),
  },
  children: [
    {
      path: 'doubao',
      name: 'DouBao',
      component: () => import('@/views/model/doubao/index.vue'),
      meta: {
        // affix: true,
        title: t('routes.model.doubao'),
      },
    },
  ],
};

export default tool;
