import { LineEdge, LineEdgeModel } from '@logicflow/core';

import {
  getShapeStyleFuction,
  getTextStyleFunction,
} from '../getShapeStyleUtil';

// 直线
class Model extends LineEdgeModel {
  constructor(data: any, graphModel: any) {
    super(data, graphModel);
    this.strokeWidth = 1;
  }
  override getEdgeStyle() {
    const attributes = super.getEdgeStyle();
    const properties = this.properties;
    const style = getShapeStyleFuction(attributes, properties);
    return { ...style, fill: 'none' };
  }

  override getTextStyle() {
    const style = super.getTextStyle();
    return getTextStyleFunction(style, this.properties);
  }
}
export default {
  model: Model,
  type: 'pro-line',
  view: LineEdge,
};
