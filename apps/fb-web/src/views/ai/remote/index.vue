<script lang="ts" setup>
import { Page } from '@vben/common-ui';

import { Avatar, List, ListItem, ListItemMeta } from 'ant-design-vue';

interface DataItem {
  title: string;
  image: string;
  url: string;
  description: string;
}
// cursor
const data: DataItem[] = [
  {
    description:
      'An AI tool with rich functions, intelligence, and strong usability, suitable for various content creators',
    image: '/llm/openai-avatar.png',
    title: 'ChatGPT',
    url: 'https://chat.openai.com/',
  },
  {
    description: 'It is an AI painting and chatting tool on Discord',
    image: '/llm/midjourney.png',
    title: 'Midjourney',
    url: 'https://www.midjourney.com/home/',
  },
  {
    description: 'Open source datasets and pre-trained models',
    image: '/llm/huggingface.png',
    title: 'Hugging Face',
    url: 'https://huggingface.co/',
  },
  {
    description:
      'The AI assistant in China that is closest to chatGPT, and its popularity is still rising, with related stocks hitting the daily limit',
    image: '/llm/kimi-logo.png',
    title: 'Kimi',
    url: 'https://kimi.moonshot.cn/?utm_campaign=TR_jmM0VDCh&utm_content=&utm_medium=CPA&utm_source=CH_VmpNgGW9&utm_term=',
  },
  {
    description: 'An AI-driven creative tool',
    image: '/llm/runway.png',
    title: 'Runway',
    url: 'https://runwayml.com/',
  },
  {
    description:
      'It is a chatbot web application based on a neural language model',
    image: '/llm/character-ai.jpg',
    title: 'Character AI',
    url: 'https://beta.character.ai/',
  },
  {
    description: 'Make unlimited creations with Firefly',
    image: '/llm/adobe-firefly.png',
    title: 'Adobe Firefly',
    url: 'https://firefly.adobe.com/',
  },
  {
    description:
      'An AI-generated UI platform that enables easy and fast design conception',
    image: '/llm/galileo-ai.png',
    title: 'Galileo AI',
    url: 'https://www.usegalileo.ai/',
  },
  {
    description:
      'Automatically generate code, help you debug bugs, improve development efficiency, and save time.',
    image: '/llm/fittencode.png',
    title: 'Fitten Code',
    url: 'https://code.fittentech.com/?ref=cnjc',
  },
  {
    description: 'A tool for processing images with AI technology',
    image: '/llm/clipdrop.jpg',
    title: 'Clipdrop',
    url: 'https://clipdrop.co/tools',
  },
  {
    description: 'Remove the background and create product images',
    image: '/llm/photoroom.jpg',
    title: 'PhotoRoom',
    url: 'https://www.photoroom.com/',
  },
  {
    description: 'The best AI writer for creation',
    image: '/llm/writersonic.jpg',
    title: 'Writesonic',
    url: 'https://writesonic.com/',
  },
  {
    description:
      'A small assistant for marketing soft articles and self-media manuscripts',
    image: '/llm/copy.png',
    title: 'Copy.AI',
    url: 'https://www.copy.ai/',
  },
  {
    description: 'Create amazing avatars',
    image: '/llm/getimg.png',
    title: 'getimg.ai',
    url: 'https://getimg.ai/',
  },
  {
    description:
      'Can help users generate logos, book covers, banners, etc. in a few minutes',
    image: '/llm/stockimg.jpg',
    title: 'stockimg',
    url: 'https://stockimg.ai/',
  },
  {
    description: 'Allow creators to use neural networks without code',
    image: '/llm/p.jpg',
    title: 'Phygital+',
    url: 'https://phygital.plus/',
  },
  {
    description:
      'Provide music source separation services, using AI technology to extract vocals, accompaniment, and various instruments from any audio',
    image: '/llm/lalal.png',
    title: 'LALAL',
    url: 'https://www.lalal.ai/',
  },
  {
    description:
      'A new royalty-free music ecosystem for content creators, brands, and developers',
    image: '/llm/mubert.jpg',
    title: 'Mubert',
    url: 'https://mubert.com/',
  },
  {
    description:
      'The simplest AI-assisted keyframe animation production software',
    image: '/llm/cascadeur.jpg',
    title: 'Cascadeur',
    url: 'https://cascadeur.com/',
  },
  {
    description:
      'This platform provides an intuitive interface that can simplify video creation for anyone',
    image: '/llm/synthesia.jpg',
    title: 'Synthesia',
    url: 'https://www.synthesia.io',
  },
  {
    description: 'Turn text into video with AI voice',
    image: '/llm/fliki.jpg',
    title: 'Fliki',
    url: 'https://fliki.ai/',
  },
  {
    description: 'An AI painting platform',
    image: '/llm/autodraw.jpg',
    title: 'Autodraw',
    url: 'https://www.autodraw.com/',
  },
  {
    description: 'A magical tool for coloring photos',
    image: '/llm/palette.png',
    title: 'Palette.fm',
    url: 'https://palette.fm/',
  },
  {
    description: 'AI painting, generate pictures by entering keywords',
    image: '/llm/stable-diffusion.png',
    title: 'Stable Diffusion',
    url: 'https://stability.ai/',
  },
  {
    description: 'A website for sharing trained models',
    image: '/llm//civitai.png',
    title: 'Civitai',
    url: 'https://civitai.com/',
  },
  {
    description: 'AI for generating videos from pictures',
    image: '/llm/genmo.jpg',
    title: 'Genmo',
    url: 'https://www.genmo.ai/',
  },
  {
    description: 'AI for generating pictures from text',
    image: '/llm/vega.jpg',
    title: 'Vega AI',
    url: 'https://www.vegaai.net/',
  },
  {
    description: 'A text generator that uses AI to create amazing copy',
    image: '/llm/jaster.png',
    title: 'Jasper',
    url: 'https://www.jasper.ai/',
  },
  {
    description:
      "A programming assistant under Doubao, used by 70% of ByteDance's R & D personnel",
    image: '/llm/db-ai.jpg',
    title: '豆包MarsCode',
    url: 'https://www.marscode.cn/?utm_source=jyshare&utm_medium=banner1',
  },
  {
    description:
      "Huawei's Pangu large model, reshaping thousands of industries",
    image: '/llm/huawei-pangu.png',
    title: '盘古',
    url: 'https://pangu.huaweicloud.com/',
  },
  {
    description: 'An artificial intelligence language model produced by Baidu',
    image: '/llm/wxyy.png',
    title: '文心一言',
    url: 'https://yiyan.baidu.com/',
  },
  {
    description: 'A large language model developed by Alibaba',
    image: '/llm/ty.png',
    title: '通义',
    url: 'https://tongyi.aliyun.com/',
  },
  {
    description: 'A double-billion-level large language model',
    image: '/llm/tiangong.png',
    title: '天工',
    url: 'https://www.tiangong.cn/',
  },
  {
    description:
      'A code auto-generation tool developed by GitHub and OpenAI in collaboration',
    image: '/llm/copilot-x.png',
    title: 'Copilot X',
    url: 'https://github.com/features/preview/copilot-x',
  },
  {
    description: 'It is an AI code assistant',
    image: '/llm/tabnine.png',
    title: 'Tabnine',
    url: 'https://www.tabnine.com/',
  },
  {
    description: 'Artificial Intelligence Coding and Retrieval',
    image: '/llm/codeium.png',
    title: 'Codeium',
    url: 'https://codeium.com/',
  },
  {
    description: 'Real-time Analysis of AI Code',
    image: '/llm/codiga.png',
    title: 'Codiga',
    url: 'https://www.codiga.io/',
  },
  {
    description: 'Magical generation of secondary element paintings',
    image: '/llm/niji-journey.png',
    title: 'Nijijourney',
    url: 'https://nijijourney.com/zh/',
  },
];
</script>
<template>
  <Page
    description="Some links to help you quickly reach AI Apps"
    title="Remote AI Apps"
  >
    <div class="p-5">
      <List
        :data-source="data"
        :grid="{ gutter: 16, xs: 1, sm: 1, md: 2, lg: 3, xl: 4, xxl: 6 }"
      >
        <template #renderItem="{ item }">
          <ListItem>
            <ListItemMeta :description="item.description">
              <template #title>
                <span class="underline">
                  <a :href="item.url">{{ item.title }}</a>
                </span>
              </template>
              <template #avatar>
                <Avatar :src="item.image" class="h-8 w-8" />
              </template>
            </ListItemMeta>
          </ListItem>
        </template>
      </List>
    </div>
  </Page>
</template>
