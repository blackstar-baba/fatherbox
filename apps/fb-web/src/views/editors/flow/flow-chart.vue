<script setup lang="ts">
import { onMounted, ref } from 'vue';

import { LogicFlow } from '@logicflow/core';
import { Control } from '@logicflow/extension';

import '@logicflow/core/lib/style/index.css';
import '@logicflow/extension/lib/style/index.css';

// import RoughRect from './nodes/RoughRect';
// import RoughPolygon from './nodes/RoughPolygon';
// import RoughStar from './nodes/RoughStar';
// import RoughCloud from './nodes/RoughCloud';
// import RoughQuestion from './nodes/RoughQuestion';
// import RoughCube from './nodes/RoughCube';
// import RoughPolyline from './edges/RoughPolyline';
// import RoughUser from './nodes/RoughUser';
// import RoughDatabase from './nodes/RoughDatabase';]

const customTheme: Partial<LogicFlow.Theme> = {
  arrow: {
    offset: 4, // 箭头长度
    verticalLength: 2, // 箭头垂直于边的距离
  },
  baseNode: {
    stroke: '#4E93F5',
  },
  edgeText: {
    fontSize: 13,
    lineHeight: 1.5,
    overflowMode: 'ellipsis',
    textWidth: 100,
  }, // 确认 textWidth 是否必传
  nodeText: {
    fontSize: 13,
    lineHeight: 1.5,
    overflowMode: 'ellipsis',
  },
  polyline: {
    stroke: 'red',
  },
  rect: {
    height: 40,
    width: 200,
  },
};
const data = {
  nodes: [
    {
      id: 'custom-node-1',
      rotate: 1.172_273_881_128_476_3,
      text: {
        value: 'xxxxx',
        x: 600,
        y: 200,
      },
      type: 'rect',
      x: 600,
      y: 200,
    },
  ],
};

// const data = {
//   nodes: [
//     {
//       id: "fc73b1bb-e92b-4fa2-b77c-b6edcaa73286",
//       type: "rough-cloud",
//       x: 513,
//       y: 409,
//       properties: {},
//       text: { x: 513, y: 409, value: "Look in cache" },
//     },
//     {
//       id: "2",
//       type: "rough-rect",
//       x: 660,
//       y: 315,
//       properties: {},
//       text: { x: 660, y: 260, value: "Main" },
//     },
//     {
//       id: "a20314c7-58af-45aa-982f-f63286431dd7",
//       type: "rough-question",
//       x: 576,
//       y: 159,
//       properties: {},
//       text: { x: 576, y: 159, value: "Cache Miss" },
//     },
//     {
//       id: "900fa4d4-9921-456b-9040-900ecfae05cb",
//       type: "rough-cube",
//       x: 398,
//       y: 341,
//       properties: {},
//     },
//     {
//       id: "9de6d7fc-eb3f-4108-85b8-bc4f4ac70d1b",
//       type: "rough-database",
//       x: 847,
//       y: 332,
//       properties: {},
//       text: { x: 847, y: 252, value: "MySql" },
//     },
//     {
//       id: "93fdf590-9e7c-46a8-85c8-823f540f21fc",
//       type: "rough-user",
//       x: 145,
//       y: 316,
//       properties: {},
//       text: { x: 145, y: 256, value: "Client" },
//     },
//     {
//       id: "f4c860f1-499b-4e4b-b1d1-f4fecadb251e",
//       type: "text",
//       x: 408,
//       y: 65,
//       properties: {},
//       text: { x: 408, y: 65, value: "Redis是如何使用的？" },
//     },
//   ],
//   edges: [
//     {
//       id: "873e9ebb-18aa-4691-86f8-20888cc7a7a9",
//       type: "rough-polyline",
//       sourceNodeId: "93fdf590-9e7c-46a8-85c8-823f540f21fc",
//       targetNodeId: "900fa4d4-9921-456b-9040-900ecfae05cb",
//       startPoint: { x: 195, y: 316 },
//       endPoint: { x: 320.5, y: 316 },
//       properties: {},
//       pointsList: [
//         { x: 195, y: 316 },
//         { x: 320.5, y: 316 },
//       ],
//     },
//     {
//       id: "e9d8d73a-f494-4208-9550-be67552ddd46",
//       type: "rough-polyline",
//       sourceNodeId: "900fa4d4-9921-456b-9040-900ecfae05cb",
//       targetNodeId: "2",
//       startPoint: { x: 448, y: 316 },
//       endPoint: { x: 600, y: 315 },
//       properties: {},
//       pointsList: [
//         { x: 448, y: 316 },
//         { x: 524, y: 316 },
//         { x: 524, y: 315 },
//         { x: 600, y: 315 },
//       ],
//     },
//     {
//       id: "19a6517a-da6f-4492-bd88-f9b7da8c15ff",
//       type: "rough-polyline",
//       sourceNodeId: "900fa4d4-9921-456b-9040-900ecfae05cb",
//       targetNodeId: "2",
//       startPoint: { x: 384.25, y: 391 },
//       endPoint: { x: 660, y: 355 },
//       properties: {},
//       pointsList: [
//         { x: 384.25, y: 391 },
//         { x: 384.25, y: 446 },
//         { x: 660, y: 446 },
//         { x: 660, y: 355 },
//       ],
//     },
//     {
//       id: "57a1d989-c2f4-40fb-855c-444528f871a2",
//       type: "rough-polyline",
//       sourceNodeId: "900fa4d4-9921-456b-9040-900ecfae05cb",
//       targetNodeId: "9de6d7fc-eb3f-4108-85b8-bc4f4ac70d1b",
//       startPoint: { x: 384.25, y: 241 },
//       endPoint: { x: 846, y: 232 },
//       properties: {},
//       pointsList: [
//         { x: 384.25, y: 241 },
//         { x: 384.25, y: 199 },
//         { x: 846, y: 199 },
//         { x: 846, y: 232 },
//       ],
//     },
//   ],
// };

