import { BezierEdge, BezierEdgeModel } from '@logicflow/core';

import {
  getShapeStyleFuction,
  getTextStyleFunction,
} from '../getShapeStyleUtil';

// 贝塞尔曲线
class Model extends BezierEdgeModel {
  constructor(data, graphModel) {
    super(data, graphModel);
    this.strokeWidth = 1;
  }
  getEdgeStyle() {
    const attributes = super.getEdgeStyle();
    const properties = this.properties;
    const style = getShapeStyleFuction(attributes, properties);
    return { ...style, fill: 'none' };
  }

  getTextStyle() {
    const style = super.getTextStyle();
    return getTextStyleFunction(style, this.properties);
  }
}
export default {
  model: Model,
  type: 'pro-bezier',
  view: BezierEdge,
};
