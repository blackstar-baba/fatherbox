import CircleNode from './CircleNode';

// 椭圆
class EllipseNewModel extends CircleNode.model {
  getNodeStyle() {
    const style = super.getNodeStyle();
    return { ...style };
  }
  initNodeData(data) {
    super.initNodeData(data);
    this.rx = 60;
    this.ry = 30;
  }
}
export default {
  model: EllipseNewModel,
  type: 'pro-ellipse',
  view: CircleNode.view,
};
