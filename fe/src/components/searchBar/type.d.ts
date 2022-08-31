import type { TreeSelectOption } from '@/store/modules/dictionary/types';
import type { dictionaryKeys } from '@/store/modules/dictionary';

interface TextMeta {
  placeholder?: string;
  allowClear?: boolean;
}
interface RemoteSelectMeta {
  placeholder?: string;
  remoteType: dictionaryKeys;
  useCache?: boolean;
  allowClear?: boolean;
}
interface NumberMeta {
  placeholder?: string;
  min?: number;
  max?: number;
  allowClear?: boolean;
}
interface SelectMeta {
  placeholder?: string;
  options: TreeSelectOption[];
  allowClear?: boolean;
}

interface DateRangeMeta {
  format?: string;
  style?: string;
  mode?: 'date' | 'year' | 'quarter' | 'month' | 'week';
}

interface SearchItemBase {
  name: string;
  label: string;
  labelSpan?: number;
  warpperSpan?: number;
  span?: number;
}

interface TextSearchItem extends SearchItemBase {
  type: 'text';
  meta: TextMeta;
}
interface RemoteSelectSearchItem extends SearchItemBase {
  type: 'remoteSelect';
  meta: RemoteSelectMeta;
}
interface NumberSearchItem extends SearchItemBase {
  type: 'number';
  meta: NumberMeta;
}
interface SelectSearchItem extends SearchItemBase {
  type: 'select';
  meta: SelectMeta;
}

/**
 * date range 类型,会自动将 name 转为  name__gt 和 name__lt 两个参数
 * 例如: created_at 会被转为 created_at__gt 和 created_at__lt 两个参数
 */
interface DateRangeSearchItem extends SearchItemBase {
  type: 'dateRange';
  meta: DateRangeMeta;
}

export type SearchItem =
  | TextSearchItem
  | RemoteSelectSearchItem
  | NumberSearchItem
  | SelectSearchItem
  | DateRangeSearchItem;
