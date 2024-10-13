import { message } from 'ant-design-vue';

export * from './dateUtil';

export function openWindow(
  url: string,
  opt?: {
    noopener?: boolean;
    noreferrer?: boolean;
    target?: '_blank' | '_self' | string;
  },
) {
  const { noopener = true, noreferrer = true, target = '__blank' } = opt || {};
  const feature: string[] = [];

  noopener && feature.push('noopener=yes');
  noreferrer && feature.push('noreferrer=yes');

  window.open(url, target, feature.join(','));
}

export function copy(value: any) {
  const input = document.createElement('textarea');
  input.style.position = 'fixed';
  input.style.opacity = '0';
  input.value = value;
  document.body.append(input);
  input.select();
  document.execCommand('Copy');
  input.remove();
  message.info('copy success');
}
