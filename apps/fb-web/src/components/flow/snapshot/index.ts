import LogicFlow from '@logicflow/core';

// Blob | base64
export type SnapshotResponse = {
  data: Blob;
  height: number;
  width: number;
};

/**
 * 快照插件，生成视图
 */
export class Snapshot {
  static pluginName = 'snapshot';
  customCssRules: string;
  fileName?: string;
  lf: LogicFlow;
  offsetX?: number;
  offsetY?: number;
  useGlobalRules: boolean;

  constructor({ lf }: { lf: LogicFlow }) {
    this.lf = lf;
    this.customCssRules = '';
    this.useGlobalRules = true;
  }

  /**
   * clone svg node
   * @param svg
   * @returns Node
   */
  private cloneSvg(svg: Element, addStyle: boolean = true): Node {
    const copy = svg.cloneNode(true);
    const graph = copy.lastChild;
    let childLength = graph?.childNodes?.length;
    if (childLength) {
      for (let i = 0; i < childLength; i++) {
        const lfLayer = graph?.childNodes[i] as SVGGraphicsElement;
        // 只保留包含节点和边的基础图层进行下载，其他图层删除
        const layerClassList = lfLayer.classList && [...lfLayer.classList];
        if (layerClassList && !layerClassList.includes('lf-base')) {
          lfLayer.remove();
          childLength--;
          i--;
        } else {
          // 删除锚点
          const lfBase = graph?.childNodes[i];
          lfBase &&
            lfBase.childNodes.forEach((item) => {
              const element = item as SVGGraphicsElement;
              // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
              this.removeAnchor(element.firstChild!);
              // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
              this.removeRotateControl(element.firstChild!);
            });
        }
      }
    }
    // set css style
    if (addStyle) {
      const style = document.createElement('style');
      style.innerHTML = this.getClassRules();
      const foreignObject = document.createElement('foreignObject');
      foreignObject.append(style);
      copy.append(foreignObject);
    }
    return copy;
  }

  /**
   * get css class rules
   * @returns rule string
   */
  private getClassRules(): string {
    let rules = '';
    if (this.useGlobalRules) {
      const { styleSheets } = document;
      for (const sheet of styleSheets) {
        // 这里是为了过滤掉不同源 css 脚本，防止报错终止导出
        try {
          if (sheet) {
            for (let j = 0; j < sheet.cssRules.length; j++) {
              rules += sheet.cssRules[j]?.cssText;
            }
          }
        } catch {
          // console.log(
          //   'CSS scripts from different sources have been filtered out',
          // );
        }
      }
    }
    if (this.customCssRules) {
      rules += this.customCssRules;
    }
    return rules;
  }

  /**
   * get svg root dom
   * @param lf
   * @returns Element
   */
  private getSvgRootElement(lf: LogicFlow) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const svgRootElement = lf.container.querySelector('.lf-canvas-overlay')!;
    return svgRootElement;
  }

  /**
   * remove anchor
   * @param element
   */
  private removeAnchor(element: ChildNode) {
    const { childNodes } = element;
    let childLength = element.childNodes.length;
    for (let i = 0; i < childLength; i++) {
      const child = childNodes[i] as SVGGraphicsElement;
      const classList = (child.classList && [...child.classList]) || [];
      if (classList.includes('lf-anchor')) {
        child.remove();
        childLength--;
        i--;
      }
    }
  }
  /**
   * remove rotate control
   * @param element
   */
  private removeRotateControl(element: ChildNode) {
    const { childNodes } = element;
    let childLength = element.childNodes.length;
    for (let i = 0; i < childLength; i++) {
      const child = childNodes[i] as SVGGraphicsElement;
      const classList = (child.classList && [...child.classList]) || [];
      if (classList.includes('lf-rotate-control')) {
        child.remove();
        childLength--;
        i--;
      }
    }
  }

  public getSnapshotBlob(): SnapshotResponse {
    const svg = this.getSvgRootElement(this.lf);
    const copy = this.cloneSvg(svg);
    const svgString = new XMLSerializer().serializeToString(copy);
    const blob = new Blob([svgString], {
      type: 'image/svg+xml;charset=utf-8',
    });
    return {
      data: blob,
      height: 0,
      width: 0,
    };
  }
}

export default Snapshot;
