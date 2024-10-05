import { h } from '@logicflow/core';

import RectNode from '../basic/RectNode';
import {
  getShapeStyleFuction,
  getTextStyleFunction,
} from '../getShapeStyleUtil';

// 减号
class MinusModel extends RectNode.model {
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
    this.height = 20;
  }
}

class MinusView extends RectNode.view {
  getResizeShape() {
    const { height, width, x, y } = this.props.model;
    const style = this.props.model.getNodeStyle();
    const pointList = [
      [x - (1 / 2) * width, y - (1 / 2) * height],
      [x + (1 / 2) * width, y - (1 / 2) * height],
      [x + (1 / 2) * width, y + (1 / 2) * height],
      [x - (1 / 2) * width, y + (1 / 2) * height],
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
  model: MinusModel,
  type: 'minus',
  view: MinusView,
};
