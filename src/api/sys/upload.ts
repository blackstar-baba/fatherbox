import { UploadApiResult } from './model/uploadModel';
import { defHttp } from '@/utils/http/axios';
import { UploadFileParams } from '#/axios';
import { useGlobSetting } from '@/hooks/setting';
import { AxiosProgressEvent } from 'axios';

const { uploadUrl = '' } = useGlobSetting();

/**
 * @description: Upload interface
 */
export function uploadApi(
  params: UploadFileParams,
  onUploadProgress: (progressEvent: AxiosProgressEvent) => void,
) {
  return defHttp.uploadFile<UploadApiResult>(
    {
      url: uploadUrl,
      onUploadProgress,
    },
    params,
  );
}

/**
 * @description: Upload interface
 */
// export function uploadApi(
//   params: UploadFileParams,
//   onUploadProgress: (progressEvent: AxiosProgressEvent) => void,
// ) {
//   return new Promise<any>((resolve) => {
//     const result = {
//       message: '123',
//       code: 456,
//       url: 'xxx.png',
//     };
//     onUploadProgress({
//       loaded: 100,
//       total: 100,
//       bytes: 100,
//     });
//     resolve({ data: result });
//   });
// }