const lfRef = ref<LogicFlow | null>(null);
const containerRef = ref<HTMLDivElement | null>(null);

onMounted(() => {
  if (containerRef.value) {
    lfRef.value = new LogicFlow({
      container: containerRef.value,
      grid: true,
      height: 500,
      plugins: [Control],
    });
    lfRef.value.setTheme(customTheme);
    lfRef.value.render(data);
  }
  // const graph = new Graph({
  //   container: document.getElementById('container'),
  //   background: {
  //     color: '#F2F7FA',
  //   },
  //   grid: {
  //     visible: true,
  //     type: 'doubleMesh',
  //     args: [
  //       {
  //         color: '#eee', // 主网格线颜色
  //         thickness: 1, // 主网格线宽度
  //       },
  //       {
  //         color: '#ddd', // 次网格线颜色
  //         thickness: 1, // 次网格线宽度
  //         factor: 4, // 主次网格线间隔
  //       },
  //     ],
  //   },
  // })
  // graph.fromJSON(data);
  // graph.centerContent();
});
</script>
<template>
  <div id="graph" ref="containerRef" class="viewport"></div>
</template>

<style scoped>
.flex-wrapper {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
}

.viewport {
  position: relative;
  overflow: hidden;
}

.el-button + .el-button {
  margin-left: 0;
}

*:focus {
  outline: none;
}

.rect {
  width: 50px;
  height: 50px;
  background: #fff;
  border: 2px solid #000;
}

.circle {
  width: 50px;
  height: 50px;
  background: #fff;
  border: 2px solid #000;
  border-radius: 50%;
}

.uml-wrapper {
  box-sizing: border-box;
  width: 100%;
  height: 100%;
  background: rgb(255 242 204);
  border: 1px solid rgb(214 182 86);
  border-radius: 10px;
}

.uml-head {
  font-size: 16px;
  font-weight: bold;
  line-height: 30px;
  text-align: center;
}

.uml-body {
  padding: 5px 10px;
  font-size: 12px;
  border-top: 1px solid rgb(214 182 86);
  border-bottom: 1px solid rgb(214 182 86);
}

.uml-footer {
  padding: 5px 10px;
  font-size: 14px;
}

/* 输入框字体大小和设置的大小保持一致，自动换行输入和展示保持一致 */

.lf-text-input {
  font-size: 12px;
}

.buttons {
  position: absolute;
  z-index: 1;
}

.button-list {
  display: flex;
  align-items: center;
}
</style>
