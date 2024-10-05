import { RectResize } from '@logicflow/extension';

import {
  getShapeStyleFuction,
  getTextStyleFunction,
} from '../getShapeStyleUtil';

// 矩形
class RectNewModel extends RectResize.model {
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

  setToBottom() {
    this.zIndex = 0;
  }
}

export default {
  model: RectNewModel,
  type: 'pro-rect',
  view: RectResize.view,
};
