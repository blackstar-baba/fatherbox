import { h } from '@logicflow/core';

import RectNode from '../basic/RectNode';
import {
  getShapeStyleFuction,
  getTextStyleFunction,
} from '../getShapeStyleUtil';

// 五边形
class TrapezoidModel extends RectNode.model {
  getNodeStyle() {
    const style = super.getNodeStyle();
    const properties = this.getProperties();
    return getShapeStyleFuction(style, properties);
  }
  getTextStyle() {
    const style = super.getTextStyle();
    const properties = this.getProperties();
    return getTextStyleFunction(style, properties);
  }

  initNodeData(data) {
    super.initNodeData(data);
    this.width = 80;
    this.height = 80;
  }
}

class TrapezoidView extends RectNode.view {
  getResizeShape() {
    const { height, width, x, y } = this.props.model;
    const style = this.props.model.getNodeStyle();
    const pointList = [
      [x - 0.31 * width, y - 0.5 * height],
      [x + 0.31 * width, y - 0.5 * height],
      [x + 0.5 * width, y + 0.5 * height],
      [x - 0.5 * width, y + 0.5 * height],
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
  model: TrapezoidModel,
  type: 'trapezoid',
  view: TrapezoidView,
};
