<script lang="ts">
import { Sketch } from '@ckpack/vue-color';
import {
  Button,
  ConfigProvider,
  InputNumber,
  Popover,
  RadioButton,
  RadioGroup,
  Select,
  SelectOption,
} from 'ant-design-vue';

import {
  borderStyles,
  fontFamilies,
  layers,
  shortStyles,
  textAligns,
} from './constant';
import BorderStyleDashed from './icon/border-style-dashed.vue';
import BorderStyleDotted from './icon/border-style-dotted.vue';
import BorderStyleSolid from './icon/border-style-solid.vue';
import FontStyleBold from './icon/font-style-bold.vue';
import FontStyleItalic from './icon/font-style-italic.vue';
import FontStyleUnderline from './icon/font-style-underline.vue';
import LayerBottom from './icon/layer-bottom.vue';
import LayerDown from './icon/layer-down.vue';
import LayerTop from './icon/layer-top.vue';
import LayerUp from './icon/layer-up.vue';
import TextAlignCenter from './icon/text-align-center.vue';
import TextAlignLeft from './icon/text-align-left.vue';
import TextAlignRight from './icon/text-align-right.vue';

interface Rgba {
  r: number;
  g: number;
  b: number;
  a: number;
}

export default {
  components: {
    BorderStyleDashed,
    BorderStyleDotted,
    BorderStyleSolid,
    Button,
    ConfigProvider,
    FontStyleBold,
    FontStyleItalic,
    FontStyleUnderline,
    InputNumber,
    LayerBottom,
    LayerDown,
    LayerTop,
    LayerUp,
    Popover,
    RadioButton,
    RadioGroup,
    Select,
    SelectOption,
    SketchPicker: Sketch,
    TextAlignCenter,
    TextAlignLeft,
    TextAlignRight,
  },
  data() {
    return {
      borderStyles,
      borderWidthOptions: Array.from({ length: 5 }, (_, i) => i),
      fontFamilies,
      fontWeight: '', // 文本加粗
      layers,
      lineHeightOptions: Array.from<number>({ length: 5 })
        .fill(1)
        .map((value, i) => value + i * 0.5),
      shortStyles,
      SketchPicker: Sketch,
      style: {
        backgroundColor: '', // 填充色
        borderColor: '#000000', // 填充颜色
        borderStyle: 'solid', // 线条类型
        borderType: 0, // 线条类型
        borderWidth: 1, // 线条宽度
        fontColor: '#000000', // 文本颜色
        fontFamily: '', // 文本样式
        fontSize: 12, // 文本大小
        fontStyle: '', // 文字风格
        fontWeight: '', // 文本加粗
        gradientColor: '', // 渐变色
        lineHeight: 1, // 行高
        textAlign: 'center', // 对齐
        textDecoration: '', // 下划线
      },
      textAligns,
    };
  },
  // TODO 基于vue3以及ts重构
  // eslint-disable-next-line vue/order-in-components
  emits: ['setStyle', 'setZIndex'],
  methods: {
    $_changeBorderWidth(val: any) {
      this.$emit('setStyle', {
        borderWidth: val,
      });
    },

    $_changeColorProperty(
      { rgba: { a, b, g, r } }: { rgba: Rgba },
      type: string,
    ) {
      const color = `rgba(${r},${g},${b},${a})`;
      this.$emit('setStyle', {
        [type]: color,
      });
    },
    $_changeFontFamily(val: any) {
      this.$emit('setStyle', {
        fontFamily: val,
      });
    },
    $_changeFontSize(val: any) {
      this.$emit('setStyle', {
        fontSize: val,
      });
    },
    $_changeFontStyle() {
      if (this.style.fontStyle === 'italic') {
        this.$emit('setStyle', {
          fontStyle: 'normal',
        });
      } else {
        this.$emit('setStyle', {
          fontStyle: 'italic',
        });
      }
    },
    $_changeFontWeight() {
      if (this.style.fontWeight === 'bold') {
        this.$emit('setStyle', {
          fontWeight: 'normal',
        });
      } else {
        this.$emit('setStyle', {
          fontWeight: 'bold',
        });
      }
    },
    $_changeLineHeight(val: any) {
      this.$emit('setStyle', {
        lineHeight: val,
      });
    },
    $_changeTextAlign(val: any) {
      this.$emit('setStyle', {
        textAlign: val,
      });
    },
    $_changeTextDecoration() {
      if (this.style.textDecoration === 'underline') {
        this.$emit('setStyle', {
          textDecoration: 'none',
        });
      } else {
        this.$emit('setStyle', {
          textDecoration: 'underline',
        });
      }
    },
    $_selectBorder(val: any) {
      this.$emit('setStyle', {
        borderStyle: val,
      });
    },
    capitalize(str: string) {
      return str.charAt(0).toUpperCase() + str.slice(1);
    },
    setStyle(item: any) {
      this.$emit('setStyle', item);
    },
  },
  // eslint-disable-next-line vue/order-in-components
  props: {
    // eslint-disable-next-line vue/require-default-prop
    elementsStyle: Object,
    onlyEdge: Boolean, // 是否是只设置边的属性，当只设置边的属性时，隐藏快捷样式和背景色设置
  },
  // eslint-disable-next-line vue/order-in-components
  watch: {
    elementsStyle: {
      handler(val) {
        this.style = { ...this.style, ...val };
      },
      immediate: true,
    },
  },
};
</script>

