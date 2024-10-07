<script lang="ts">
import LogicFlow from '@logicflow/core';
import { SelectionSelect } from '@logicflow/extension';
import { useResizeObserver } from '@vueuse/core';
import { ConfigProvider, theme } from 'ant-design-vue';

import Snapshot from '#/components/flow/snapshot';

import DiagramSidebar from './DiagramSidebar.vue';
import DiagramToolbar from './DiagramToolbar.vue';
import { registerCustomElement } from './node';
import PropertyPanel from './PropertyPanel.vue';

import '@logicflow/core/lib/style/index.css';
import '@logicflow/extension/lib/style/index.css';

export default {
  components: {
    ConfigProvider,
    DiagramSidebar,
    DiagramToolbar,
    PropertyPanel,
  },
  computed: {
    computePanelLeft() {
      // need divide menu width
      return this.width - 150;
    },
    computePanelTop() {
      return this.top - 80;
    },
    computeToolbarLeft() {
      // need divide menu width
      return 250;
    },
    computeToolbarTop() {
      return this.top + this.height - 150;
    },
    theme() {
      return theme;
    },
  },
  // eslint-disable-next-line vue/order-in-components
  data() {
    const lf: any = null;
    const snapshot: any = null;
    return {
      activeEdges: [],
      activeNodes: [],
      filename: '',
      height: 0,
      left: 0,
      lf,
      properties: {},
      sidebarWidth: 200,
      snapshot,
      top: 0,
      width: 0,
    };
  },
  methods: {
    $_changeNodeFill(color: any) {
      const { nodes } = this.lf.graphModel.getSelectElements();
      nodes.forEach(({ id }: { id: string }) => {
        this.lf.setProperties(id, {
          fill: color,
        });
      });
    },
    $_dragInNode(type: any) {
      this.lf.dnd.startDrag({
        type,
      });
    },
    $_getProperty() {
      let properties = {};
      const { edges, nodes } = this.lf.getSelectElements();
      nodes.forEach((node: any) => {
        properties = { ...properties, ...node.properties };
      });
      edges.forEach((edge: any) => {
        properties = { ...properties, ...edge.properties };
      });
      this.properties = properties;
      return properties;
    },
    $_saveGraph() {
      const data = this.lf.getGraphData();
      this.download(this.filename, JSON.stringify(data));
    },
    $_setStyle(item: any) {
      this.activeNodes.forEach(({ id }) => {
        this.lf.setProperties(id, item);
      });
      this.activeEdges.forEach(({ id }) => {
        this.lf.setProperties(id, item);
      });
      this.$_getProperty();
    },
    $_setZIndex(type: any) {
      this.activeNodes.forEach(({ id, zindex }) => {
        if (type === 'up') {
          this.lf.setElementZIndex(id, zindex + 1);
        } else if (type === 'down') {
          this.lf.setElementZIndex(id, zindex - 1);
        } else {
          this.lf.setElementZIndex(id, type);
        }
      });
      this.activeEdges.forEach(({ id }) => {
        this.lf.setElementZIndex(id, type);
      });
    },
    download(filename: string, text: string) {
      window.sessionStorage.setItem(filename, text);
      const element = document.createElement('a');
      element.setAttribute(
        'href',
        `data:text/plain;charset=utf-8,${encodeURIComponent(text)}`,
      );
      element.setAttribute('download', filename);
      element.style.display = 'none';
      document.body.append(element);
      element.click();
      element.remove();
    },
    getContent() {
      return JSON.stringify(this.lf?.getGraphData());
    },
    getImg() {
      return this.snapshot?.getSnapshotBlob();
    },
    initLocation() {
      const rect = (this.$refs.diagram as any).getBoundingClientRect();
      this.top = rect.top;
      this.left = rect.left;
      this.width = rect.width;
      this.height = rect.height;
    },
    initLogicFlow(data: any) {
      // use selection plugin
      LogicFlow.use(SelectionSelect);
      const lf = new LogicFlow({
        autoWrap: true,
        // background: {
        //   backgroundImage:
        //     'url("data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNDAiIGhlaWdodD0iNDAiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PGRlZnM+PHBhdHRlcm4gaWQ9ImdyaWQiIHdpZHRoPSI0MCIgaGVpZ2h0PSI0MCIgcGF0dGVyblVuaXRzPSJ1c2VyU3BhY2VPblVzZSI+PHBhdGggZD0iTSAwIDEwIEwgNDAgMTAgTSAxMCAwIEwgMTAgNDAgTSAwIDIwIEwgNDAgMjAgTSAyMCAwIEwgMjAgNDAgTSAwIDMwIEwgNDAgMzAgTSAzMCAwIEwgMzAgNDAiIGZpbGw9Im5vbmUiIHN0cm9rZT0iI2QwZDBkMCIgb3BhY2l0eT0iMC4yIiBzdHJva2Utd2lkdGg9IjEiLz48cGF0aCBkPSJNIDQwIDAgTCAwIDAgMCA0MCIgZmlsbD0ibm9uZSIgc3Ryb2tlPSIjZDBkMGQwIiBzdHJva2Utd2lkdGg9IjEiLz48L3BhdHRlcm4+PC9kZWZzPjxyZWN0IHdpZHRoPSIxMDAlIiBoZWlnaHQ9IjEwMCUiIGZpbGw9InVybCgjZ3JpZCkiLz48L3N2Zz4=")',
        //   backgroundRepeat: 'repeat',
        // },
        container: this.$refs.diagram as HTMLElement,
        grid: {
          size: 5,
          visible: false,
        },
        keyboard: {
          enabled: true,
        },
        metaKeyMultipleSelected: true,
        overlapMode: 1,
      });
      lf.setTheme({
        baseEdge: { strokeWidth: 1 },
        baseNode: { strokeWidth: 1 },
        // edgeText: { lineHeight: 1.5, overflowMode: 'autoWrap' },
        // nodeText: { lineHeight: 1.5, overflowMode: 'autoWrap' },
      });
      // register nodes
      registerCustomElement(lf);
      // init plugin
      this.snapshot = new Snapshot({ lf });
      lf.setDefaultEdgeType('pro-polyline');
      lf.render(data);
      this.lf = lf;
      this.lf.on('selection:selected,node:click,blank:click,edge:click', () => {
        this.$nextTick(() => {
          const { edges, nodes } = this.lf.getSelectElements();
          // this.$set(this, 'activeNodes', nodes);
          this.activeNodes = nodes;
          this.activeEdges = edges;
          this.$_getProperty();
        });
      });
    },
    setContent(data: string) {
      this.lf?.render(JSON.parse(data));
    },
  },
  // eslint-disable-next-line vue/order-in-components
  mounted() {
    useResizeObserver(this.$refs.diagram as any, (_) => {
      this.initLocation();
    });
    this.initLocation();
    const rect = (this.$refs.diagram as any).getBoundingClientRect();
    this.top = rect.top;
    this.left = rect.left;
    this.width = rect.width;
    this.height = rect.height;
    let data = '';
    const query: { [key: string]: string } = {};
    if (window.location.search) {
      window.location.search
        .slice(1)
        .split('&')
        .forEach((kv) => {
          const [key, value] = kv.split('=');
          if (key && value) {
            query[key] = value;
          }
        });
      if (query.filename) {
        this.filename = query.filename;
      }
      const d = window.sessionStorage.getItem(this.filename);
      if (d) {
        data = JSON.parse(d);
      }
    }
    this.initLogicFlow(data);
  },
  // eslint-disable-next-line vue/order-in-components
  name: 'Diagram',
};
</script>

