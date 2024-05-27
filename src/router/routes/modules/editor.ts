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
    orderNo: 40,
    icon: 'icon-park-outline:editor',
    title: t('routes.editor.editor'),
  },
  children: [
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
