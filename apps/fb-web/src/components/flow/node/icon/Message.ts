import IconNode from './IconNode';

// 左上角ICON为消息的节点
class MessageNode extends IconNode.view {
  override getImageHref() {
    return 'https://dpubstatic.udache.com/static/dpubimg/1TZgBoaq8G/message.png';
  }
}

export default {
  model: IconNode.model,
  type: 'icon-message',
  view: MessageNode,
};
