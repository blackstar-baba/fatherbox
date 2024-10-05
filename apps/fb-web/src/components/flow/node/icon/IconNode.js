import { h } from '@logicflow/core';

import RectNode from '../basic/RectNode';

// 左上角带ICON的节点
class IconNode extends RectNode.view {
  getImageHref() {}
  getResizeShape() {
    const { height, width, x, y } = this.props.model;
    const style = this.props.model.getNodeStyle();
    const href = this.getImageHref();
    const iconAttrs = {
      height: 18,
      href,
      // 根据宽高缩放
      preserveAspectRatio: 'none meet',
      width: 25,
      x: x - (1 / 2) * width + 5,
      y: y - (1 / 2) * height + 5, // icon在左上角
    };
    const rectAttrs = {
      ...style,
      height,
      rx: 5,
      ry: 5,
      strokeWidth: 1,
      width,
      x: x - (1 / 2) * width,
      y: y - (1 / 2) * height,
    };
    return h('g', {}, [
      h('rect', { ...rectAttrs }),
      h('image', { ...iconAttrs }),
    ]);
  }
}

export default {
  model: RectNode.model,
  type: 'image-node',
  view: IconNode,
};
