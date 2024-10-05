import { PolylineEdge, PolylineEdgeModel } from '@logicflow/core';

import {
  getShapeStyleFuction,
  getTextStyleFunction,
} from '../getShapeStyleUtil';

// 折线
class Model extends PolylineEdgeModel {
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
  type: 'pro-polyline',
  view: PolylineEdge,
};
