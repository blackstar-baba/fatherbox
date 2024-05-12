<template>
  <PageWrapper
    contentBackground
    title="Remote AI Chat"
    content="Some links to help you quickly reach AI Chat."
  >
    <div class="bg-gray-200 p-5">
      <List :grid="{ gutter: 16, column: 3 }" :data-source="data">
        <template #renderItem="{ item }">
          <ListItem>
            <div class="inline-flex bg-white p-2">
              <img :src="item.image" alt="" />
              <div class="ml-4">
                <a :href="item.url" target="_blank">{{ item.title }}</a>
                {{ item.description }}
              </div>
            </div>
          </ListItem>
        </template>
      </List>
    </div>
  </PageWrapper>
</template>
<script lang="ts" setup>
  import { PageWrapper } from '@/components/Page';
  import { List, ListItem } from 'ant-design-vue';
  import doubao from '@/assets/images/doubao.png';
  import kimi from '@/assets/images/kimi.png';

  interface DataItem {
    title: string;
    image: string;
    url: string;
    description: string;
  }
  const data: DataItem[] = [
    {
      title: 'DouBao',
      image: doubao,
      url: 'https://www.doubao.com',
      description: 'Hello, I am DouBao, Your super assistant is online!',
    },
    {
      title: 'Kimi',
      image: kimi,
      url: 'https://kimi.moonshot.cn/',
      description: 'Help you see the bigger world.',
    },
  ];
</script>
