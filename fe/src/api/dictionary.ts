/* eslint-disable @typescript-eslint/no-unused-vars */
/* eslint-disable import/prefer-default-export */
/**
 * 一些下拉框
 */
import axios from 'axios';

import type {
  SelectOption,
  TreeSelectOption,
} from '@/store/modules/dictionary/types';

/**
 * FIXME: 这是一个示例
 */
export function AIKinds(params: any) {
  return axios.get<{ items: SelectOption[] }>('/api/ai_kind/option', {
    params,
  });
}