<template>
  <div class="diagram-panel">
    <div class="setting-block">
      <div class="setting-item">
        <span>Shortcut</span>
        <div class="short-styles">
          <div
            v-for="(item, index) in shortStyles"
            :key="index"
            :style="{
              backgroundColor: item.backgroundColor,
              borderColor: item.borderColor,
              borderWidth: item.borderWidth,
            }"
            @click="setStyle(item)"
          ></div>
        </div>
      </div>
    </div>
    <div class="setting-block">
      <div class="setting-item">
        <span>Background</span>
        <Popover class="w-200" trigger="click">
          <template #content>
            <SketchPicker
              v-model="style.backgroundColor"
              @update:model-value="
                (c: any) => $_changeColorProperty(c, 'backgroundColor')
              "
            />
          </template>
          <div
            :style="{ backgroundColor: style.backgroundColor }"
            class="border-color"
          ></div>
        </Popover>
      </div>
      <div class="setting-item">
        <span>Gradient</span>
        <Popover trigger="click" width="220">
          <template #content>
            <SketchPicker
              v-model="style.gradientColor"
              @update:model-value="
                (c: any) => $_changeColorProperty(c, 'gradientColor')
              "
            />
          </template>
          <div
            :style="{ backgroundColor: style.gradientColor }"
            class="border-color"
          ></div>
        </Popover>
      </div>
      <div class="setting-item">
        <span>Border Style</span>
        <div
          v-for="(border, index) in borderStyles"
          :key="index"
          :class="
            style.borderStyle === border.value
              ? 'item-style selected'
              : 'item-style'
          "
          @click="$_selectBorder(border.value)"
        >
          <component
            :is="`BorderStyle${capitalize(border.value)}`"
            class="svg-node"
          />
        </div>
        <!--        <Select-->
        <!--          v-model:value="style.borderStyle"-->
        <!--          class="min-w-40"-->
        <!--          @change="$_selectBorder"-->
        <!--        >-->
        <!--          <SelectOption label="" value="hidden" />-->
        <!--          <SelectOption-->
        <!--            v-for="(border, index) in borderStyles"-->
        <!--            :key="index"-->
        <!--            :value="border.value"-->
        <!--          >-->
        <!--            <div-->
        <!--              :style="`border-bottom-style: ${border.value};`"-->
        <!--              class="border-style"-->
        <!--            ></div>-->
        <!--          </SelectOption>-->
        <!--        </Select>-->
      </div>
      <div class="setting-item">
        <span>Border Color</span>
        <Popover trigger="click" width="220">
          <template #content>
            <SketchPicker
              v-model="style.borderColor"
              @update:model-value="
                (c: any) => $_changeColorProperty(c, 'borderColor')
              "
            />
          </template>
          <div
            :style="{ backgroundColor: style.borderColor }"
            class="border-color"
          ></div>
        </Popover>
      </div>
      <div class="setting-item">
        <span>Border Width</span>
        <div
          v-for="item in borderWidthOptions"
          :key="item"
          class="border-width"
          @click="$_changeBorderWidth(item)"
        >
          {{ item }}
        </div>
      </div>
      <div class="setting-item">
        <span>Font Color</span>
        <Popover trigger="click" width="220">
          <template #content>
            <SketchPicker
              v-model="style.fontColor"
              @update:model-value="
                (c: any) => $_changeColorProperty(c, 'fontColor')
              "
            />
          </template>
          <div
            :style="{ backgroundColor: style.fontColor }"
            class="border-color"
          ></div>
        </Popover>
      </div>
      <div class="setting-item">
        <span>Font Size</span>
        <InputNumber
          v-model:value="style.fontSize"
          :max="30"
          :min="12"
          controls-position="right"
          size="small"
          @change="$_changeFontSize"
        />
        <span style="width: 20px">px</span>
      </div>
      <div class="setting-item">
        <span>Font Family</span>
        <Select
          v-model:value="style.fontFamily"
          class="min-w-40 text-base"
          size="small"
          @change="$_changeFontFamily"
        >
          <SelectOption
            v-for="(fontFamily, index) in fontFamilies"
            :key="index"
            :value="fontFamily.value"
          />
        </Select>
      </div>
      <div class="setting-item">
        <span>Font Line Height</span>
        <div
          v-for="(item, index) in lineHeightOptions"
          :key="index"
          class="border-width"
          @click="$_changeLineHeight(item)"
        >
          {{ item }}
        </div>
      </div>
      <div class="setting-item">
        <span>Text Align</span>
        <div
          v-for="(align, index) in textAligns"
          :key="index"
          :class="
            style.textAlign === align.value
              ? 'item-style selected'
              : 'item-style'
          "
          @click="$_changeTextAlign(align.value)"
        >
          <component
            :is="`TextAlign${capitalize(align.value)}`"
            class="svg-node"
          />
        </div>
      </div>
      <div class="setting-item">
        <span>Font Style</span>
        <div
          :class="
            style.fontWeight === 'bold' ? 'item-style selected' : 'item-style'
          "
          @click="$_changeFontWeight()"
        >
          <FontStyleBold />
        </div>
        <div
          :class="
            style.textDecoration === 'underline'
              ? 'item-style selected'
              : 'item-style'
          "
          @click="$_changeTextDecoration()"
        >
          <FontStyleUnderline />
        </div>
        <div
          :class="
            style.fontStyle === 'italic' ? 'item-style selected' : 'item-style'
          "
          @click="$_changeFontStyle()"
        >
          <FontStyleItalic />
        </div>
      </div>
      <div class="setting-item">
        <span>Layer</span>
        <div
          v-for="(layer, index) in layers"
          :key="index"
          class="item-style"
          @click="$emit('setZIndex', `${layer.value}`)"
        >
          <component :is="`Layer${capitalize(layer.value)}`" class="svg-node" />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.diagram-panel {
  padding: 20px;
}

