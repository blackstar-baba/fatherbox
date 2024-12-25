import type { RouteRecordRaw } from 'vue-router';

import { BasicLayout } from '#/layouts';
import { $t } from '#/locales';

const routes: RouteRecordRaw[] = [
  {
    component: BasicLayout,
    meta: {
      icon: 'line-md:cog-loop',
      order: 30,
      title: $t('page.system.title'),
    },
    name: 'System',
    path: '/system',
    children: [
      {
        name: 'File Manage',
        path: '/system/file-manage',
        component: () => import('#/views/system/file-manage/index.vue'),
        meta: {
          icon: 'tabler:files',
          title: $t('page.system.file-manage'),
        },
      },
    ],
  },
];

export default routes;
