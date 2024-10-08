<script lang="ts">
// import { Sketch } from 'vue-color'
// import ColorFill from './icon/ColorFill.vue'
// import ColorText from './icon/ColorText.vue'
import { LogicFlow } from '@logicflow/core';
// import IconFont from './icon/Font.vue'
import { theme } from 'ant-design-vue';

// import IconBlod from './icon/Blod.vue'
import AreaSelect from './icon/area-select.vue';
import StepBack from './icon/step-back.vue';
import StepFoward from './icon/step-foward.vue';
// import IconLine from './icon/line.vue';
import ZoomIn from './icon/zoom-in.vue';
import ZoomOut from './icon/zoom-out.vue';

export default {
  components: {
    // ColorFill,
    // ColorText,
    // IconFont,
    // IconBlod,
    AreaSelect,
    // Select,
    // SelectOption,
    StepBack,
    StepFoward,
    // IconLine,
    ZoomIn,
    ZoomOut,
    // SketchPicker: Sketch
  },
  computed: {
    theme() {
      return theme;
    },
  },
  // eslint-disable-next-line vue/order-in-components
  data() {
    return {
      colors: '#345678',
      lineOptions: [
        {
          label: '折线',
          value: 'pro-polyline',
        },
        {
          label: '直线',
          value: 'pro-line',
        },
        {
          label: '曲线',
          value: 'pro-bezier',
        },
      ],
      linetype: 'pro-polyline',
      redoAble: false,
      selectionOpened: false,
      undoAble: false,
    };
  },
  // eslint-disable-next-line vue/order-in-components
  emits: ['changeNodeFillColor', 'saveGraph'],
  methods: {
    $_changeFillColor(val: any) {
      this.$emit('changeNodeFillColor', val.hex);
    },
    $_changeLineType(value: any) {
      const { activeEdges, lf } = this.$props;
      if (lf) {
        const { graphModel } = lf;
        lf.setDefaultEdgeType(value);
        if (activeEdges && activeEdges.length > 0) {
          activeEdges.forEach((edge: any) => {
            graphModel.changeEdgeType(edge.id, value);
          });
        }
      }
    },
    $_redo() {
      if (this.$data.redoAble && this.$props.lf) {
        this.$props.lf.redo();
      }
    },
    $_saveGraph() {
      this.$emit('saveGraph');
    },
    $_selectionSelect() {
      this.selectionOpened = !this.selectionOpened;
      if (this.lf && this.lf.extension.selectionSelect) {
        // todo
        // if (this.selectionOpened) {
        //   this.lf.extension.selectionSelect.openSelectionSelect();
        // } else {
        //   this.lf.extension.selectionSelect.closeSelectionSelect();
        // }
      }
    },
    $_undo() {
      if (this.$data.undoAble && this.$props.lf) {
        this.$props.lf.undo();
      }
    },
    $_zoomIn() {
      if (this.$props.lf) {
        this.$props.lf.zoom(true);
      }
    },
    $_zoomOut() {
      if (this.$props.lf) {
        this.$props.lf.zoom(false);
      }
    },
  },
  // eslint-disable-next-line vue/order-in-components
  mounted() {
    if (this.$props.lf) {
      this.$props.lf.on(
        'history:change',
        ({ data: { redoAble, undoAble } }) => {
          this.$data.redoAble = redoAble;
          this.$data.undoAble = undoAble;
        },
      );
    }
  },
  // eslint-disable-next-line vue/order-in-components
  props: {
    // eslint-disable-next-line vue/require-default-prop
    activeEdges: Array,
    fillColor: {
      default: '',
      type: String,
    },
    lf: LogicFlow,
  },
};
</script>

<template>
  <div>
    <div
      :class="{ 'selection-active': selectionOpened }"
      class="toolbar-item"
      @click="$_selectionSelect()"
    >
      <AreaSelect size="18" />
    </div>
    <!-- <div class="toolbar-item toolbar-color-picker">
      <el-popover
        placement="top-start"
        title="填充样式"
        width="220"
        trigger="click"
      >
        <sketch-picker :value="fillColor"  @input="$_changeFillColor"/>
        <color-fill size="24" slot="reference" />
      </el-popover>
    </div> -->
    <!-- <div class="toolbar-item">
      <color-text size="20" />
    </div>
    <div class="toolbar-item">
      <icon-font size="18" />
    </div>
    <div class="toolbar-item">
      <icon-blod size="18" />
    </div>
    <div class="toolbar-item">
      <icon-line size="18" />
    </div> -->
    <div class="toolbar-item" @click="$_zoomIn()">
      <ZoomIn size="18" />
    </div>
    <div class="toolbar-item" @click="$_zoomOut()">
      <ZoomOut size="18" />
    </div>
    <div
      :class="{ disabled: !undoAble }"
      class="toolbar-item"
      @click="$_undo()"
    >
      <StepBack size="18" />
    </div>
    <div
      :class="{ disabled: !redoAble }"
      class="toolbar-item"
      @click="$_redo()"
    >
      <StepFoward size="18" />
    </div>
    <!-- <div>
      <button @click="$_saveGraph">保存</button>
    </div> -->
    <!--    <div>-->
    <!--            <Select v-model="linetype" class="w-36" @change="$_changeLineType">-->
    <!--              <SelectOption-->
    <!--                v-for="item in lineOptions"-->
    <!--                :key="item.value"-->
    <!--                :label="item.label"-->
    <!--                :value="item.value"-->
    <!--              />-->
    <!--            </Select>-->
    <!--    </div>-->
  </div>
</template>

<style scoped>
.toolbar-item {
  float: left;
  width: 18px;
  height: 18px;
  margin: 12px 4px;
  cursor: pointer;
}

.toolbar-color-picker {
  width: 24px;
  height: 24px;
  margin: 8px 4px;
}

.selection-active {
  background: #33a3dc;
}
</style>
