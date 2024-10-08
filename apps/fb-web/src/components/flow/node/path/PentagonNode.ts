import { h } from '@logicflow/core';

import RectNode from '../basic/RectNode';
import {
  getShapeStyleFuction,
  getTextStyleFunction,
} from '../getShapeStyleUtil';

// 八边形
class PentagonModel extends RectNode.model {
  override getNodeStyle() {
    const style = super.getNodeStyle();
    const properties = this.getProperties();
    return getShapeStyleFuction(style, properties);
  }
  override getTextStyle() {
    const style = super.getTextStyle();
    const properties = this.getProperties();
    return getTextStyleFunction(style, properties);
  }

  override initNodeData(data: any) {
    super.initNodeData(data);
    this.width = 80;
    this.height = 80;
  }
}

class PentagonView extends RectNode.view {
  override getResizeShape() {
    const { height, width, x, y } = this.props.model;
    const style = this.props.model.getNodeStyle();
    const pointList = [
      [x - 0.5 * width, y],
      [x, y - 0.5 * height],
      [x + 0.5 * width, y],
      [x + 0.3 * width, y + 0.5 * height],
      [x - 0.3 * width, y + 0.5 * height],
    ];
    const points = pointList.map((item) => {
      return `${item[0]},${item[1]}`;
    });
    const attrs = {
      ...style,
      height,
      points: points.join(' '),
      width,
      x,
      y,
    };

    return h('g', {}, [h('polygon', { ...attrs })]);
  }
}

export default {
  model: PentagonModel,
  type: 'pentagon',
  view: PentagonView,
};
