import RectNode from './RectNode';

// 带圆角的矩形
class RectRadiusModel extends RectNode.model {
  override setAttributes() {
    super.setAttributes();
    this.radius = 20;
  }
}
export default {
  model: RectRadiusModel,
  type: 'rect-radius',
  view: RectNode.view,
};
