import { defineStore } from 'pinia';
import * as optionFuns from '@/api/dictionary';
import { TreeSelectOption } from './types';

export type dictionaryKeys = keyof typeof optionFuns;

export type dictionaryStore = {
  [key in dictionaryKeys]: TreeSelectOption[];
};

const useDictionaryStore = defineStore('dictionary', {
  state: (): Partial<dictionaryStore> => {
    return Object.keys(optionFuns).reduce(
      (a: { [key in dictionaryKeys]?: any }, b) => {
        a[b as dictionaryKeys] = undefined;
        return a;
      },
      {}
    );
  },
  actions: {
    updateOptions(partial: Partial<dictionaryStore>) {
      // @ts-ignore-next-line
      this.$patch(partial);
    },
    async initOption(type: dictionaryKeys, useCache = false, query?: any) {
      try {
        const func = (optionFuns as typeof optionFuns)[type];
        if (func) {
          // 如果有query参数，则 useCache 不生效
          useCache = useCache && !query;
          // 如果使用缓存,且存储中有值,则不用获取
          if (useCache && this[type]) {
            return;
          }
          const {
            data: { items },
          } = await func(query);
          this.updateOptions({ [type]: items });
        }
      } catch (err) {
        console.error(err);
      }
    },
    async translateOption(type: dictionaryKeys, value: string) {
      try {
        let options = this[type];
        if (!options) {
          await this.initOption(type);
        }
        options = this[type] || [];
        const option = options.find((o: TreeSelectOption) => o.value === value);
        return Promise.resolve(option ? option.label : '未知');
      } catch (err) {
        console.error(err);
        return Promise.resolve('未知');
      }
    },
  },
});

export default useDictionaryStore;
