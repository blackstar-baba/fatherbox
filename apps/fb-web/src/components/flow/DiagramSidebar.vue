<script lang="ts">
import IconActor from './icon/actor.vue';
import IconCircle from './icon/circle.vue';
import IconCross from './icon/cross.vue';
import IconCylinde from './icon/cylinde.vue';
import IconDiamond from './icon/diamond.vue';
import IconDivide from './icon/divide.vue';
import IconDownArrow from './icon/down-arrow.vue';
import IconEllipse from './icon/ellipse.vue';
import IconHeptagon from './icon/heptagon.vue';
import IconHexagon from './icon/hexagon.vue';
import IconHorizontalArrow from './icon/horizontal-arrow.vue';
import IconLeftArrow from './icon/left-arrow.vue';
import IconMinus from './icon/minus.vue';
import IconParallelogram from './icon/parallelogram.vue';
import IconPentagon from './icon/pentagon.vue';
import IconRect from './icon/rect.vue';
import IconRectRadius from './icon/rect-radius.vue';
import IconRightArrow from './icon/right-arrow.vue';
import IconSeptagon from './icon/septagon.vue';
import IconText from './icon/text.vue';
import IconTimes from './icon/times.vue';
import IconTrapezoid from './icon/trapezoid.vue';
import IconTriangle from './icon/triangle.vue';
import IconUpArrow from './icon/up-arrow.vue';
import IconVerticalArrow from './icon/vertical-arrow.vue';

export default {
  components: {
    IconActor,
    IconCircle,
    IconCross,
    IconCylinde,
    IconDiamond,
    IconDivide,
    IconDownArrow,
    IconEllipse,
    IconHeptagon,
    IconHexagon,
    IconHorizontalArrow,
    IconLeftArrow,
    IconMinus,
    IconParallelogram,
    IconPentagon,
    IconRect,
    IconRectRadius,
    IconRightArrow,
    IconSeptagon,
    IconText,
    IconTimes,
    IconTrapezoid,
    IconTriangle,
    IconUpArrow,
    IconVerticalArrow,
  },
  emits: ['dragInNode'],
  methods: {
    dragInNode(type: any) {
      this.$emit('dragInNode', type);
    },
  },
  // eslint-disable-next-line vue/order-in-components
  name: 'DiagramSidebar',
};
</script>

<template>
  <div class="diagram-sidebar">
    <div>
      <h1 class="node-category-title">General</h1>
      <div class="node-category">
        <div class="node-item" @mousedown="dragInNode('pro-circle')">
          <IconCircle class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('pro-rect')">
          <IconRect class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('rect-radius')">
          <IconRectRadius class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('triangle')">
          <IconTriangle class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('pro-ellipse')">
          <IconEllipse class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('pro-diamond')">
          <IconDiamond class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('cylinde')">
          <IconCylinde class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('actor')">
          <IconActor class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('parallelogram')">
          <IconParallelogram class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('pro-text')">
          <IconText class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('left-arrow')">
          <IconLeftArrow class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('right-arrow')">
          <IconRightArrow class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('horizontal-arrow')">
          <IconHorizontalArrow class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('up-arrow')">
          <IconUpArrow class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('down-arrow')">
          <IconDownArrow class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('vertical-arrow')">
          <IconVerticalArrow class="svg-node" />
        </div>
        <!-- <div class="node-item" @mousedown="dragInNode('star')">
          star
        </div> -->
        <div class="node-item" @mousedown="dragInNode('pentagon')">
          <IconPentagon class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('hexagon')">
          <IconHexagon class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('septagon')">
          <IconSeptagon class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('heptagon')">
          <IconHeptagon class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('trapezoid')">
          <IconTrapezoid class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('cross')">
          <IconCross class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('minus')">
          <IconMinus class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('times')">
          <IconTimes class="svg-node" />
        </div>
        <div class="node-item" @mousedown="dragInNode('divide')">
          <IconDivide class="svg-node" />
        </div>
      </div>
    </div>
    <div>
      <h1 class="node-category-title">Image</h1>
      <div
        class="image-node image-setting"
        @mousedown="dragInNode('image-setting')"
      ></div>
      <div
        class="image-node image-user"
        @mousedown="dragInNode('image-user')"
      ></div>
      <div
        class="image-node image-cloud"
        @mousedown="dragInNode('image-cloud')"
      ></div>
    </div>
    <div>
      <h1 class="node-category-title">Icon</h1>
      <div
        class="icon-node icon-message"
        @mousedown="dragInNode('icon-message')"
      ></div>
    </div>
  </div>
</template>

<style scoped>
.diagram-sidebar {
  user-select: none;
}

.node-category-title {
  display: block;
  margin: 0 0 10px;
  font-size: 14px;
  line-height: 30px;
  border-bottom: 1px solid #e5e5e5;
}

.node-item {
  display: inline-block;
  width: 35px;
  height: 35px;
  margin-right: 5px;
}

.node-category {
  border-bottom: 1px solid #e5e5e5;
}

.svg-node {
  position: relative;
  top: 1px;
  left: 1px;
  display: block;
  width: 32px;
  height: 30px;
  overflow: hidden;
}

.image-node,
.icon-node {
  display: inline-block;
  width: 30px;
  height: 30px;
  margin: 10px;
  cursor: pointer;
}

.image-setting {
  background: url('https://dpubstatic.udache.com/static/dpubimg/UzI4AFUcfO/setting.png');
  background-size: cover;
}

.image-user {
  width: 40px;
  background: url('https://dpubstatic.udache.com/static/dpubimg/-6Fd2uIoJ-/user.png');
  background-size: cover;
}

.image-cloud {
  width: 40px;
  background: url('https://dpubstatic.udache.com/static/dpubimg/0oqFX1nvbD/cloud.png');
  background-size: cover;
}

.icon-message {
  height: 20px;
  background: url('https://dpubstatic.udache.com/static/dpubimg/1TZgBoaq8G/message.png');
  background-size: cover;
}
</style>
