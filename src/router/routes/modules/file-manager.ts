import type { AppRouteModule } from '@/router/types';

import { LAYOUT } from '@/router/constant';
import { t } from '@/hooks/web/useI18n';

const fileManager: AppRouteModule = {
  path: 'file-system',
  name: 'File',
  redirect: '/file/file-manager/cloud',
  component: LAYOUT,
  meta: {
    // icon: 'carbon:table-split',
    orderNo: 50,
    icon: 'tabler:files',
    title: t('routes.file-manager.file-manager'),
  },
  children: [
    {
      path: 'json',
      component: () => import('@/views/file-manager/cloud/index.vue'),
      name: 'File',
      meta: {
        title: t('routes.file-manager.cloud'),
      },
    },
  ],
};

export default fileManager;
