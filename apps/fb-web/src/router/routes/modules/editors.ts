import type { RouteRecordRaw } from 'vue-router';

import { BasicLayout } from '#/layouts';
import { $t } from '#/locales';

const routes: RouteRecordRaw[] = [
  {
    component: BasicLayout,
    meta: {
      icon: 'gala:editor',
      order: 30,
      title: $t('page.editors.title'),
    },
    name: 'Editors',
    path: '/',
    children: [
      {
        name: 'Markdown',
        path: '/markdown',
        component: () => import('#/views/editors/markdown/index.vue'),
        meta: {
          affixTab: true,
          icon: 'material-symbols:markdown-outline',
          title: $t('page.editors.markdown'),
        },
      },
    ],
  },
];

export default routes;
