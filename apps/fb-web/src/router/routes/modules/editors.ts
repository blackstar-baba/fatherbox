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
          icon: 'material-symbols:markdown-outline',
          title: $t('page.editors.markdown'),
        },
      },
      {
        name: 'Flow',
        path: '/editors/flow',
        component: () => import('#/views/editors/flow/index.vue'),
        meta: {
          icon: 'mdi:flowchart-outline',
          title: $t('page.editors.flow'),
        },
      },
      {
        name: 'Text',
        path: '/editors/text',
        component: () => import('#/views/editors/text/index.vue'),
        meta: {
          icon: 'mdi:text-box-outline',
          title: $t('page.editors.text'),
        },
      },
    ],
  },
];

export default routes;
