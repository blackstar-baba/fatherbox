import type { RouteRecordRaw } from 'vue-router';

import { BasicLayout } from '#/layouts';
import { $t } from '#/locales';

const routes: RouteRecordRaw[] = [
  {
    component: BasicLayout,
    meta: {
      icon: 'ion:hammer-outline',
      order: 1,
      title: $t('page.tools.title'),
    },
    name: 'Tools',
    path: '/',
    children: [
      {
        name: 'TimeParser',
        path: '/time-parser',
        component: () => import('#/views/tools/time-parser/index.vue'),
        meta: {
          affixTab: true,
          icon: 'lucide:timer',
          title: $t('page.tools.time-parser'),
        },
      },
      {
        name: 'JsonFormatter',
        path: 'json-formatter',
        component: () => import('#/views/tools/json-formatter/index.vue'),
        meta: {
          // affix: true,
          icon: 'lucide:file-json',
          title: $t('page.tools.json-formatter'),
        },
      },
      {
        name: 'UuidGenerator',
        path: 'uuid-generator',
        component: () => import('#/views/tools/uuid-generator/index.vue'),
        meta: {
          // affix: true,
          icon: 'lucide:key-round',
          title: $t('page.tools.uuid-generator'),
        },
      },
      {
        name: 'Encryptor',
        path: 'encryptor',
        component: () => import('#/views/tools/encryptor/index.vue'),
        meta: {
          // affix: true,
          icon: 'mdi:encryption-outline',
          title: $t('page.tools.encryptor'),
        },
      },
    ],
  },
];

export default routes;
