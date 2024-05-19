import type { AppRouteModule } from '@/router/types';

import { LAYOUT } from '@/router/constant';
import { t } from '@/hooks/web/useI18n';

const editor: AppRouteModule = {
  path: 'editor',
  name: 'Editor',
  redirect: '/editor/markdown',
  component: LAYOUT,
  meta: {
    // icon: 'carbon:table-split',
    title: t('routes.editor.editor'),
  },
  children: [
    {
      path: 'json',
      component: () => import('@/views/editor/json/index.vue'),
      name: 'JsonEditor',
      meta: {
        title: t('routes.editor.jsonEditor'),
      },
    },
    {
      path: 'markdown',
      name: 'MarkDownEditor',
      component: () => import('@/views/editor/markdown/index.vue'),
      meta: {
        title: t('routes.editor.markdownEditor'),
      },
    },
    {
      path: 'rich-text',
      name: 'RichTextEditor',
      component: () => import('@/views/editor/rich-text/index.vue'),
      meta: {
        title: t('routes.editor.richTextEditor'),
      },
    },
  ],
};

export default editor;
