import { h } from '@logicflow/core';

import RectNode from '../basic/RectNode';
import {
  getShapeStyleFuction,
  getTextStyleFunction,
} from '../getShapeStyleUtil';

// 五边形
class HeptagonModel extends RectNode.model {
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

class HeptagonView extends RectNode.view {
  override getResizeShape() {
    const { height, width, x, y } = this.props.model;
    const style = this.props.model.getNodeStyle();
    const pointList = [
      [x - 0.205 * width, y - 0.5 * height],
      [x + 0.205 * width, y - 0.5 * height],
      [x + 0.5 * width, y - 0.205 * height],
      [x + 0.5 * width, y + 0.205 * height],
      [x + 0.205 * width, y + 0.5 * height],
      [x - 0.205 * width, y + 0.5 * height],
      [x - 0.5 * width, y + 0.205 * height],
      [x - 0.5 * width, y - 0.205 * height],
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
  model: HeptagonModel,
  type: 'heptagon',
  view: HeptagonView,
};
