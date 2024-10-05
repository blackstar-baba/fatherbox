import { DiamondResize } from '@logicflow/extension';

import {
  getShapeStyleFuction,
  getTextStyleFunction,
} from '../getShapeStyleUtil';

// 菱形
/**
 * model控制初始化的值
 */
class DiamondModel extends DiamondResize.model {
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
    this.rx = 35;
    this.ry = 35;
  }

  setToBottom() {
    this.zIndex = 0;
  }
}

export default {
  model: DiamondModel,
  type: 'pro-diamond',
  view: DiamondResize.view,
};
