import { defHttp } from '@/utils/http/axios';
import { ModelItems } from './model/model';

enum Api {
  GetModels = '/ollama/tags',
}

/**
 * @description: Get all models
 */

export const getModels = () => {
  return defHttp.get<ModelItems>({ url: Api.GetModels }, { apiUrl: '' });
};
