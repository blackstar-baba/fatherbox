import ImageNode from './ImageNode';

// 云形状的图片节点
class CloudNode extends ImageNode.view {
  override getImageHref() {
    return 'https://dpubstatic.udache.com/static/dpubimg/0oqFX1nvbD/cloud.png';
  }
}

export default {
  model: ImageNode.model,
  type: 'image-cloud',
  view: CloudNode,
};