.short-styles {
  width: 240px;
}

.short-styles > div {
  box-sizing: border-box;
  float: left;
  width: 20px;
  height: 20px;
  margin: 0 10px 5px 0;
  cursor: pointer;
  border: 1px solid #fff;
}

.item-style {
  box-sizing: border-box;
  float: left;
  width: 25px;
  height: 25px;
  margin: 0 10px 5px 0;
  cursor: pointer;
  border: 1px solid #eaeaeb;
  border-radius: 5px;
}

.selected {
  background-color: #dae8fc;
}

.border-width {
  box-sizing: border-box;
  float: left;
  width: 25px;
  height: 25px;
  padding: 3px;
  margin: 0 10px 5px 0;
  text-align: center;
  cursor: pointer;
  border: 1px solid #eaeaeb;
}

.setting-block {
  overflow: hidden;
}

.setting-item {
  display: flex;
  align-items: center;
  margin-top: 10px;
  font-size: 12px;
  line-height: 12px;
  vertical-align: middle;
}

.setting-item > span {
  flex-grow: 0;
  flex-shrink: 0;
  width: 80px;
  margin-right: 10px;
  text-align: right;
}

.border-color {
  display: inline-block;
  width: 80px;
  height: 30px;
  border: 1px solid #eaeaeb;
}

.selected-button {
  color: #fff;
  background-color: #1890ff;
  border-color: #1890ff;
}

.setting-item > select {
  color: #000;
}
</style>
