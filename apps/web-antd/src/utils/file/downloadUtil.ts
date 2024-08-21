import { openWindow } from '..';
import { dataURLtoBlob, urlToBase64 } from './base64util';

/**
 * Download online pictures
 * @param url
 * @param filename
 * @param mime
 * @param bom
 */
export function downloadByOnlineUrl(
  url: string,
  filename: string,
  mime?: string,
  bom?: BlobPart,
) {
  urlToBase64(url).then((base64) => {
    downloadByBase64(base64, filename, mime, bom);
  });
}

/**
 * Download pictures based on base64
 * @param buf
 * @param filename
 * @param mime
 * @param bom
 */
export function downloadByBase64(
  buf: string,
  filename: string,
  mime?: string,
  bom?: BlobPart,
) {
  const base64Buf = dataURLtoBlob(buf);
  if (!base64Buf) {
    return;
  }
  downloadByData(base64Buf, filename, mime, bom);
}

/**
 * Download according to the background interface file stream
 * @param {*} data
 * @param {*} filename
 * @param {*} mime
 * @param {*} bom
 */
export function downloadByData(
  data: BlobPart,
  filename: string,
  mime?: string,
  bom?: BlobPart,
) {
  const blobData = bom === undefined ? [data] : [bom, data];
  const blob = new Blob(blobData, { type: mime || 'application/octet-stream' });

  const blobURL = window.URL.createObjectURL(blob);
  const tempLink = document.createElement('a');
  tempLink.style.display = 'none';
  tempLink.href = blobURL;
  tempLink.setAttribute('download', filename);
  if (tempLink.download === undefined) {
    tempLink.setAttribute('target', '_blank');
  }
  document.body.append(tempLink);
  tempLink.click();
  tempLink.remove();
  window.URL.revokeObjectURL(blobURL);
}

/**
 * Download file according to file address
 * @param {*} sUrl
 */
/* eslint-disable regexp/no-unused-capturing-group */
/* eslint-disable unicorn/prefer-string-slice */
export function downloadByUrl({
  fileName,
  target = '_blank',
  url,
}: {
  fileName?: string;
  target?: '_blank' | '_self';
  url: string;
}): boolean {
  const isChrome = window.navigator.userAgent.toLowerCase().includes('chrome');
  const isSafari = window.navigator.userAgent.toLowerCase().includes('safari');

  if (/(iP)/.test(window.navigator.userAgent)) {
    console.error('Your browser does not support download!');
    return false;
  }
  if (isChrome || isSafari) {
    const link = document.createElement('a');
    link.href = url;
    link.target = target;

    if (link.download !== undefined) {
      link.download =
        fileName || url.substring(url.lastIndexOf('/') + 1, url.length);
    }

    if (document.createEvent) {
      const e = document.createEvent('MouseEvents');
      e.initEvent('click', true, true);
      link.dispatchEvent(e);
      return true;
    }
  }
  if (!url.includes('?')) {
    url += '?download';
  }

  openWindow(url, { target });
  return true;
}
