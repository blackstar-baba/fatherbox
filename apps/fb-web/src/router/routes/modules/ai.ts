import type { RouteRecordRaw } from 'vue-router';

import { BasicLayout } from '#/layouts';
import { $t } from '#/locales';

const routes: RouteRecordRaw[] = [
  {
    component: BasicLayout,
    meta: {
      icon: 'vscode-icons:file-type-ai',
      order: 30,
      title: $t('page.ai.title'),
    },
    name: 'AI',
    path: '/ai',
    children: [
      {
        name: 'Remote',
        path: '/ai/remote',
        component: () => import('#/views/ai/remote/index.vue'),
        meta: {
          icon: 'carbon:ai-launch',
          title: $t('page.ai.remote'),
        },
      },
      {
        name: 'Local',
        path: '/ai/local',
        component: () => import('#/views/ai/local/index.vue'),
        meta: {
          icon: 'hugeicons:ai-chat-02',
          title: $t('page.ai.local'),
        },
      },
    ],
  },
];

export default routes;