<template>
  <ConfigProvider
    :theme="{
      algorithm: theme.defaultAlgorithm,
      token: {
        colorBgContainer: '#ffffff',
        colorBgElevated: '#ffffff',
        colorBorder: '#d9d9d9',
        borderRadius: 0,
        colorText: '#000000',
      },
    }"
  >
    <div class="diagram">
      <DiagramToolbar
        v-if="lf"
        :active-edges="activeEdges"
        :lf="lf"
        :style="{
          top: `${computeToolbarTop}px`,
          left: `${computeToolbarLeft}px`,
        }"
        class="diagram-toolbar"
        @change-node-fill-color="$_changeNodeFill"
        @save-graph="$_saveGraph"
      />
      <div class="diagram-main">
        <DiagramSidebar class="diagram-sidebar" @drag-in-node="$_dragInNode" />
        <div class="diagram-container">
          <div class="diagram-wrapper">
            <div ref="diagram" class="lf-diagram"></div>
          </div>
        </div>
      </div>
      <!-- right property panel -->
      <PropertyPanel
        v-if="activeNodes.length > 0 || activeEdges.length > 0"
        :elements-style="properties"
        :only-edge="activeNodes.length === 0"
        :style="{
          top: `${computePanelTop}px`,
          right: `20px`,
        }"
        class="diagram-panel"
        @set-style="$_setStyle"
        @set-z-index="$_setZIndex"
      />
    </div>
  </ConfigProvider>
</template>

<style scoped>
.diagram {
  width: 100%;
  height: 100%;
  color: #000;
}

.diagram * {
  box-sizing: border-box;
}

.diagram-toolbar {
  position: absolute;
  z-index: 9999;
  display: flex;
  align-items: center;
  width: 310px;
  height: 40px;
  background: #fff;
}

.diagram-main {
  display: flex;
  width: 100%;
  height: 100%;
}

.diagram-sidebar {
  width: 185px;
  height: calc(100% - 40px);
  padding: 10px;
  border-right: 1px solid #dadce0;
}

.diagram-panel {
  position: absolute;
  right: 0;
  width: 300px;
  height: auto;
  background: #fff;
  border: 1px solid #dadce0;
}

.diagram-container {
  flex: 1;
}

.diagram-wrapper {
  box-sizing: border-box;
  width: 100%;
  height: 100%;
}

.lf-diagram {
  width: 100%;
  height: 100%;
  box-shadow: 0 0 4px #838284;
}

::-webkit-scrollbar {
  width: 9px;
  height: 9px;
  background: white;
  border-left: 1px solid #e8e8e8;
}

::-webkit-scrollbar-thumb {
  background: #c9c9c9;
  border-color: #fff;
  border-style: solid;
  border-width: 1px;
  border-radius: 6px;
}

::-webkit-scrollbar-thumb:hover {
  background: #b5b5b5;
}
</style>
