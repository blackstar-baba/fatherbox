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
    path: '/editors',
    children: [
      {
        name: 'Markdown',
        path: '/editors/markdown',
        component: () => import('#/views/editors/markdown/index.vue'),
        meta: {
          affixTab: true,
          icon: 'material-symbols:markdown-outline',
          title: $t('page.editors.markdown'),
        },
      },
      {
        name: 'Flow',
        path: '/editors/flow',
        component: () => import('#/views/editors/flow/index.vue'),
        meta: {
          affixTab: true,
          icon: 'mdi:flowchart-outline',
          title: $t('page.editors.flow'),
        },
      },
    ],
  },
];

export default routes;
