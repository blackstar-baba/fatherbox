import { BaseNode, BaseNodeModel, h } from '@logicflow/core';

class BaseNewNode extends BaseNode {
  getShape(): h.JSX.Element | null {
    return null;
  }
}

class BaseNewModel extends BaseNodeModel {
  override setAttributes() {
    this.fill = 'red';
  }
}

export default {
  model: BaseNewModel,
  type: 'BaseNode',
  view: BaseNewNode,
};
