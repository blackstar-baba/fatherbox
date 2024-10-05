import ImageNode from './ImageNode';

// 图片-设置节点
class SettingModel extends ImageNode.model {
  override initNodeData(data: any) {
    super.initNodeData(data);
    this.width = 60;
    this.height = 60;
  }
}
class SettingNode extends ImageNode.view {
  override getImageHref() {
    return 'https://dpubstatic.udache.com/static/dpubimg/UzI4AFUcfO/setting.png';
  }
}

export default {
  model: SettingModel,
  type: 'image-setting',
  view: SettingNode,
};
