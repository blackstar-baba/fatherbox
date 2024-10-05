// 基础图形
import DownArrow from './arrow/DownArrowNode';
import HorizontalArrow from './arrow/HorizontalArrowNode';
// 多边形绘制的箭头
import LeftArrow from './arrow/LeftArrow';
import RightArrow from './arrow/RightArrow';
import UpArrow from './arrow/UpArrowNode';
import VerticalArrow from './arrow/VerticalArrowNode';
import CircleNode from './basic/CircleNode';
import DiamondNode from './basic/DiamondNode';
import EllipseNode from './basic/EllipseNode';
import RectNode from './basic/RectNode';
import RectRadiusNode from './basic/RectRadiusNode';
import TextNode from './basic/TextNode';
import Bezier from './edge/Bezier';
import Line from './edge/Line';
// 注册边
import Ployline from './edge/Polyline';
// image绘制左上角icon节点
import IconMessage from './icon/Message';
import ImageCloud from './image/Cloud';
// image绘制图片节点
import ImageSetting from './image/Setting';
import ImageUser from './image/User';
import ActorNode from './path/ActorNode';
import CrossNode from './path/CrossNode';
// path绘制的个性化图形
import CylindeNode from './path/CylindeNode';
import DivideNode from './path/DivideNode';
import HeptagonNode from './path/HeptagonNode';
import HexagonNode from './path/HexagonNode';
import MinusNode from './path/MinusNode';
import ParallelogramNode from './path/ParallelogramNode';
import PentagonNode from './path/PentagonNode';
import SeptagonNode from './path/SeptagonNode';
import StarNode from './path/Star';
import TimesNode from './path/TimesNode';
import TrapezoidNode from './path/TrapezoidNode';
import TriangleNode from './path/TriangleNode';

export const registerCustomElement = (lf) => {
  // 注册基础图形
  lf.register(CircleNode);
  lf.register(RectNode);
  lf.register(RectRadiusNode);
  lf.register(EllipseNode);
  lf.register(DiamondNode);
  lf.register(TextNode);
  // 注册path绘制的个性化图形
  lf.register(CylindeNode);
  lf.register(TriangleNode);
  lf.register(ParallelogramNode);
  lf.register(ActorNode);
  lf.register(StarNode);
  lf.register(PentagonNode);
  lf.register(HexagonNode);
  lf.register(SeptagonNode);
  lf.register(HeptagonNode);
  lf.register(TrapezoidNode);
  lf.register(CrossNode);
  lf.register(MinusNode);
  lf.register(TimesNode);
  lf.register(DivideNode);
  // 注册多边形绘制的箭头
  lf.register(LeftArrow);
  lf.register(RightArrow);
  lf.register(HorizontalArrow);
  lf.register(UpArrow);
  lf.register(DownArrow);
  lf.register(VerticalArrow);
  // 注册image绘制图片节点
  lf.register(ImageSetting);
  lf.register(ImageUser);
  lf.register(ImageCloud);
  // 注册image绘制左上角icon节点
  lf.register(IconMessage);
  // 注册边
  lf.register(Ployline);
  lf.register(Line);
  lf.register(Bezier);
};
