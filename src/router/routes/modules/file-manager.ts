import type { AppRouteModule } from '@/router/types';

import { LAYOUT } from '@/router/constant';
import { t } from '@/hooks/web/useI18n';

const fileManager: AppRouteModule = {
  path: '/file-manager',
  name: 'File Manager',
  redirect: '/file-manager/index',
  component: LAYOUT,
  meta: {
    hideChildrenInMenu: true,
    orderNo: 50,
    icon: 'tabler:files',
    title: t('routes.file-manager.file-manager'),
  },
  children: [
    {
      path: 'index',
      name: 'File Manager Page',
      component: () => import('@/views/file-manager/index.vue'),
      meta: {
        title: t('routes.file-manager.file-manager'),
        hideMenu: true,
      },
    },
  ],
};

export default fileManager;
