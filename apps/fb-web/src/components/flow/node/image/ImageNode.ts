import { h } from '@logicflow/core';

import RectNode from '../basic/RectNode';

// 图片-基础节点
class ImageModel extends RectNode.model {
  override initNodeData(data: any) {
    super.initNodeData(data);
    this.width = 80;
    this.height = 60;
  }
}

class ImageNode extends RectNode.view {
  getImageHref() {
    return '';
  }
  override getResizeShape() {
    const { height, width, x, y } = this.props.model;
    const href = this.getImageHref();
    const attrs = {
      height,
      href,
      // 根据宽高缩放
      preserveAspectRatio: 'none meet',
      width,
      x: x - (1 / 2) * width,
      y: y - (1 / 2) * height,
    };
    return h('g', {}, [h('image', { ...attrs })]);
  }
}

export default {
  model: ImageModel,
  type: 'image-node',
  view: ImageNode,
};
