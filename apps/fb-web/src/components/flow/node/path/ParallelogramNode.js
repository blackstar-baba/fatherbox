import { h } from '@logicflow/core';
import { RectResize } from '@logicflow/extension';

import {
  getShapeStyleFuction,
  getTextStyleFunction,
} from '../getShapeStyleUtil';

// 平行四边形
class ParallelogramModel extends RectResize.model {
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
    this.width = 100;
    this.height = 60;
  }
}

class ParallelogramView extends RectResize.view {
  getResizeShape() {
    const { height, width, x, y } = this.props.model;
    const style = this.props.model.getNodeStyle();
    const pointList = [
      [x - width / 2, y + height / 2],
      [x - width / 5, y - height / 2],
      [x + width / 2, y - height / 2],
      [x + width / 5, y + height / 2],
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
  model: ParallelogramModel,
  type: 'parallelogram',
  view: ParallelogramView,
};
