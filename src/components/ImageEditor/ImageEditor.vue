<template>
  <div class="m-4">
    <Button type="success" @click="selectFile">
      <template #icon>
        <ExpandOutlined />
      </template>
      Load
    </Button>
    <input type="file" style="display: none" ref="fileInput" @change="handleFileChange" />
    <Button type="warning" class="ml-2" @click="download">
      <template #icon>
        <CompressOutlined />
      </template>
      Download
    </Button>
  </div>
  <div id="tui-image-editor"></div>
</template>

<script type="ts">
  import 'tui-image-editor/dist/tui-image-editor.css'
  import 'tui-color-picker/dist/tui-color-picker.css'
  import ImageEditor from 'tui-image-editor'
  import demoImg from '@/assets/images/demo.png';
  import { CompressOutlined, ExpandOutlined } from "@ant-design/icons-vue";
  import { Button } from "ant-design-vue";
  import { downloadByBase64 } from "@/utils/file/download";
  import { save } from '@tauri-apps/api/dialog';
  import { writeTextFile } from '@tauri-apps/api/fs';

  const customTheme = {
    // image 坐上角度图片
    "common.bi.image": "", // 在这里换上你喜欢的logo图片
    "common.bisize.width": "0px",
    "common.bisize.height": "0px",
    "common.backgroundImage": "none",
    "common.backgroundColor": "#555555",
    "common.border": "1px solid #ffffff ",

    // header
    "header.backgroundImage": "none",
    "header.backgroundColor": "#555555",
    "header.border": "0px",

    // load button
    "loadButton.backgroundColor": "#fff",
    "loadButton.border": "1px solid #ddd",
    "loadButton.color": "#222",
    "loadButton.fontFamily": "NotoSans, sans-serif",
    "loadButton.fontSize": "12px",
    "loadButton.display": "none", // 可以直接隐藏掉

    // download button
    "downloadButton.backgroundColor": "#fdba3b",
    "downloadButton.border": "1px solid #fdba3b",
    "downloadButton.color": "#fff",
    "downloadButton.fontFamily": "NotoSans, sans-serif",
    "downloadButton.fontSize": "12px",
    "downloadButton.display": "none", // 可以直接隐藏掉

    // icons default
    "menu.normalIcon.color": "#8a8a8a",
    "menu.activeIcon.color": "#555555",
    "menu.disabledIcon.color": "#434343",
    "menu.hoverIcon.color": "#e9e9e9",
    "submenu.normalIcon.color": "#8a8a8a",
    "submenu.activeIcon.color": "#e9e9e9",

    "menu.iconSize.width": "24px",
    "menu.iconSize.height": "24px",
    "submenu.iconSize.width": "32px",
    "submenu.iconSize.height": "32px",

    // submenu primary color
    "submenu.backgroundColor": "#555555",
    "submenu.partition.color": "#858585",

    // submenu labels
    "submenu.normalLabel.color": "#858585",
    "submenu.normalLabel.fontWeight": "lighter",
    "submenu.activeLabel.color": "#fff",
    "submenu.activeLabel.fontWeight": "lighter",

    // checkbox style
    "checkbox.border": "1px solid #ccc",
    "checkbox.backgroundColor": "#555555",

    // rango style
    "range.pointer.color": "#fff",
    "range.bar.color": "#666",
    "range.subbar.color": "#d1d1d1",

    "range.disabledPointer.color": "#414141",
    "range.disabledBar.color": "#282828",
    "range.disabledSubbar.color": "#414141",

    "range.value.color": "#fff",
    "range.value.fontWeight": "lighter",
    "range.value.fontSize": "11px",
    "range.value.border": "1px solid #353535",
    "range.value.backgroundColor": "#555555",
    "range.title.color": "#fff",
    "range.title.fontWeight": "lighter",

    // colorpicker style
    "colorpicker.button.border": "1px solid #1e1e1e",
    "colorpicker.title.color": "#fff"
  };
  export default {
    components: { Button, ExpandOutlined, CompressOutlined },
    data() {
      return {
        instance: null,
      }
    },
    mounted() {
      this.instance = new ImageEditor(
        document.querySelector("#tui-image-editor"),
        {
          includeUI: {
            loadImage: {
              path: demoImg,
              name: "demo.png",
            },
            initMenu: "draw",
            menuBarPosition: "right",
            theme: customTheme,
          },
        }
      );
    },
    methods: {
      selectFile() {
        this.$refs.fileInput.click();
      },
      async download() {
        let imageName = this.instance.getImageName();
        let dataURL = this.instance.toDataURL();
        console.info(imageName + ": " + dataURL);
         if (window.__TAURI__) {
           const filePath = await save({ defaultPath: imageName });
           await writeTextFile(filePath, dataURL);
         } else {
           downloadByBase64(dataURL, imageName);
         }
      },
      handleFileChange(event) {
        let file = event.target.files[0];
        this.instance.loadImageFromFile(file).then(function (result) {
          console.log(result);
          this.instance.clearUndoStack();
        });
      }
    },
  }
</script>
