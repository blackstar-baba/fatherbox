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
    path: '/tools',
    children: [
      {
        name: 'TimeParser',
        path: '/tools/time-parser',
        component: () => import('#/views/tools/time-parser/index.vue'),
        meta: {
          icon: 'lucide:timer',
          title: $t('page.tools.time-parser'),
        },
      },
      {
        name: 'JsonFormatter',
        path: '/tools/json-formatter',
        component: () => import('#/views/tools/json-formatter/index.vue'),
        meta: {
          icon: 'lucide:file-json',
          title: $t('page.tools.json-formatter'),
        },
      },
      {
        name: 'UuidGenerator',
        path: '/tools/uuid-generator',
        component: () => import('#/views/tools/uuid-generator/index.vue'),
        meta: {
          icon: 'lucide:key-round',
          title: $t('page.tools.uuid-generator'),
        },
      },
      {
        name: 'Encryptor',
        path: '/tools/encryptor',
        component: () => import('#/views/tools/encryptor/index.vue'),
        meta: {
          icon: 'mdi:encryption-outline',
          title: $t('page.tools.encryptor'),
        },
      },
      {
        name: 'RegularExpression',
        path: '/tools/reg-exp',
        component: () => import('#/views/tools/reg-exp/index.vue'),
        meta: {
          icon: 'lucide:regex',
          title: $t('page.tools.reg-exp'),
        },
      },
    ],
  },
];

export default routes;
