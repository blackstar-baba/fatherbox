<template>
  <PageWrapper
    contentBackground
    title="Image Editor"
    content="Use tui-image-editor, the Image Editor is a Full-Featured one that is perfect for everyday use. Respect!!!"
  >
    <div class="h-150 w-12/12">
      <ImageEditor />
    </div>
  </PageWrapper>
</template>
<script lang="ts" setup>
  import { PageWrapper } from '@/components/Page';
  import { ImageEditor } from '@/components/ImageEditor';
</script>
